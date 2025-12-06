# commander - Terminal File Manager

A powerful TUI (Terminal User Interface) file manager built with Fusabi, providing an intuitive way to navigate and manage files from the command line.

## Features

- Interactive terminal-based file browser
- File operations (view, edit, delete, copy, move)
- Directory navigation with breadcrumbs
- File search and filtering
- Syntax highlighting for text files
- Keyboard shortcuts for efficient navigation
- Permissions and metadata display

## Installation

```bash
# Using the registry (future)
fus add commander

# Or clone directly
git clone https://github.com/fusabi-lang/fusabi-community.git
cd fusabi-community/packages/commander
fus run src/main.fsx
```

## Usage

### Basic Navigation

```bash
# Launch commander in current directory
fus run src/main.fsx

# Launch in specific directory
fus run src/main.fsx /path/to/directory
```

### Keyboard Shortcuts

- `↑/↓` or `j/k` - Navigate files
- `←/→` or `h/l` - Navigate directories (back/forward)
- `Enter` - Open file or enter directory
- `Space` - Select/deselect file
- `d` - Delete selected files
- `c` - Copy selected files
- `m` - Move selected files
- `n` - Create new file
- `N` - Create new directory
- `/` - Search files
- `?` - Show help
- `q` - Quit

### File Operations

```fusabi
# View file contents
# Press 'v' on any file

# Edit file
# Press 'e' to open in default editor

# Delete files
# Select with Space, then press 'd'

# Copy/Move files
# Select files, press 'c' or 'm', navigate to destination, press 'p' to paste
```

## Configuration

Create a configuration file at `~/.config/commander/config.toml`:

```toml
[appearance]
theme = "dark"
show_hidden = false
icons = true

[editor]
default = "vim"
use_external = true

[shortcuts]
quit = "q"
help = "?"
search = "/"
```

## Integration with fusabi-stdlib-ext

Commander uses fusabi-stdlib-ext for file system operations:

```fusabi
use fusabi_stdlib_ext::fs;
use fusabi_stdlib_ext::io;

// Read directory contents
let entries = fs.readDir("/home/user");

// Get file metadata
let info = fs.stat("file.txt");

// Read file contents
let content = io.readFile("document.txt");
```

## Capabilities

Commander requires the following runtime capabilities:

- **stdio**: Terminal input/output
- **filesystem**: File system access
- **tui**: Terminal UI rendering

### Permissions

- **Read**: Browse file system (`/`)
- **Write**: Edit and manage files in accessible directories

**Compatible Runtimes**: scarab, fusabi-plugin-runtime

See [fusabi.toml](./fusabi.toml) for full capability metadata.

## Examples

### Custom File Viewer

```fusabi
use commander::FileViewer;

let viewer = FileViewer.new();
viewer.registerHandler("*.md", (path) => {
  // Custom markdown viewer
  let content = readFile(path);
  renderMarkdown(content);
});
```

### Plugin Extension

```fusabi
use commander::Plugin;

let myPlugin = Plugin.new("git-status", {
  onFileSelect: (path) => {
    let status = shell.exec("git status " + path);
    print(status);
  }
});

commander.registerPlugin(myPlugin);
```

## Architecture

Commander is built with a modular architecture:

```
src/
├── main.fsx           # Entry point and main loop
├── ui/
│   ├── layout.fsx     # Layout manager
│   ├── components.fsx # UI components
│   └── theme.fsx      # Theming system
├── fs/
│   ├── browser.fsx    # File browser logic
│   ├── operations.fsx # File operations
│   └── watcher.fsx    # File system watcher
└── plugins/
    └── system.fsx     # Plugin system
```

## Testing

Run the test suite:

```bash
fus test src/main.fsx
```

Run in development mode with hot reload:

```bash
fus dev src/main.fsx
```

## Troubleshooting

### Terminal rendering issues

If you experience rendering problems:

```bash
# Force 256-color mode
TERM=xterm-256color fus run src/main.fsx

# Disable icons if font doesn't support them
fus run src/main.fsx --no-icons
```

### Permission errors

Ensure commander has appropriate file system permissions:

```bash
# Run with capability constraints
fus run --capabilities=stdio,filesystem,tui src/main.fsx
```

## Comparison with Similar Tools

| Feature | Commander | ranger | nnn | mc |
|---------|-----------|--------|-----|-----|
| Language | Fusabi | Python | C | C |
| Memory | Low | Medium | Low | Medium |
| Extensibility | High | High | Low | Medium |
| UI Framework | Custom TUI | curses | curses | S-Lang |

## Contributing

Contributions are welcome! Areas for improvement:

- Additional file type viewers
- Plugin ecosystem
- Performance optimizations
- Accessibility features

See the main [repository README](../../README.md) for contribution guidelines.

## License

MIT License - see [LICENSE](../../LICENSE) file for details.

## See Also

- [Fusabi TUI Framework](https://github.com/fusabi-lang/fusabi-tui)
- [fusabi-stdlib-ext](https://github.com/fusabi-lang/fusabi-stdlib-ext)
- [Community Registry](../../registry/index.toml)

## Credits

Inspired by:
- [ranger](https://github.com/ranger/ranger)
- [nnn](https://github.com/jarun/nnn)
- [Midnight Commander](https://midnight-commander.org/)
