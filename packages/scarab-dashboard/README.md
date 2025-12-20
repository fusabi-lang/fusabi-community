# Scarab Dashboard

Terminal status bar widgets for Scarab terminal emulator.

## Overview

Scarab is Raibid Labs' high-performance split-process terminal emulator built with Bevy. This package provides status bar widgets showing:

- Current working directory
- Git branch and status indicator
- CPU and memory mini gauges
- Current time display
- Active terminal tabs count

## Installation

```bash
fpm add scarab-dashboard
```

## Usage

```fsharp
#load "scarab-dashboard/lib.fsx"

// Create status bar components
let dirDisplay = ScarabDashboard.createDirectoryDisplay "/home/user/project"
let gitDisplay = ScarabDashboard.createGitBranchDisplay "main" Modified
let cpuGauge = ScarabDashboard.createCpuMiniGauge 45
let memGauge = ScarabDashboard.createMemoryMiniGauge 62
let timeDisplay = ScarabDashboard.createTimeDisplay "14:32"
let tabCount = ScarabDashboard.createTabCountDisplay 3
```

## Git Status Colors

- **Green** - Clean (no changes)
- **Yellow** - Modified (uncommitted changes)
- **Cyan** - Staged (changes staged)
- **Red** - Conflict (merge conflict)
- **White** - Not a git repository

## License

MIT
