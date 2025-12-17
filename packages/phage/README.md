# Phage Plugin for Scarab

Phage AI context injection and status bar plugin for the Scarab terminal emulator.

## Features

- **Status Bar Integration**: Real-time display of Phage daemon connection status
- **Context Monitoring**: Shows active rules count, MCP servers, and current layer
- **Workspace Management**: Initialize Phage workspaces from the terminal
- **Scarab-Nav Integration**: Keyboard-driven navigation support
- **Dock Menu**: Quick access to Phage commands

## Installation

```bash
# Using the Fusabi package manager
fpm add phage

# Or copy manually to Scarab plugins directory
cp -r . ~/.config/scarab/plugins/phage/
```

## Requirements

- Scarab >= 0.3.0
- Phage daemon running on `localhost:15702`

## Status Bar Display

When connected:
```
 |   R:5 M:3 [project]
```

- ``: DNA icon (Phage branding)
- ``: Connection status (green = connected)
- `R:5`: Number of active rules
- `M:3`: Number of MCP servers
- `[project]`: Current active layer

When disconnected:
```
 |   offline
```

## Menu Commands

| Command | Shortcut | Description |
|---------|----------|-------------|
| Init Workspace | `Ctrl+Alt+I` | Initialize Phage workspace in current directory |
| Chat | `Ctrl+Alt+C` | Open Phage chat interface |
| Explain Selection | - | Explain selected text using AI |
| Fix Last Command | - | Get AI suggestion to fix last failed command |
| Context Info | - | Show current Phage context details |
| Refresh Status | - | Manually refresh daemon status |

## Configuration

The plugin polls the Phage daemon every 5 seconds by default. Configuration options:

```fsharp
// In phage.fsx
let daemon_url = "http://localhost:15702"
let poll_interval_ms = 5000
```

## Workspace Structure

When initializing a workspace, the following structure is created:

```
.phage/
├── workspace.toml        # Workspace metadata
├── .gitignore           # Ignores session/ and logs
└── layers/
    ├── base/
    │   └── config.toml  # Organization-wide rules
    ├── project/
    │   └── config.toml  # Project-specific rules
    └── session/
        └── config.toml  # Runtime session overrides (volatile)
```

## Capabilities Required

- `network`: Communication with Phage daemon
- `filesystem`: Workspace initialization
- `status_bar`: Status bar rendering
- `navigation`: Scarab-nav integration

## License

MIT License - See [LICENSE](../../LICENSE) for details.

## Related

- [Phage](https://github.com/raibid-labs/phage) - AI context injection system
- [Scarab](https://github.com/raibid-labs/scarab) - Terminal emulator
- [Scarab-Nav](https://github.com/raibid-labs/scarab-nav) - Navigation plugin
