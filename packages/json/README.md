# json - JSON Parsing and Serialization

A Fusabi library providing JSON parsing and serialization combinators built on top of the standard library.

## Features

- Parse JSON strings into structured data
- Serialize data to JSON format
- Path traversal and accessor utilities
- Type-safe value extraction
- Pure library with no runtime dependencies

## Installation

Add to your project's dependencies or download from the registry:

```bash
# Using the registry (future)
fus add json

# Or clone directly
git clone https://github.com/fusabi-lang/fusabi-community.git
```

## Usage

### Basic Parsing

```fusabi
use json::*;

let data = Json.parse("{\"name\": \"John\", \"age\": 30}");

match data {
  Ok(value) => {
    // Access fields
    let name = Json.getString(value, "name");
    let age = Json.getInt(value, "age");

    print("Name: " + name);
    print("Age: " + toString(age));
  }
  Err(e) => print("Parse error: " + e)
}
```

### Serialization

```fusabi
use json::*;

let obj = Json.object([
  ("name", Json.string("John")),
  ("age", Json.number(30)),
  ("active", Json.bool(true))
]);

let jsonStr = Json.stringify(obj);
print(jsonStr);
// Output: {"name":"John","age":30,"active":true}
```

### Path Traversal

```fusabi
use json::*;

let data = Json.parse("{\"user\": {\"profile\": {\"name\": \"Alice\"}}}");

match data {
  Ok(value) => {
    // Navigate nested structures
    let name = Json.get(value, "user.profile.name");
    match name {
      Some(n) => print("Name: " + Json.asString(n)),
      None => print("Name not found")
    }
  }
  Err(e) => print("Error: " + e)
}
```

### Array Operations

```fusabi
use json::*;

let arr = Json.array([
  Json.string("apple"),
  Json.string("banana"),
  Json.string("cherry")
]);

let items = Json.asArray(arr);
match items {
  Some(list) => {
    list.forEach(item => {
      print(Json.asString(item));
    });
  }
  None => print("Not an array")
}
```

## API Reference

### Parsing

- `Json.parse(str: String) -> Result<Value, String>` - Parse JSON string

### Serialization

- `Json.stringify(value: Value) -> String` - Convert to JSON string
- `Json.prettyPrint(value: Value, indent: Int) -> String` - Formatted output

### Constructors

- `Json.object(pairs: [(String, Value)]) -> Value` - Create object
- `Json.array(items: [Value]) -> Value` - Create array
- `Json.string(s: String) -> Value` - Create string value
- `Json.number(n: Number) -> Value` - Create number value
- `Json.bool(b: Bool) -> Value` - Create boolean value
- `Json.null() -> Value` - Create null value

### Accessors

- `Json.get(value: Value, path: String) -> Option<Value>` - Path traversal
- `Json.getString(value: Value, key: String) -> Option<String>`
- `Json.getInt(value: Value, key: String) -> Option<Int>`
- `Json.getBool(value: Value, key: String) -> Option<Bool>`

### Type Converters

- `Json.asString(value: Value) -> Option<String>`
- `Json.asInt(value: Value) -> Option<Int>`
- `Json.asFloat(value: Value) -> Option<Float>`
- `Json.asBool(value: Value) -> Option<Bool>`
- `Json.asArray(value: Value) -> Option<[Value]>`
- `Json.asObject(value: Value) -> Option<[(String, Value)]>`

## Integration with fusabi-stdlib-ext

This package is designed to work seamlessly with fusabi-stdlib-ext modules:

```fusabi
use fusabi_stdlib_ext::io;
use json::*;

// Read JSON from file
let content = io.readFile("data.json");
match content {
  Ok(str) => {
    let data = Json.parse(str);
    // Process data...
  }
  Err(e) => print("Error reading file: " + e)
}
```

## Capabilities

This is a pure library package with no runtime requirements:

- **Requires**: None
- **Compatible Runtimes**: scarab, hibana, fusabi-plugin-runtime
- **Permissions**: None required

See [fusabi.toml](./fusabi.toml) for full capability metadata.

## Examples

See the [examples](./examples/) directory for more usage examples:

- `basic_parsing.fsx` - Simple parsing and access
- `serialization.fsx` - Creating and serializing JSON
- `file_processing.fsx` - Reading/writing JSON files
- `nested_data.fsx` - Working with nested structures

## Testing

Run the test suite:

```bash
fus test src/lib.fsx
```

## Contributing

Contributions are welcome! Please see the main [repository README](../../README.md) for contribution guidelines.

## License

MIT License - see [LICENSE](../../LICENSE) file for details.

## See Also

- [Fusabi Standard Library](https://github.com/fusabi-lang/fusabi)
- [fusabi-stdlib-ext](https://github.com/fusabi-lang/fusabi-stdlib-ext)
- [Community Registry](../../registry/index.toml)
