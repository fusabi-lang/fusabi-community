# Fusabi Protobuf Type Provider

A type provider for Fusabi that generates Fusabi types from Protocol Buffer (.proto) definitions.

## Features

- Parse proto2 and proto3 syntax
- Support for message definitions (converted to Records)
- Support for enum definitions (converted to Discriminated Unions)
- Support for nested types (messages and enums)
- Support for all protobuf scalar types (int32, int64, string, bytes, bool, etc.)
- Support for repeated fields (converted to lists)
- Support for optional fields (converted to option types)
- Support for map fields
- Support for service definitions (parsed but not converted to types)
- Support for imports and packages
- File and inline proto content support

## Usage

```rust
use fusabi_provider_protobuf::ProtobufProvider;
use fusabi_type_providers::{TypeProvider, ProviderParams};

let provider = ProtobufProvider::new();

// Load from file
let schema = provider.resolve_schema("schema.proto", &ProviderParams::default())?;
let types = provider.generate_types(&schema, "MyProto")?;

// Or use inline proto content
let proto = r#"
    syntax = "proto3";

    message User {
        string name = 1;
        int32 age = 2;
    }
"#;
let schema = provider.resolve_schema(proto, &ProviderParams::default())?;
let types = provider.generate_types(&schema, "User")?;
```

## Type Mapping

| Protobuf Type | Fusabi Type |
|--------------|-------------|
| double, float | float |
| int32, sint32, sfixed32 | int |
| int64, sint64, sfixed64 | int64 |
| uint32, fixed32 | uint |
| uint64, fixed64 | uint64 |
| bool | bool |
| string | string |
| bytes | bytes |
| message Foo | Foo |
| enum Bar | Bar (as DU) |
| repeated T | T list |
| optional T | T option |
| map<K, V> | Map<K, V> list |

## Example

Given this proto file:

```protobuf
syntax = "proto3";
package user.v1;

enum Status {
    UNKNOWN = 0;
    ACTIVE = 1;
    INACTIVE = 2;
}

message User {
    string id = 1;
    string name = 2;
    Status status = 3;
    repeated string tags = 4;
    map<string, string> metadata = 5;
}
```

The provider will generate:
- A module with path `["user", "v1"]`
- A DU type `Status` with variants: `Unknown`, `Active`, `Inactive`
- A Record type `User` with fields:
  - `id: string`
  - `name: string`
  - `status: Status`
  - `tags: string list`
  - `metadata: Map<string, string> list`

## Implementation Details

The provider consists of three main components:

1. **types.rs**: Defines the AST for protobuf files (ProtoFile, Message, Enum, etc.)
2. **parser.rs**: Simple lexer and parser for .proto files
3. **lib.rs**: TypeProvider implementation that converts proto AST to Fusabi types

## Testing

Run tests with:

```bash
cargo test -p fusabi-provider-protobuf
```

All tests pass, including:
- Parser tests for messages, enums, and nested types
- Type generation tests for various proto constructs
- Comprehensive integration tests

## License

MIT
