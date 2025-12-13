# fusabi-provider-sql

SQL DDL (Data Definition Language) type provider for Fusabi. Generates Fusabi type definitions from SQL CREATE TABLE statements.

## Features

- Parse SQL CREATE TABLE statements
- Generate Fusabi RecordDef for each table
- Map SQL types to Fusabi types
- Support for PRIMARY KEY, NOT NULL, DEFAULT, and other constraints
- Handle nullable fields with option types
- Compatible with multiple SQL dialects:
  - PostgreSQL
  - MySQL
  - SQLite
  - Generic SQL

## Supported SQL Types

### Integer Types
- `TINYINT`, `SMALLINT`, `INT`, `INTEGER`, `BIGINT` -> `int`
- `SERIAL`, `BIGSERIAL` -> `int`

### Floating Point Types
- `REAL`, `FLOAT`, `DOUBLE PRECISION` -> `float`
- `DECIMAL(p,s)`, `NUMERIC(p,s)` -> `float`

### String Types
- `CHAR(n)`, `VARCHAR(n)`, `TEXT` -> `string`

### Boolean Type
- `BOOLEAN`, `BOOL` -> `bool`

### Date/Time Types
- `DATE`, `TIME`, `TIMESTAMP`, `TIMESTAMPTZ` -> `string`

### Binary Types
- `BLOB`, `BYTEA`, `BINARY` -> `bytes`

### JSON Types
- `JSON`, `JSONB` -> `string`

### UUID Type
- `UUID` -> `string`

### Array Types (PostgreSQL)
- `TYPE[]` -> `TYPE list`

## Supported Constraints

- `PRIMARY KEY` - marks field as non-nullable
- `NOT NULL` - marks field as non-nullable
- `NULL` - marks field as nullable (generates option type)
- `UNIQUE` - parsed but not reflected in type system
- `DEFAULT value` - parsed but not reflected in type system
- `AUTO_INCREMENT` / `AUTOINCREMENT` - parsed but not reflected in type system

## Usage

### Basic Example

```rust
use fusabi_provider_sql::SqlProvider;
use fusabi_type_providers::{TypeProvider, ProviderParams};

let provider = SqlProvider::new();

let sql = r#"
    CREATE TABLE users (
        id INT PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        email TEXT,
        age INT
    );
"#;

let schema = provider.resolve_schema(sql, &ProviderParams::default())?;
let types = provider.generate_types(&schema, "Database")?;
```

This generates a Fusabi type equivalent to:

```fusabi
type Users = {
    id: int
    name: string
    email: string option
    age: int option
}
```

### Multiple Tables

```rust
let sql = r#"
    CREATE TABLE users (
        id INT PRIMARY KEY,
        name VARCHAR(255) NOT NULL
    );

    CREATE TABLE posts (
        id INT PRIMARY KEY,
        user_id INT NOT NULL,
        title TEXT NOT NULL,
        content TEXT
    );
"#;

let schema = provider.resolve_schema(sql, &ProviderParams::default())?;
let types = provider.generate_types(&schema, "Database")?;
```

Generates:

```fusabi
module Database {
    type Users = {
        id: int
        name: string
    }

    type Posts = {
        id: int
        user_id: int
        title: string
        content: string option
    }
}
```

### PostgreSQL Arrays

```rust
let sql = r#"
    CREATE TABLE articles (
        id SERIAL PRIMARY KEY,
        title TEXT NOT NULL,
        tags TEXT[],
        view_counts INT[]
    );
"#;
```

Generates:

```fusabi
type Articles = {
    id: int
    title: string
    tags: string list option
    view_counts: int list option
}
```

### Loading from File

```rust
// Load SQL schema from file
let schema = provider.resolve_schema("schema.sql", &ProviderParams::default())?;

// Or with file:// prefix
let schema = provider.resolve_schema("file://path/to/schema.sql", &ProviderParams::default())?;
```

## Type Mapping

| SQL Type | Fusabi Type |
|----------|-------------|
| INT, INTEGER, SMALLINT, BIGINT | int |
| REAL, FLOAT, DOUBLE | float |
| DECIMAL, NUMERIC | float |
| CHAR, VARCHAR, TEXT | string |
| BOOLEAN, BOOL | bool |
| DATE, TIME, TIMESTAMP | string |
| BLOB, BYTEA | bytes |
| JSON, JSONB | string |
| UUID | string |
| TYPE[] | TYPE list |

## Nullable Fields

Fields are marked as nullable (wrapped in `option`) when:
- No `NOT NULL` constraint is specified
- Not a `PRIMARY KEY`

Example:

```sql
CREATE TABLE example (
    id INT PRIMARY KEY,           -- not nullable
    required VARCHAR(100) NOT NULL, -- not nullable
    optional VARCHAR(100)           -- nullable -> string option
);
```

## Limitations

- Table-level constraints (composite primary keys, foreign keys) are parsed but not reflected in types
- CHECK constraints are parsed but not validated
- Custom types are passed through as-is
- Complex default expressions are simplified
- No support for views, stored procedures, or triggers
- No schema/namespace support beyond single database

## Implementation Details

The provider consists of three main modules:

### types.rs
Defines the internal representation of SQL types, columns, tables, and constraints.

### parser.rs
Implements a simple SQL DDL parser that:
- Splits statements by semicolons
- Extracts table names and column definitions
- Parses column types and constraints
- Handles quoted identifiers and nested parentheses

### lib.rs
Implements the `TypeProvider` trait and provides the mapping from SQL types to Fusabi types.

## Testing

Run tests with:

```bash
cargo test -p fusabi-provider-sql
```

The test suite includes:
- SQL statement parsing tests
- Type mapping tests
- Constraint handling tests
- Multi-table schema tests
- Array type tests

## License

MIT
