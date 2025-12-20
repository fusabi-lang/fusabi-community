# Phage Dashboard

Context visualization dashboard for Phage memory system.

## Overview

Phage is Raibid Labs' intelligent context management system. This dashboard provides:

- Memory usage and allocation monitoring
- Active contexts with topic tracking
- Recent topic events and transitions
- Context tree visualization
- Memory persistence status

## Installation

```bash
fpm add phage-dashboard
```

## Usage

```fsharp
#load "phage-dashboard/lib.fsx"

let dashboard = PhageDashboard.create()
let memoryGauge = PhageDashboard.createMemoryGauge 45 100
let contextList = PhageDashboard.createContextList()
```

## License

MIT
