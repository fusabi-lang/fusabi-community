# Sigilforge Dashboard

Symbol management dashboard for Sigilforge.

## Overview

Sigilforge is Raibid Labs' symbol management system. This dashboard provides:

- Symbol registry visualization
- Binding status tracking
- Namespace hierarchy display
- Symbol resolution metrics
- Reference tracking

## Installation

```bash
fpm add sigilforge-dashboard
```

## Usage

```fsharp
#load "sigilforge-dashboard/lib.fsx"

let dashboard = SigilforgeDashboard.create()
let symbolList = SigilforgeDashboard.createSymbolList()
```

## License

MIT
