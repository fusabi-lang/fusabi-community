# Hibana Dashboard

GPU metrics dashboard for Hibana monitoring systems.

## Overview

Hibana is Raibid Labs' GPU compute orchestration system. This dashboard provides real-time monitoring of:

- Multi-GPU utilization gauges
- VRAM memory usage per GPU
- Temperature monitoring with sparklines
- Power consumption tracking
- Running process list with GPU assignment
- System health status

## Installation

```bash
fpm add hibana-dashboard
```

## Usage

```fsharp
#load "hibana-dashboard/lib.fsx"

// Create dashboard widgets
let dashboard = HibanaDashboard.create()

// Create individual components
let gpu0Gauge = HibanaDashboard.createGpuGauge 0 "RTX 4090" 78 Active
let vramGauge = HibanaDashboard.createVramGauge 0 16 24
let tempSparkline = HibanaDashboard.createTempSparkline 0 68
let powerGauge = HibanaDashboard.createPowerGauge 890 1200 4
```

## Components

| Widget | Description |
|--------|-------------|
| `createGpuGauge` | GPU utilization gauge with status coloring |
| `createVramGauge` | VRAM memory usage gauge |
| `createTempSparkline` | Temperature history sparkline |
| `createPowerGauge` | Total power consumption gauge |
| `createProcessList` | List of running GPU processes |
| `createHealthStatus` | System health status panel |
| `createNavigationTabs` | Tab navigation |
| `createHelpSection` | Keyboard shortcut help |

## GPU Status Colors

- **White** - Idle (< 10% utilization)
- **Green** - Active (10-80%)
- **Yellow** - High Load (80-95%)
- **Red** - Maxed (> 95%) or Error

## Requirements

- fusabi-dashboards
- Optional: nvml capability for live GPU data

## License

MIT
