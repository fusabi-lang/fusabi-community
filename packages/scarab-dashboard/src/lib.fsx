// Scarab Terminal Status Bar
// Status bar widgets for Scarab terminal emulator
//
// Usage:
//   #load "scarab-dashboard/lib.fsx"
//   let statusBar = ScarabDashboard.createStatusBar()

#r "fusabi-dashboards"
#load "fusabi-dashboards/fsx/tui.fsx"

module ScarabDashboard =

    // Git repository status
    type GitStatus =
        | Clean        // No changes
        | Modified     // Files modified
        | Staged       // Changes staged
        | Conflict     // Merge conflict
        | NotRepo      // Not a git repository

    // Get git status color
    let getGitStatusColor status =
        match status with
        | Clean -> green
        | Modified -> yellow
        | Staged -> cyan
        | Conflict -> red
        | NotRepo -> white

    // Get git status symbol
    let getGitStatusSymbol status =
        match status with
        | Clean -> "✓"
        | Modified -> "●"
        | Staged -> "+"
        | Conflict -> "✗"
        | NotRepo -> "-"

    // Current directory display
    let createDirectoryDisplay path =
        let dirBlock = createBlock
                       |> withBorders rightBorder
                       |> withBorderType plainBorder in
        let dirText = textFromString path in
        let dirStyle = emptyStyle |> withFg cyan |> withBold in

        createParagraph dirText
        |> withParagraphBlock dirBlock
        |> withParagraphStyle dirStyle
        |> withAlignment leftAlign
        |> withWrap wrapWord

    // Git branch indicator
    let createGitBranchDisplay branch status =
        let gitBlock = createBlock
                       |> withBorders rightBorder
                       |> withBorderType plainBorder in
        let statusColor = getGitStatusColor status in
        let statusSymbol = getGitStatusSymbol status in
        let gitText = textFromString (branch + " " + statusSymbol) in
        let gitStyle = emptyStyle |> withFg statusColor in

        createParagraph gitText
        |> withParagraphBlock gitBlock
        |> withParagraphStyle gitStyle
        |> withAlignment leftAlign
        |> withWrap wrapWord

    // CPU mini gauge
    let createCpuMiniGauge percent =
        let gaugeStyle = emptyStyle |> withFg white in
        let barStyle = if percent > 80 then emptyStyle |> withFg red
                       else if percent > 50 then emptyStyle |> withFg yellow
                       else emptyStyle |> withFg green in

        gaugeFromPercent percent
        |> withLabel (sprintf "CPU %d%%" percent)
        |> withGaugeStyle gaugeStyle
        |> withGaugeBarStyle barStyle

    // Memory mini gauge
    let createMemoryMiniGauge percent =
        let gaugeStyle = emptyStyle |> withFg white in
        let barStyle = emptyStyle |> withFg cyan in

        gaugeFromPercent percent
        |> withLabel (sprintf "MEM %d%%" percent)
        |> withGaugeStyle gaugeStyle
        |> withGaugeBarStyle barStyle

    // Time display
    let createTimeDisplay timeStr =
        let timeBlock = createBlock
                        |> withBorders leftBorder
                        |> withBorderType plainBorder in
        let timeText = textFromString timeStr in
        let timeStyle = emptyStyle |> withFg white in

        createParagraph timeText
        |> withParagraphBlock timeBlock
        |> withParagraphStyle timeStyle
        |> withAlignment rightAlign
        |> withWrap wrapWord

    // Tab count display
    let createTabCountDisplay tabCount =
        let tabText = textFromString (sprintf "⊞ %d" tabCount) in
        let tabStyle = emptyStyle |> withFg magenta in

        createParagraph tabText
        |> withParagraphStyle tabStyle
        |> withAlignment centerAlign
        |> withWrap wrapWord

    // Create full status bar
    let createStatusBar () =
        let statusBlock = createBlock
                          |> withBorders topBorder
                          |> withBorderType plainBorder
                          |> withBlockStyle (emptyStyle |> withBg black) in
        statusBlock

// Exports
let createDirectoryDisplay = ScarabDashboard.createDirectoryDisplay
let createGitBranchDisplay = ScarabDashboard.createGitBranchDisplay
let createCpuMiniGauge = ScarabDashboard.createCpuMiniGauge
let createMemoryMiniGauge = ScarabDashboard.createMemoryMiniGauge
let createTimeDisplay = ScarabDashboard.createTimeDisplay
let createTabCountDisplay = ScarabDashboard.createTabCountDisplay
let createStatusBar = ScarabDashboard.createStatusBar
