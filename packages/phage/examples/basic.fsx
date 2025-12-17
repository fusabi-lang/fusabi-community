// Basic Phage plugin usage example
// This demonstrates how to use the Phage plugin API

open Scarab.Host

// Check Phage daemon status
let check_phage_status ctx =
    let url = "http://localhost:15702/health"
    match http_get url 1000 with
    | Ok _ ->
        log_info "Phage daemon is running"
        true
    | Error e ->
        log_warn ("Phage daemon not available: " + e)
        false

// Initialize workspace if not exists
let ensure_workspace ctx =
    let cwd = get_cwd ctx
    let phage_dir = cwd + "/.phage"

    if not (file_exists phage_dir) then
        log_info "Initializing Phage workspace..."
        // Trigger init command
        Host.triggerCommand ctx "phage" "init_cmd"
    else
        log_info "Phage workspace already exists"

// Example: Get current context info
let show_context ctx =
    let url = "http://localhost:15702/context/get"
    match http_get url 500 with
    | Ok response ->
        match json_parse response with
        | Ok json ->
            let rules = json |> json_get_path "context.rules" |> json_array_length |> Option.defaultValue 0
            let mcp = json |> json_get_path "context.mcp_configs" |> json_array_length |> Option.defaultValue 0
            log_info (sprintf "Active rules: %d, MCP servers: %d" rules mcp)
        | Error e ->
            log_warn ("Failed to parse: " + e)
    | Error e ->
        log_warn ("Failed to fetch context: " + e)

// Run example
let main ctx =
    log_info "=== Phage Plugin Example ==="

    if check_phage_status ctx then
        ensure_workspace ctx
        show_context ctx
    else
        log_warn "Start the Phage daemon first: phage daemon start"
