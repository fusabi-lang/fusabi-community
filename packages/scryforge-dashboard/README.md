# Scryforge Dashboard

Code generation dashboard for Scryforge.

## Overview

Scryforge is Raibid Labs' code generation system. This dashboard provides:

- Generation pipeline status
- Template usage metrics
- Output file tracking
- Error and warning display
- Performance metrics

## Installation

```bash
fpm add scryforge-dashboard
```

## Usage

```fsharp
#load "scryforge-dashboard/lib.fsx"

let dashboard = ScryforgeDashboard.create()
let pipelineStatus = ScryforgeDashboard.createPipelineStatus()
```

## License

MIT
