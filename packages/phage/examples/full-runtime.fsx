// Phage Plugin for Scarab Terminal - FULL RUNTIME VERSION
// This script requires the full Fusabi runtime with status bar API support.
// Use this when Scarab implements the complete Fusabi integration.
//
// Status bar + AI context management via the Dock & Menu system
//
// Install: fpm add phage
// Repository: https://github.com/raibid-labs/scarab

module Phage

open Scarab.Host
open Scarab.StatusBar
open Scarab.Nav

// ============================================================================
// Plugin Metadata
// ============================================================================

let metadata = {
    Name = "phage"
    Version = "0.2.0"
    Description = "Phage AI context injection and status bar plugin"
    Author = "Raibid Labs"
    ApiVersion = "0.1.0"
    MinScarabVersion = "0.3.0"
}

// ============================================================================
// Configuration
// ============================================================================

let daemon_url = "http://localhost:15702"
let poll_interval_ms = 5000

// ============================================================================
// State
// ============================================================================

// Mutable state for tracking connection and context
let mutable connection_state = "unknown"  // unknown, connected, disconnected, connecting
let mutable rules_count = 0
let mutable mcp_servers_count = 0
let mutable active_layer = ""
let mutable last_error = ""
let mutable status_item_id = 0

// ============================================================================
// Status Bar Rendering
// ============================================================================

// Build status bar items based on current state
let build_status_items () =
    let items = []

    // Separator
    let items = items @ [Separator " | "]

    // Phage icon (DNA helix)
    let icon_color =
        match connection_state with
        | "connected" -> "#00FF00"  // Matrix green
        | "disconnected" -> "#FF5555"  // Red
        | "connecting" -> "#F1FA8C"  // Yellow
        | _ -> "#6272A4"  // Gray

    let items = items @ [
        Foreground (Hex icon_color)
        Text ""  // DNA icon
        ResetForeground
        Padding 1
    ]

    // Connection indicator
    let conn_icon =
        match connection_state with
        | "connected" -> ""
        | "disconnected" -> ""
        | "connecting" -> ""
        | _ -> "?"

    let items = items @ [
        Foreground (Hex icon_color)
        Text conn_icon
        ResetForeground
        Padding 1
    ]

    // Status details based on connection
    let items =
        match connection_state with
        | "connected" ->
            items @ [
                ForegroundAnsi BrightCyan
                Text ("R:" + string rules_count)
                ResetForeground
                Padding 1
                ForegroundAnsi BrightMagenta
                Text ("M:" + string mcp_servers_count)
                ResetForeground
            ] @ (
                if active_layer <> "" then
                    [Padding 1; ForegroundAnsi BrightBlack; Text ("[" + active_layer + "]"); ResetForeground]
                else []
            )
        | "disconnected" ->
            items @ [ForegroundAnsi BrightBlack; Italic; Text "offline"; ResetAttributes]
        | "connecting" ->
            items @ [ForegroundAnsi Yellow; Text "..."; ResetForeground]
        | _ ->
            items @ [ForegroundAnsi BrightBlack; Text "?"; ResetForeground]

    items

// Update the status bar display
let update_status_bar ctx =
    let items = build_status_items ()

    // Remove old status item if exists
    if status_item_id > 0 then
        Host.removeStatusItem ctx status_item_id

    // Add new status item
    let item = {
        Side = Left
        Priority = 100
        Items = items
    }
    status_item_id <- Host.addStatusItem ctx item

// ============================================================================
// Daemon Communication
// ============================================================================

// Poll the Phage daemon for context
let poll_daemon ctx =
    connection_state <- "connecting"
    update_status_bar ctx

    let url = daemon_url + "/context/get"

    match http_get url 500 with
    | Ok response ->
        connection_state <- "connected"
        last_error <- ""

        // Parse JSON response
        match json_parse response with
        | Ok json ->
            // Extract context info
            rules_count <- json |> json_get_path "context.rules" |> json_array_length |> Option.defaultValue 0
            mcp_servers_count <- json |> json_get_path "context.mcp_configs" |> json_array_length |> Option.defaultValue 0
            active_layer <- json |> json_get_path "context.layer" |> json_as_string |> Option.defaultValue ""

            log_debug ("Phage: " + string rules_count + " rules, " + string mcp_servers_count + " MCP servers")
        | Error e ->
            log_warn ("Phage: Failed to parse response: " + e)

        update_status_bar ctx

    | Error e ->
        connection_state <- "disconnected"
        last_error <- e
        log_warn ("Phage: " + e)
        update_status_bar ctx

// ============================================================================
// Plugin Lifecycle
// ============================================================================

let on_load ctx =
    log_info "Phage plugin loaded"
    log_info ("  Daemon URL: " + daemon_url)

    // Initial daemon poll
    poll_daemon ctx

    // Auto-start daemon if not running
    if connection_state = "disconnected" then
        log_info "Phage daemon not running, attempting to start..."
        start_daemon ctx |> ignore

    // Start polling timer
    Host.setInterval ctx poll_interval_ms (fun () -> poll_daemon ctx)

    ()

let on_unload ctx =
    // Remove status bar item
    if status_item_id > 0 then
        Host.removeStatusItem ctx status_item_id

    log_info "Phage plugin unloaded"
    ()

// ============================================================================
// Command Handlers
// ============================================================================

// Check if daemon is running
let is_daemon_running () =
    match http_get (daemon_url + "/health") 500 with
    | Ok _ -> true
    | Error _ -> false

// Start the Phage daemon
let start_daemon ctx =
    if is_daemon_running () then
        notify_info ctx "Daemon Running" "Phage daemon is already running"
        false
    else
        log_info "Starting Phage daemon..."
        match shell_exec_background "phage daemon start" with
        | Ok _ ->
            sleep_ms 1000
            poll_daemon ctx
            if connection_state = "connected" then
                notify_success ctx "Daemon Started" "Phage daemon is now running"
                true
            else
                notify_warn ctx "Daemon Starting" "Daemon is starting up, please wait..."
                true
        | Error e ->
            log_error ("Failed to start daemon: " + e)
            notify_error ctx "Start Failed" ("Could not start daemon: " + e)
            false

// Stop the Phage daemon
let stop_daemon ctx =
    if not (is_daemon_running ()) then
        notify_info ctx "Daemon Not Running" "Phage daemon is not running"
        false
    else
        log_info "Stopping Phage daemon..."
        match shell_exec "phage daemon stop" with
        | Ok _ ->
            connection_state <- "disconnected"
            update_status_bar ctx
            notify_success ctx "Daemon Stopped" "Phage daemon has been stopped"
            true
        | Error e ->
            log_error ("Failed to stop daemon: " + e)
            notify_error ctx "Stop Failed" ("Could not stop daemon: " + e)
            false

// ============================================================================
// Plugin Registration
// ============================================================================

Plugin.Register {
    Metadata = metadata
    OnLoad = Some on_load
    OnUnload = Some on_unload
}
