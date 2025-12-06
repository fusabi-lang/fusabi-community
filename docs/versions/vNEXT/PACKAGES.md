# Package Documentation

## Available Packages

### json (v0.1.0)

JSON parsing and serialization combinators built on top of Fusabi's standard library.

**Features:**
- Parse JSON strings into structured data
- Serialize data to JSON format
- Path traversal and accessor utilities
- Type-safe value extraction

**Location:** `packages/json/`

See [packages/json/README.md](../../../packages/json/README.md) for detailed documentation.

### commander (v0.1.0)

A TUI (Terminal User Interface) file manager built with Fusabi.

**Features:**
- Navigate file systems
- File operations (view, edit, delete)
- Terminal-based interface

**Location:** `packages/commander/`

See [packages/commander/README.md](../../../packages/commander/README.md) for detailed documentation.

## Package Structure

Each package follows this structure:

```
packages/<package-name>/
├── fusabi.toml          # Package metadata
├── README.md            # Package documentation
├── src/
│   ├── main.fsx         # Entry point for executables
│   └── lib.fsx          # Library entry point
├── examples/            # Example usage
└── tests/               # Test files
```

## Capability Metadata

Packages may declare capabilities for use with fusabi-plugin-runtime. See [CAPABILITIES.md](CAPABILITIES.md) for the schema and examples.
