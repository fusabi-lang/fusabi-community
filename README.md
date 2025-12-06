# Fusabi Community

Community-maintained packages, libraries, and tools for the Fusabi programming language.

## Overview

This repository serves as the central registry and distribution point for community-contributed Fusabi packages. It provides high-quality, reusable components organized into curated categories.

## Quick Start

### Browse Packages

Explore available packages in the [registry index](./registry/index.toml) or browse by category:

- **Terminal UI**: TUI components and applications ([commander](./packages/commander/))
- **Utilities**: Core utilities and helpers ([json](./packages/json/))
- **Observability**: Logging, tracing, and monitoring (coming soon)
- **K8s/Cloud**: Cloud infrastructure and Kubernetes integrations (coming soon)
- **MCP/AI**: Model Context Protocol and AI integrations (coming soon)

### Install a Package

```bash
# Using the Fusabi package manager (future)
fus add json

# Or clone and use directly
git clone https://github.com/fusabi-lang/fusabi-community.git
cd fusabi-community/packages/json
fus run src/lib.fsx
```

### Example Usage

```fusabi
// Using the JSON package
use json::*;

let data = Json.parse("{\"name\": \"Alice\", \"age\": 30}");
match data {
  Ok(value) => {
    let name = Json.getString(value, "name");
    print("Hello, " + name);
  }
  Err(e) => print("Error: " + e)
}
```

## Available Packages

### json (v0.1.0)

JSON parsing and serialization combinators built on Fusabi's standard library.

- Pure library with no runtime dependencies
- Type-safe value extraction
- Path traversal support

[Documentation](./packages/json/README.md) | [Examples](./packages/json/examples/)

### commander (v0.1.0)

A powerful TUI file manager for terminal-based file navigation and management.

- Interactive file browser
- File operations (view, edit, delete, copy, move)
- Keyboard-driven interface

[Documentation](./packages/commander/README.md) | [Examples](./packages/commander/examples/)

## Documentation

- **Latest (vNEXT)**: [docs/versions/vNEXT](./docs/versions/vNEXT/README.md)
- **Structure Guide**: [docs/STRUCTURE.md](./docs/STRUCTURE.md)
- **Release Process**: [docs/RELEASE.md](./docs/RELEASE.md)
- **Capabilities**: [docs/versions/vNEXT/CAPABILITIES.md](./docs/versions/vNEXT/CAPABILITIES.md)

## Package Development

### Creating a New Package

1. Create package directory:
   ```bash
   mkdir -p packages/my-package/src
   ```

2. Create `fusabi.toml`:
   ```toml
   [package]
   name = "my-package"
   version = "0.1.0"
   description = "My awesome package"
   authors = ["Your Name"]
   license = "MIT"

   [capabilities]
   requires = ["stdio"]  # List required capabilities
   compatible_with = ["scarab", "hibana", "fusabi-plugin-runtime"]
   ```

3. Implement your package in `src/lib.fsx` or `src/main.fsx`

4. Add tests and examples

5. Update registry index in `registry/index.toml`

6. Submit a pull request

### Package Requirements

All packages must include:

- [ ] `fusabi.toml` with complete metadata
- [ ] `README.md` with usage documentation
- [ ] `examples/` directory with working examples
- [ ] Capability declarations for runtime compatibility
- [ ] Tests (when applicable)
- [ ] License information

### Integration with fusabi-stdlib-ext

Packages should leverage [fusabi-stdlib-ext](https://github.com/fusabi-lang/fusabi-stdlib-ext) modules for enhanced functionality:

```toml
[dependencies]
fusabi-stdlib-ext = { version = "0.1" }
```

This enables better integration with plugin-runtime hosts like scarab and hibana.

## Contributing

We welcome contributions! Here's how to get started:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/my-package`
3. **Develop your package** following our guidelines
4. **Test thoroughly** with different runtime environments
5. **Submit a pull request**

### Contribution Guidelines

- Follow the [Fusabi style guide](https://github.com/fusabi-lang/fusabi/blob/main/STYLE.md)
- Add comprehensive documentation
- Include examples demonstrating key features
- Declare capabilities accurately
- Test with scarab and hibana when applicable
- Keep dependencies minimal

### Code Review Process

All submissions require:

- Passing CI checks (tests, linting, capability validation)
- Documentation review
- Code review from maintainers (see [CODEOWNERS](./.github/CODEOWNERS))
- Security audit for packages with network/filesystem access

## Registry

The package registry is maintained in [registry/index.toml](./registry/index.toml). It provides:

- Semantic versioning for all packages
- Capability metadata for runtime compatibility
- Download URLs and checksums
- Category and tag organization

### Registry Schema

```toml
[packages.<name>]
version = "0.1.0"
description = "Package description"
path = "packages/<name>"
license = "MIT"
category = "utilities"  # terminal-ui, observability, k8s-cloud, mcp-ai, utilities
requires_capabilities = ["stdio", "filesystem"]
compatible_runtimes = ["scarab", "hibana", "fusabi-plugin-runtime"]
```

## Versioning

We follow [Semantic Versioning 2.0.0](https://semver.org/):

- **MAJOR**: Incompatible API changes
- **MINOR**: Backwards-compatible features
- **PATCH**: Backwards-compatible fixes

Packages are versioned independently. The registry maintains the latest stable version of each package.

## Release Process

Releases are automated via GitHub Actions:

1. Update package versions in `fusabi.toml`
2. Update `registry/index.toml`
3. Create a git tag: `git tag -a v0.1.0 -m "Release v0.1.0"`
4. Push tag: `git push origin v0.1.0`
5. GitHub Actions handles the rest (building, packaging, publishing)

See [docs/RELEASE.md](./docs/RELEASE.md) for detailed release procedures.

## Testing

### Run Package Tests

```bash
# Test all packages
make test

# Test specific package
fus test packages/json/src/lib.fsx
```

### Validate Capabilities

```bash
# Run with capability constraints
fus run --capabilities=stdio,filesystem packages/commander/src/main.fsx
```

### CI Pipeline

Our CI validates:

- Package metadata and TOML syntax
- Capability schema compliance
- Documentation completeness
- Code compilation
- Test suite execution

## Support

- **Issues**: [GitHub Issues](https://github.com/fusabi-lang/fusabi-community/issues)
- **Discussions**: [GitHub Discussions](https://github.com/fusabi-lang/fusabi-community/discussions)
- **Chat**: Join our community chat (link TBD)

## License

This repository and all packages (unless otherwise specified) are licensed under the MIT License. See [LICENSE](./LICENSE) for details.

Individual packages may have different licenses - check each package's README for specific licensing information.

## Acknowledgments

Built with love by the Fusabi community. Special thanks to all contributors!

## Related Projects

- [Fusabi Language](https://github.com/fusabi-lang/fusabi) - The Fusabi programming language
- [fusabi-stdlib-ext](https://github.com/fusabi-lang/fusabi-stdlib-ext) - Extended standard library
- [fusabi-plugin-runtime](https://github.com/fusabi-lang/fusabi-plugin-runtime) - Plugin runtime system
- [scarab](https://github.com/fusabi-lang/scarab) - Fusabi runtime environment
- [hibana](https://github.com/fusabi-lang/hibana) - Alternative runtime

## Roadmap

Upcoming packages and features:

- [ ] Observability pack (logging, tracing, metrics)
- [ ] K8s/Cloud pack (kubectl bindings, cloud SDKs)
- [ ] MCP/AI pack (LLM integrations, Model Context Protocol)
- [ ] HTTP client and server libraries
- [ ] Database connectors
- [ ] Testing framework
- [ ] Package manager integration

## Status

This repository is under active development. Package APIs may change before v1.0.0 release. We aim for stability while exploring the best patterns for the Fusabi ecosystem.
