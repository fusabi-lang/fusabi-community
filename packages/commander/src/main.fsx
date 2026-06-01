// Commander - A TUI File Explorer for Fusabi
// Demonstrates Events, TerminalControl, TerminalInfo, Process, Console, List, and String modules

// ============================================================================
// MODEL
// ============================================================================

let initialModel = {
    currentDir = Process.cwd ();
    files = [];
    selectedIndex = 0;
    running = true
}

// ============================================================================
// HELPERS
// ============================================================================

let parseFiles stdout =
    let lines = String.split "\n" stdout in
    let nonEmpty = List.filter (fun line -> String.length line > 0) lines in
    // Skip the first line (total count from ls -l)
    match List.tail nonEmpty with
    | Some tail -> tail
    | None -> []

let getFileName line =
    let parts = String.split " " line in
    let filtered = List.filter (fun p -> String.length p > 0) parts in
    // Get the last part (filename)
    match List.nth 8 filtered with
    | Some name -> name
    | None -> line

let loadDirectory dir =
    let result = Process.runShell (sprintf "ls -la \"%s\"" dir) in
    if result.exitCode == 0 then
        parseFiles result.stdout
    else
        []

let clamp min max value =
    if value < min then min
    else if value > max then max
    else value

// ============================================================================
// VIEW
// ============================================================================

let clearScreen () =
    TerminalControl.sendText "\x1b[2J\x1b[H"

let renderHeader model =
    let _ = printfn (sprintf "=== Commander TUI ===") in
    let _ = printfn (sprintf "Directory: %s" model.currentDir) in
    printfn (sprintf "")

let renderFile index selected fileName =
    let marker = if index == selected then "> " else "  " in
    printfn (sprintf "%s%s" marker fileName)

let renderFiles model =
    let renderWithIndex = fun (index, fileName) ->
        renderFile index model.selectedIndex fileName
    in
    let indexed = List.mapi (fun i f -> (i, f)) model.files in
    List.iter renderWithIndex indexed

let renderFooter () =
    let _ = printfn (sprintf "") in
    printfn (sprintf "Controls: j/k (navigate) | Enter (select) | q (quit)")

let render model =
    let _ = clearScreen () in
    let _ = renderHeader model in
    let _ = renderFiles model in
    renderFooter ()

// ============================================================================
// UPDATE
// ============================================================================

let handleKeyDown model =
    let maxIndex = List.length model.files - 1 in
    let newIndex = clamp 0 maxIndex (model.selectedIndex + 1) in
    { model with selectedIndex = newIndex }

let handleKeyUp model =
    let maxIndex = List.length model.files - 1 in
    let newIndex = clamp 0 maxIndex (model.selectedIndex - 1) in
    { model with selectedIndex = newIndex }

let handleEnter model =
    match List.nth model.selectedIndex model.files with
    | Some selectedFile ->
        let fileName = getFileName selectedFile in
        let newPath =
            if fileName == "." then
                model.currentDir
            else if fileName == ".." then
                let result = Process.runShell (sprintf "cd \"%s\" && cd .. && pwd" model.currentDir) in
                if result.exitCode == 0 then
                    String.trim result.stdout
                else
                    model.currentDir
            else
                sprintf "%s/%s" model.currentDir fileName
        in
        // Check if it's a directory
        let checkResult = Process.runShell (sprintf "test -d \"%s\" && echo \"dir\"" newPath) in
        if String.contains "dir" checkResult.stdout then
            let files = loadDirectory newPath in
            { model with currentDir = newPath; files = files; selectedIndex = 0 }
        else
            let _ = TerminalControl.showToast (sprintf "Not a directory: %s" fileName) in
            model
    | None -> model

let handleQuit model =
    { model with running = false }

let update event model =
    match event with
    | "key:j" -> handleKeyDown model
    | "key:k" -> handleKeyUp model
    | "key:enter" -> handleEnter model
    | "key:q" -> handleQuit model
    | _ -> model

// ============================================================================
// EVENT LOOP
// ============================================================================

let rec eventLoop model =
    if model.running then
        // Render current state
        let _ = render model in

        // Get real user input
        let _ = Console.write "Enter command (j/k/enter/q): " in
        let input = Console.readLine () in

        // Convert input to event name
        let eventName =
            match input with
            | "j" -> "key:j"
            | "k" -> "key:k"
            | "enter" -> "key:enter"
            | "" -> "key:enter"   // Empty line = Enter
            | "q" -> "key:q"
            | other -> sprintf "key:%s" other
        in
        // Update model and continue loop
        let newModel = update eventName model in
        eventLoop newModel
    else
        let _ = printfn (sprintf "\nExiting Commander...") in
        model

// ============================================================================
// MAIN
// ============================================================================

let main () =
    let _ = printfn (sprintf "Starting Commander TUI...") in
    let _ = printfn (sprintf "") in

    // Get terminal size
    let (cols, rows) = TerminalInfo.getTerminalSize () in
    let _ = printfn (sprintf "Terminal size: %d cols x %d rows" cols rows) in

    // Get current directory
    let cwd = Process.cwd () in
    let _ = printfn (sprintf "Starting directory: %s" cwd) in
    let _ = printfn (sprintf "") in

    // Load initial files
    let files = loadDirectory cwd in
    let startModel = { initialModel with currentDir = cwd; files = files } in

    // Register event handlers
    let handlerJ = Events.on "key:j" (fun _ -> printfn (sprintf "Down pressed")) in
    let handlerK = Events.on "key:k" (fun _ -> printfn (sprintf "Up pressed")) in
    let handlerEnter = Events.on "key:enter" (fun _ -> printfn (sprintf "Enter pressed")) in
    let handlerQ = Events.on "key:q" (fun _ -> printfn (sprintf "Quit pressed")) in

    let _ = printfn (sprintf "Event handlers registered: %d, %d, %d, %d" handlerJ handlerK handlerEnter handlerQ) in
    let _ = printfn (sprintf "") in

    // Start event loop
    let finalModel = eventLoop startModel in

    // Cleanup event handlers
    let _ = Events.off handlerJ in
    let _ = Events.off handlerK in
    let _ = Events.off handlerEnter in
    let _ = Events.off handlerQ in

    let _ = printfn (sprintf "Commander TUI exited") in
    printfn (sprintf "Final directory: %s" finalModel.currentDir)

// Run the application
main ()
