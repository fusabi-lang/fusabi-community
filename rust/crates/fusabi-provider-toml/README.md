# fusabi-provider-toml

TOML configuration type provider for Fusabi, generating type-safe configurations from TOML files.

## Overview

This provider analyzes TOML configuration files and generates Fusabi type definitions by inferring types from the actual values in the configuration. It supports:

- Basic types: strings, integers, floats, booleans, datetimes
- Tables (mapped to records)
- Nested tables (mapped to nested records)
- Arrays with homogeneous types
- Array of tables
- Inline tables

## Features

- **Type Inference**: Automatically infers types from TOML values
- **Nested Structures**: Handles deeply nested table structures
- **Array Support**: Supports arrays of primitives and tables
- **TOML 1.0 Compatible**: Full support for TOML 1.0 specification
- **Zero Configuration**: No schema required - types are inferred from actual data

## Usage

```rust
use fusabi_provider_toml::TomlProvider;
use fusabi_type_providers::{TypeProvider, ProviderParams};

let provider = TomlProvider::new();

// From inline TOML
let toml = r#"
    name = "myapp"
    port = 8080

    [database]
    host = "localhost"
    port = 5432
"#;

let schema = provider.resolve_schema(toml, &ProviderParams::default())?;
let types = provider.generate_types(&schema, "Config")?;

// From file
let schema = provider.resolve_schema("config.toml", &ProviderParams::default())?;
let types = provider.generate_types(&schema, "AppConfig")?;
```

## Type Mappings

| TOML Type | Fusabi Type |
|-----------|-------------|
| String | `string` |
| Integer | `int` |
| Float | `float` |
| Boolean | `bool` |
| Datetime | `string` |
| Array | `T list` where T is the element type |
| Table | `record` with inferred fields |

## Example

Given this TOML configuration:

```toml
name = "my-app"
version = "1.0.0"
port = 8080
debug = true

[database]
host = "localhost"
port = 5432
max_connections = 100

[[services]]
name = "api"
port = 8000

[[services]]
name = "web"
port = 8001
```

The provider generates types equivalent to:

```fusabi
type Config = {
    name: string,
    version: string,
    port: int,
    debug: bool,
    database: ConfigDatabase,
    services: ConfigServicesItem list
}

type ConfigDatabase = {
    host: string,
    port: int,
    max_connections: int
}

type ConfigServicesItem = {
    name: string,
    port: int
}
```

## Implementation Details

- Nested tables are extracted as separate type definitions
- Array of tables creates a dedicated item type
- All fields are required (TOML doesn't have a concept of optional fields)
- Datetime values are represented as strings
- Mixed-type arrays use the type of the first element

## License

MIT
