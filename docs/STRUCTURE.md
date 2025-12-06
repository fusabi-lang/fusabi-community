# Documentation Structure

This document describes the organization and required sections for fusabi-community documentation.

## Directory Layout

```
docs/
├── STRUCTURE.md          # This file - describes documentation organization
├── RELEASE.md            # Release process and versioning guidelines
└── versions/             # Versioned documentation snapshots
    ├── vNEXT/            # Current development version
    │   ├── README.md     # Overview and quick start
    │   ├── PACKAGES.md   # Package catalog and documentation
    │   └── CAPABILITIES.md  # Capability metadata specification
    ├── v0.1.0/           # Released version documentation (future)
    └── v0.2.0/           # Released version documentation (future)
```

## Required Documentation Sections

### Repository Root

- **README.md**: Project overview, quick start, contribution guidelines
- **LICENSE**: Repository license (MIT)

### docs/

- **STRUCTURE.md**: This file describing documentation organization
- **RELEASE.md**: Release process, versioning strategy, and guidelines

### docs/versions/vNEXT/

The `vNEXT` directory contains documentation for the current development version. It becomes a versioned snapshot at release time.

Required files:
- **README.md**: Version overview, quick start, package categories
- **PACKAGES.md**: Detailed package catalog with examples
- **CAPABILITIES.md**: Capability metadata schema and examples

### Package Documentation

Each package in `packages/<name>/` must include:

- **README.md**: Package overview, features, installation, usage
- **examples/**: Directory with example code
- **fusabi.toml**: Package metadata including capability declarations

## Documentation Standards

### Markdown Formatting

- Use ATX-style headers (`#` syntax)
- Include code fences with language tags
- Use relative links for internal references
- Keep lines under 120 characters when possible

### Code Examples

All code examples must:
- Be runnable or clearly marked as pseudocode
- Include necessary imports
- Show expected output when relevant
- Follow Fusabi style guidelines

### Capability Documentation

When documenting capabilities:
- Explain the purpose of each capability
- Show minimal and complete examples
- Document permission requirements
- Include runtime compatibility matrix

## Version Management

### Creating a New Version

When cutting a release:

1. Copy `docs/versions/vNEXT` to `docs/versions/v<version>`
2. Update version references in the new directory
3. Create fresh `vNEXT` for next development cycle
4. Update main README.md to point to latest stable version

### Documentation Review Checklist

- [ ] All required files present
- [ ] Code examples tested
- [ ] Links validated (no 404s)
- [ ] Version numbers consistent
- [ ] Capability schemas validated
- [ ] No AI prompts or development notes
- [ ] Grammar and spelling checked

## CI Validation

Documentation is validated in CI via `.github/workflows/docs-check.yml`:

- Validates all required files exist
- Checks markdown formatting
- Validates code examples compile
- Verifies capability schemas
- Ensures no broken links

## Contribution Guidelines

When adding documentation:

1. Place new guides in `docs/versions/vNEXT/`
2. Update PACKAGES.md when adding new packages
3. Run `make docs-check` locally before committing
4. Keep documentation synchronized with code

## Archive Policy

- **No archive/ directories**: Old content is preserved in git history
- **No AI prompts**: Remove scaffolding and prompts before committing
- **No development notes**: Keep only distilled, user-facing documentation
- **Legacy content**: Delete or migrate to current documentation structure
