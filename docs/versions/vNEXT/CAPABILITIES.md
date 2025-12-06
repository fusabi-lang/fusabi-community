# Capability Metadata

## Overview

Packages can declare capabilities that describe their runtime requirements and features for integration with fusabi-plugin-runtime and host environments like scarab and hibana.

## Capability Schema

Capabilities are declared in the package's `fusabi.toml` file under the `[capabilities]` section.

### Basic Structure

```toml
[package]
name = "my-package"
version = "0.1.0"

[capabilities]
# Required runtime features
requires = ["stdio", "filesystem", "network"]

# Optional features
optional = ["tui", "graphics"]

# Permissions needed
permissions = [
    { type = "read", path = "/home" },
    { type = "write", path = "/tmp" },
    { type = "network", scope = "all" }
]

# Runtime compatibility
compatible_with = ["scarab", "hibana", "fusabi-plugin-runtime"]
```

## Capability Types

### Standard Capabilities

- **stdio**: Standard input/output
- **filesystem**: File system access
- **network**: Network operations
- **tui**: Terminal UI capabilities
- **graphics**: Graphics/rendering capabilities
- **ipc**: Inter-process communication
- **env**: Environment variable access

### Permission Model

Permissions use a fine-grained model:

```toml
[[capabilities.permissions]]
type = "read"
path = "/home/user/data"
description = "Read user data files"

[[capabilities.permissions]]
type = "write"
path = "/tmp"
description = "Write temporary files"

[[capabilities.permissions]]
type = "network"
scope = "localhost"
port = 8080
description = "Connect to local development server"
```

### Runtime Compatibility

Declare which runtime environments the package supports:

```toml
[capabilities]
compatible_with = ["scarab", "hibana"]

[capabilities.runtime_requirements]
min_version = "0.1.0"
features = ["async", "threading"]
```

## Example: JSON Package

```toml
[package]
name = "json"
version = "0.1.0"
description = "JSON parsing and serialization combinators"

[capabilities]
requires = []  # Pure library, no runtime requirements
compatible_with = ["scarab", "hibana", "fusabi-plugin-runtime"]
```

## Example: Commander Package

```toml
[package]
name = "commander"
version = "0.1.0"
description = "A TUI file manager"

[capabilities]
requires = ["stdio", "filesystem", "tui"]

[[capabilities.permissions]]
type = "read"
path = "/"
description = "Browse file system"

[[capabilities.permissions]]
type = "write"
path = "/home"
description = "Edit files in home directory"

[capabilities]
compatible_with = ["scarab", "fusabi-plugin-runtime"]
```

## Validation

The CI pipeline validates capability declarations against the schema. See `.github/workflows/validate-capabilities.yml` for the validation workflow.

## Integration with fusabi-stdlib-ext

Packages should use fusabi-stdlib-ext modules when implementing capabilities:

```fusabi
// Import from stdlib-ext
use fusabi_stdlib_ext::fs;
use fusabi_stdlib_ext::io;
use fusabi_stdlib_ext::net;

// Declare capability requirements
// (automatically detected from imports)
```

## Testing with Capability Constraints

Test your packages under different capability constraints:

```bash
# Test with minimal capabilities
fusabi run --capabilities=stdio src/main.fsx

# Test with full capabilities
fusabi run --capabilities=all src/main.fsx
```

## Best Practices

1. **Minimal Capabilities**: Request only the capabilities you need
2. **Document Permissions**: Explain why each permission is required
3. **Test in Constrained Environments**: Validate behavior with limited capabilities
4. **Graceful Degradation**: Handle missing optional capabilities gracefully
5. **Version Compatibility**: Keep compatible_with updated with tested runtimes
