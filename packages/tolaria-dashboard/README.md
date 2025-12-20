# Tolaria Dashboard

Cluster monitoring dashboard for Tolaria.

## Overview

Tolaria is Raibid Labs' distributed compute cluster. This dashboard provides:

- Node status and health monitoring
- Resource allocation visualization
- Job queue status
- Network topology display
- Performance metrics

## Installation

```bash
fpm add tolaria-dashboard
```

## Usage

```fsharp
#load "tolaria-dashboard/lib.fsx"

let dashboard = TolariaDashboard.create()
let nodeStatus = TolariaDashboard.createNodeStatus()
let jobQueue = TolariaDashboard.createJobQueue()
```

## License

MIT
