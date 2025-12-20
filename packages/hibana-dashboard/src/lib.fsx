// Hibana GPU Metrics Dashboard
// Real-time GPU monitoring and process management
//
// Hibana is Raibid Labs' GPU compute orchestration system for managing
// distributed training workloads, model inference, and GPU resource allocation.
//
// Usage:
//   #load "hibana-dashboard/lib.fsx"
//   let dashboard = HibanaDashboard.create()

#r "fusabi-dashboards"
#load "fusabi-dashboards/fsx/tui.fsx"

module HibanaDashboard =

    // ========================================================================
    // Data Models
    // ========================================================================

    type GpuStatus =
        | Idle           // < 10% utilization
        | Active         // 10-80% utilization
        | HighLoad       // 80-95% utilization
        | Maxed          // > 95% utilization
        | Error          // GPU error state

    type ProcessType =
        | Training       // Model training
        | Inference      // Model inference
        | Compute        // General compute
        | Graphics       // Graphics rendering

    // ========================================================================
    // Helper Functions
    // ========================================================================

    let getGpuStatusColor status =
        match status with
        | Idle -> white
        | Active -> green
        | HighLoad -> yellow
        | Maxed -> red
        | Error -> red

    let getProcessTypeColor processType =
        match processType with
        | Training -> magenta
        | Inference -> cyan
        | Compute -> green
        | Graphics -> yellow

    let formatMemoryGB gb = string gb + " GB"
    let formatTemp celsius = string celsius + "°C"
    let formatWatts watts = string watts + "W"

    // ========================================================================
    // Title Block
    // ========================================================================

    let createTitleBlock =
        let titleStyle = emptyStyle |> withFg red |> withBg black in
        createBlock
        |> withTitle "Hibana GPU Metrics Dashboard"
        |> withBorders allBorders
        |> withBorderType doubleBorder
        |> withBlockStyle titleStyle

    // ========================================================================
    // GPU Utilization Gauges
    // ========================================================================

    let createGpuGauge gpuIndex name utilization status =
        let gaugeBlock = createBlock
                         |> withTitle (sprintf "GPU %d - %s" gpuIndex name)
                         |> withBorders allBorders
                         |> withBorderType roundedBorder in
        let gaugeStyle = emptyStyle |> withFg white in
        let barStyle = emptyStyle |> withFg (getGpuStatusColor status) in
        let label = sprintf "%d%% utilized" utilization in

        gaugeFromPercent utilization
        |> withLabel label
        |> withGaugeBlock gaugeBlock
        |> withGaugeStyle gaugeStyle
        |> withGaugeBarStyle barStyle

    // ========================================================================
    // VRAM Memory Gauge
    // ========================================================================

    let createVramGauge gpuIndex usedGB totalGB =
        let percent = (usedGB * 100) / totalGB in
        let gaugeBlock = createBlock
                         |> withTitle (sprintf "GPU %d VRAM" gpuIndex)
                         |> withBorders allBorders
                         |> withBorderType plainBorder in
        let gaugeStyle = emptyStyle |> withFg white in
        let barStyle = emptyStyle |> withFg cyan in
        let label = sprintf "%s / %s" (formatMemoryGB usedGB) (formatMemoryGB totalGB) in

        gaugeFromPercent percent
        |> withLabel label
        |> withGaugeBlock gaugeBlock
        |> withGaugeStyle gaugeStyle
        |> withGaugeBarStyle barStyle

    // ========================================================================
    // Temperature Sparkline
    // ========================================================================

    let createTempSparkline gpuIndex currentTemp =
        let color = if currentTemp > 75 then red else if currentTemp > 60 then yellow else green in
        let sparklineBlock = createBlock
                             |> withTitle (sprintf "GPU %d Temperature (1h)" gpuIndex)
                             |> withBorders allBorders
                             |> withBorderType plainBorder in
        let sparklineStyle = emptyStyle |> withFg color in

        sparklineFromData currentTemp
        |> withMax 100
        |> withSparklineBlock sparklineBlock
        |> withSparklineStyle sparklineStyle
        |> withDirection leftToRight

    // ========================================================================
    // Power Consumption Gauge
    // ========================================================================

    let createPowerGauge currentWatts maxWatts gpuCount =
        let percent = (currentWatts * 100) / maxWatts in
        let gaugeBlock = createBlock
                         |> withTitle "Total Power Consumption"
                         |> withBorders allBorders
                         |> withBorderType roundedBorder in
        let gaugeStyle = emptyStyle |> withFg white in
        let barStyle = emptyStyle |> withFg yellow in
        let label = sprintf "%dW / %dW (%d GPUs)" currentWatts maxWatts gpuCount in

        gaugeFromPercent percent
        |> withLabel label
        |> withGaugeBlock gaugeBlock
        |> withGaugeStyle gaugeStyle
        |> withGaugeBarStyle barStyle

    // ========================================================================
    // Process List
    // ========================================================================

    let createProcessItem gpuIndex pid name vramGB processType =
        let itemStyle = emptyStyle |> withFg (getProcessTypeColor processType) in
        let typeStr = match processType with
                      | Training -> "TRAINING"
                      | Inference -> "INFERENCE"
                      | Compute -> "COMPUTE"
                      | Graphics -> "GRAPHICS" in
        let itemText = sprintf "GPU %d | PID %d | %-16s %5.1fGB  [%s]" gpuIndex pid name vramGB typeStr in
        styledListItem itemText itemStyle

    let createProcessList =
        let listBlock = createBlock
                        |> withTitle "Active GPU Processes"
                        |> withBorders allBorders
                        |> withBorderType roundedBorder
                        |> withBlockStyle (emptyStyle |> withFg white) in
        let highlightStyle = emptyStyle |> withFg black |> withBg red in

        // Placeholder - actual processes would be passed in
        let proc1 = createProcessItem 0 12345 "llama-train" 15.2 Training in

        createList proc1
        |> withListBlock listBlock
        |> withHighlightStyle highlightStyle

    // ========================================================================
    // Navigation Tabs
    // ========================================================================

    let createNavigationTabs selectedTab =
        let tabsBlock = createBlock |> withBorders bottomBorder in
        let normalStyle = emptyStyle |> withFg white in
        let highlightStyle = emptyStyle |> withFg red |> withBold in

        tabsFromTitles "Overview"
        |> withSelected selectedTab
        |> withDivider pipeDivider
        |> withTabsBlock tabsBlock
        |> withTabsStyle normalStyle
        |> withTabsHighlightStyle highlightStyle

    // ========================================================================
    // Health Status
    // ========================================================================

    let createHealthStatus statusText warningText errorText =
        let healthBlock = createBlock
                          |> withTitle "System Health"
                          |> withBorders allBorders
                          |> withBorderType roundedBorder in

        let healthText = textFromString (sprintf "Status: %s\nWarnings: %s\nErrors: %s" statusText warningText errorText) in
        let healthStyle = if errorText = "None" then emptyStyle |> withFg green else emptyStyle |> withFg red in

        createParagraph healthText
        |> withParagraphBlock healthBlock
        |> withParagraphStyle healthStyle
        |> withAlignment leftAlign
        |> withWrap wrapWord

    // ========================================================================
    // Help Section
    // ========================================================================

    let createHelpSection =
        let helpBlock = createBlock
                        |> withTitle "Keyboard Shortcuts"
                        |> withBorders allBorders
                        |> withBorderType roundedBorder in

        let helpText = textFromString "g: GPU Details | p: Processes | m: Memory | t: Temperature | k: Kill Process | q: Quit" in
        let helpStyle = emptyStyle |> withFg red in

        createParagraph helpText
        |> withParagraphBlock helpBlock
        |> withParagraphStyle helpStyle
        |> withAlignment centerAlign
        |> withWrap wrapWord

    // ========================================================================
    // Dashboard Factory
    // ========================================================================

    let create () =
        createTitleBlock

// Export for easy access
let create = HibanaDashboard.create
let createGpuGauge = HibanaDashboard.createGpuGauge
let createVramGauge = HibanaDashboard.createVramGauge
let createTempSparkline = HibanaDashboard.createTempSparkline
let createPowerGauge = HibanaDashboard.createPowerGauge
let createProcessList = HibanaDashboard.createProcessList
let createHealthStatus = HibanaDashboard.createHealthStatus
let createHelpSection = HibanaDashboard.createHelpSection
