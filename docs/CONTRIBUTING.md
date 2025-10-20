# Contributing to JellRust

Thank you for your interest in contributing to JellRust! This document provides guidelines and instructions for contributing.

## Getting Started

### Prerequisites

- Rust 1.75 or higher
- Git
- Basic understanding of static site generators

### Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/jellrust.git
   cd jellrust
   ```

2. **Build the project**
   ```bash
   cargo build
   ```

3. **Run tests**
   ```bash
   cargo test
   ```

4. **Run the CLI**
   ```bash
   cargo run -- new test-site
   cd test-site
   cargo run -- serve
   ```

## Code Style

### Rust Style Guide

We follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/). Key points:

- Use `rustfmt` for formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Maximum line length: 100 characters
- Use meaningful variable names
- Document public APIs with doc comments

### Example

```rust
/// Loads configuration from a YAML file.
///
/// # Arguments
///
/// * `path` - Path to the configuration file
///
/// # Returns
///
/// Returns `Ok(Config)` on success, or an error if the file cannot be read or parsed.
///
/// # Example
///
/// ```
/// use jellrust_core::config::Config;
/// use std::path::Path;
///
/// let config = Config::load(Path::new("_config.yml"))?;
/// ```
pub fn load<P: AsRef<Path>>(path: P) -> Result<Config> {
    // Implementation
}
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p jellrust-core

# Run a specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Writing Tests

#### Unit Tests

Place unit tests in the same file as the code, in a `tests` module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        let result = my_function();
        assert_eq!(result, expected);
    }
}
```

#### Integration Tests

Place integration tests in the `tests/` directory:

```rust
// tests/integration_test.rs
use jellrust_core::config::Config;

#[test]
fn test_full_build() {
    // Test complete workflow
}
```

### Test Coverage

We aim for at least 70% test coverage. Use `tarpaulin` to check:

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## Commit Messages

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style (formatting, missing semicolons, etc.)
- `refactor`: Code restructuring without changing behavior
- `perf`: Performance improvement
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples

```
feat(markdown): add syntax highlighting for code blocks

Implements syntax highlighting using syntect library.
Supports all common programming languages.

Closes #123
```

```
fix(server): resolve live reload connection issue

The WebSocket connection was not properly handling reconnects.
Now includes exponential backoff retry logic.
```

## Pull Request Process

1. **Create a branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Write code
   - Add tests
   - Update documentation

3. **Ensure quality**
   ```bash
   cargo fmt
   cargo clippy
   cargo test
   ```

4. **Commit your changes**
   ```bash
   git add .
   git commit -m "feat: add awesome feature"
   ```

5. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create Pull Request**
   - Go to GitHub
   - Click "New Pull Request"
   - Fill out the template
   - Link related issues

### PR Checklist

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] New tests added for new functionality
- [ ] Documentation updated
- [ ] CHANGELOG.md updated (for significant changes)
- [ ] Code formatted with `rustfmt`
- [ ] No `clippy` warnings

## Issue Guidelines

### Bug Reports

Include:
- JellRust version (`jellrust --version`)
- Operating system and version
- Rust version (`rustc --version`)
- Steps to reproduce
- Expected behavior
- Actual behavior
- Error messages or logs

### Feature Requests

Include:
- Clear description of the feature
- Use cases and examples
- Why it would be useful
- Potential implementation ideas

### Issue Labels

- `bug` - Something isn't working
- `enhancement` - New feature request
- `documentation` - Documentation improvements
- `good first issue` - Good for newcomers
- `help wanted` - Extra attention needed
- `performance` - Performance related
- `question` - Further information requested

## Development Workflow

### Adding a New Feature

1. **Discuss first** - Open an issue to discuss the feature
2. **Design** - Plan the implementation
3. **Implement** - Write the code
4. **Test** - Add comprehensive tests
5. **Document** - Update docs and examples
6. **Submit PR** - Create pull request

### Fixing a Bug

1. **Reproduce** - Verify you can reproduce the bug
2. **Write test** - Create a failing test
3. **Fix** - Implement the fix
4. **Verify** - Ensure test now passes
5. **Submit PR** - Create pull request

## Project Structure

```
jellrust/
├── jellrust-cli/       # Command-line interface
├── jellrust-core/      # Core site generation
├── jellrust-markdown/  # Markdown processing
├── jellrust-template/  # Template rendering
├── jellrust-server/    # Dev server
├── docs/              # Documentation
└── tests/             # Integration tests
```

## Adding a New Crate

If you need to add a new crate to the workspace:

1. Create the directory: `mkdir jellrust-newcrate`
2. Add to workspace in root `Cargo.toml`:
   ```toml
   [workspace]
   members = [
       "jellrust-cli",
       # ... existing crates
       "jellrust-newcrate",
   ]
   ```
3. Create `jellrust-newcrate/Cargo.toml`
4. Document the purpose and API

## Documentation

### Doc Comments

Use `///` for public APIs:

```rust
/// Renders Markdown to HTML.
///
/// # Arguments
///
/// * `markdown` - The Markdown source text
///
/// # Returns
///
/// Returns the rendered HTML as a `String`.
///
/// # Errors
///
/// Returns an error if the Markdown cannot be parsed.
pub fn render(&self, markdown: &str) -> Result<String> {
    // ...
}
```

### README Updates

Update READMEs when:
- Adding new features
- Changing CLI commands
- Updating installation instructions

### Architecture Documentation

Update `ARCHITECTURE.md` when:
- Adding new crates
- Changing major design patterns
- Adding new extension points

## Performance

### Benchmarking

Use Criterion for benchmarks:

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_render(c: &mut Criterion) {
    c.bench_function("render markdown", |b| {
        b.iter(|| {
            // Code to benchmark
        });
    });
}

criterion_group!(benches, benchmark_render);
criterion_main!(benches);
```

Run benchmarks:
```bash
cargo bench
```

### Profiling

For CPU profiling:
```bash
cargo install flamegraph
cargo flamegraph --bin jellrust
```

## Security

### Reporting Vulnerabilities

**Do not** open public issues for security vulnerabilities.

Instead:
1. Email security@jellrust.dev (or use GitHub Security Advisories)
2. Include detailed description
3. Wait for response before public disclosure

### Security Checklist

- Validate all user input
- Use safe defaults
- Avoid `unsafe` unless necessary
- Check dependencies for known vulnerabilities: `cargo audit`

## Release Process

(For maintainers)

1. Update version in all `Cargo.toml` files
2. Update `CHANGELOG.md`
3. Create git tag: `git tag -a v0.1.0 -m "Release v0.1.0"`
4. Push tag: `git push origin v0.1.0`
5. Publish to crates.io: `cargo publish`
6. Create GitHub release

## Community

### Code of Conduct

We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

Key points:
- Be respectful and inclusive
- Welcome newcomers
- Assume good faith
- Be patient with questions

### Communication Channels

- GitHub Issues - Bug reports and features
- GitHub Discussions - General questions
- Discord - Real-time chat (coming soon)

## Getting Help

- Check the [README](README.md)
- Read the [Learning Path](LEARNING_PATH.md)
- Review [Architecture](ARCHITECTURE.md)
- Ask in GitHub Discussions
- Search existing issues

## Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Credited in documentation

Thank you for contributing to JellRust! 🦀

