# Release Process

This document describes the release process for fusabi-community packages and the registry.

## Versioning Strategy

We follow [Semantic Versioning 2.0.0](https://semver.org/):

- **MAJOR** version: Incompatible API changes
- **MINOR** version: Backwards-compatible functionality additions
- **PATCH** version: Backwards-compatible bug fixes

### Pre-release Versions

- **Alpha**: `0.1.0-alpha.1` - Early development, unstable
- **Beta**: `0.1.0-beta.1` - Feature complete, testing
- **RC**: `0.1.0-rc.1` - Release candidate, final testing

## Release Workflow

### Automated Release Process

Releases are managed through GitHub Actions. To create a release:

1. **Update Version Numbers**
   ```bash
   # Update package versions in fusabi.toml files
   # Update registry/index.toml
   # Update CHANGELOG entries
   ```

2. **Create Release Tag**
   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```

3. **Automated Steps** (handled by `.github/workflows/release.yml`)
   - Validates all package metadata
   - Runs test suite
   - Validates capability schemas
   - Builds packages
   - Generates changelog from commits
   - Creates GitHub release
   - Publishes to crates.io (for Rust components)
   - Updates registry index

### Manual Release Checklist

Before creating a release tag:

- [ ] All tests passing on main branch
- [ ] CHANGELOG.md updated with release notes
- [ ] Version numbers bumped in all `fusabi.toml` files
- [ ] Registry index updated with new versions
- [ ] Documentation updated in `docs/versions/vNEXT/`
- [ ] Examples tested and working
- [ ] No open critical bugs
- [ ] Security audit completed (for major releases)

## Package Versioning

### Individual Package Releases

Packages can be versioned independently:

```toml
# packages/json/fusabi.toml
[package]
name = "json"
version = "0.2.0"  # Package version

# packages/commander/fusabi.toml
[package]
name = "commander"
version = "0.1.5"  # Different version
```

Update the registry index to reflect new package versions:

```toml
# registry/index.toml
[packages.json]
version = "0.2.0"
description = "JSON parsing and serialization combinators"
path = "packages/json"
license = "MIT"
changelog = "https://github.com/fusabi-lang/fusabi-community/blob/main/packages/json/CHANGELOG.md"
```

### Registry Releases

The registry itself is versioned separately from individual packages. Registry versions indicate:

- Schema changes to the index format
- Addition/removal of package categories
- Changes to capability metadata format

## Release Branches

### Branch Protection

The `main` branch is protected with the following rules:

- Require pull request reviews (minimum 1 approval)
- Require status checks to pass (CI tests)
- Require conversation resolution before merging
- No force pushes allowed
- No deletions allowed

### Creating a Release Branch

For major releases, create a release branch:

```bash
git checkout -b release/v1.0.0
# Make release-specific changes
# Update versions, changelog, docs
git push origin release/v1.0.0
```

Create a PR from the release branch to `main` for review.

## Changelog Generation

Changelogs are auto-generated from commit messages. Use conventional commits:

```
feat: Add new JSON path traversal API
fix: Correct commander file deletion bug
docs: Update capability metadata examples
chore: Update CI dependencies
test: Add integration tests for registry
```

### Conventional Commit Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Test additions/changes
- `chore`: Maintenance tasks
- `ci`: CI/CD changes

## Publishing to crates.io

For Rust-based packages or tools:

1. Ensure `Cargo.toml` versions match `fusabi.toml`
2. Run `cargo publish --dry-run` to validate
3. GitHub Actions will publish on tag push

## Post-Release Steps

After a successful release:

1. **Create vNEXT Documentation**
   ```bash
   cp -r docs/versions/vNEXT docs/versions/v0.1.0
   # Update version references in v0.1.0/
   # Create fresh vNEXT for next cycle
   ```

2. **Announce Release**
   - Post to GitHub Discussions
   - Update project README
   - Notify community channels

3. **Monitor for Issues**
   - Watch issue tracker for bug reports
   - Prepare patch release if critical bugs found

## Hotfix Process

For critical bugs in production:

1. Create hotfix branch from release tag:
   ```bash
   git checkout -b hotfix/v0.1.1 v0.1.0
   ```

2. Fix the issue and update version to patch level

3. Create PR and fast-track review

4. Tag and release:
   ```bash
   git tag -a v0.1.1 -m "Hotfix: Critical bug in JSON parser"
   git push origin v0.1.1
   ```

## Rollback Procedure

If a release has critical issues:

1. **Yank the release** (for crates.io packages):
   ```bash
   cargo yank --vers 0.1.0 <package-name>
   ```

2. **Mark as pre-release** on GitHub:
   - Edit the GitHub release
   - Check "This is a pre-release"

3. **Update registry index**:
   - Revert to previous version
   - Add deprecation notice

4. **Communicate**:
   - Post issue describing the problem
   - Notify users via release notes

## Version Support Policy

- **Latest major version**: Full support with bug fixes and features
- **Previous major version**: Security fixes only for 6 months
- **Older versions**: Community support only

## CODEOWNERS

Release-critical files require review from designated owners. See `.github/CODEOWNERS`.

## Questions?

For questions about the release process, open a GitHub Discussion or contact the maintainers.
