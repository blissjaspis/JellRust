# Learning Rust Through JellRust

This guide will walk you through learning Rust by understanding and building JellRust, a static site generator inspired by Jekyll.

## Prerequisites

- Basic programming knowledge
- Familiarity with command-line tools
- Text editor or IDE (VS Code with rust-analyzer recommended)

## Phase 1: Rust Fundamentals (Week 1-2)

### Day 1-3: Basic Syntax & Ownership

**Concepts to Learn:**
- Variables and mutability
- Data types (scalars, tuples, arrays)
- Functions
- Ownership rules
- Borrowing and references
- Slices

**In JellRust:**
- Look at `jellrust-core/src/config.rs`
- Notice how we use `&str` vs `String`
- See borrowing in action: `pub fn load<P: AsRef<Path>>(source_dir: P)`

**Exercises:**
1. Modify `Config::default()` to add a new field
2. Create a function that borrows a string and returns its length
3. Understand why we use `&self` vs `self` in methods

### Day 4-7: Error Handling & Structs

**Concepts to Learn:**
- Enums and pattern matching
- Result and Option types
- Custom error types with `thiserror`
- Structs and impl blocks
- Methods vs associated functions

**In JellRust:**
- Study `jellrust-core/src/error.rs`
- See how errors are propagated with `?`
- Look at structs in `jellrust-core/src/content.rs`

**Exercises:**
1. Add a new error variant to the `Error` enum
2. Create a method that returns `Result<T, Error>`
3. Practice pattern matching on Option and Result

## Phase 2: Intermediate Concepts (Week 3-4)

### Day 8-10: Collections & Iterators

**Concepts to Learn:**
- Vec, HashMap, HashSet
- Iterator trait and methods
- Closures
- The `?` operator

**In JellRust:**
- See HashMap usage in `Config::custom`
- Iterator chains in `site.rs` sorting posts
- `walkdir` crate for file system iteration

**Exercises:**
1. Filter posts by category using iterators
2. Create a HashMap of tags to posts
3. Use `map()` and `filter()` on collections

### Day 11-14: Traits & Generics

**Concepts to Learn:**
- Defining and implementing traits
- Generic functions and structs
- Trait bounds
- Derive macros

**In JellRust:**
- Look at `Serialize` and `Deserialize` derives
- Generic `load<P: AsRef<Path>>` function
- Trait implementations like `From` and `Default`

**Exercises:**
1. Create a custom trait for content types
2. Implement `Display` for a custom type
3. Use generic type parameters in a function

## Phase 3: Advanced Features (Week 5-6)

### Day 15-18: Modules & Workspace

**Concepts to Learn:**
- Module system (mod, pub, use)
- Cargo workspaces
- Crate organization
- Visibility rules

**In JellRust:**
- See workspace structure in root `Cargo.toml`
- Module organization in each crate
- Public API design in `lib.rs` files

**Exercises:**
1. Add a new module to `jellrust-core`
2. Create a new crate in the workspace
3. Re-export items from submodules

### Day 19-21: Async Programming

**Concepts to Learn:**
- async/await syntax
- Tokio runtime
- Futures and tasks
- Async file I/O

**In JellRust:**
- Study `jellrust-server/src/lib.rs`
- See async handlers in the dev server
- Tokio setup in `main.rs`

**Exercises:**
1. Create an async function that reads a file
2. Use `tokio::spawn` to run tasks concurrently
3. Understand when to use async vs sync

## Phase 4: Real-World Skills (Week 7-8)

### Day 22-25: CLI Tools

**Concepts to Learn:**
- clap for argument parsing
- Subcommands and arguments
- Environment variables
- User interaction

**In JellRust:**
- Study `jellrust-cli/src/main.rs`
- See clap derives in action
- Command structure in `commands/` module

**Exercises:**
1. Add a new CLI command
2. Add command-line flags
3. Improve help messages

### Day 26-28: File System Operations

**Concepts to Learn:**
- std::fs module
- Path and PathBuf
- Walking directories
- File watching with notify

**In JellRust:**
- File operations in `site.rs`
- Path manipulation in URL generation
- Directory watching in `commands/build.rs`

**Exercises:**
1. Copy a directory recursively
2. Find all markdown files in a tree
3. Watch a directory for changes

### Day 29-31: Serialization & Parsing

**Concepts to Learn:**
- Serde framework
- YAML, JSON, TOML parsing
- Custom deserializers
- Working with external data

**In JellRust:**
- Front matter parsing in `jellrust-markdown`
- Config loading with serde_yaml
- Liquid template data in `jellrust-template`

**Exercises:**
1. Parse a YAML file into a struct
2. Serialize data to JSON
3. Handle optional fields gracefully

## Phase 5: Testing & Quality (Week 9-10)

### Day 32-35: Testing

**Concepts to Learn:**
- Unit tests with `#[test]`
- Integration tests
- Test organization
- Assertions and panics

**In JellRust:**
- See tests in `config.rs`, `content.rs`
- Run tests with `cargo test`
- Add test coverage

**Exercises:**
1. Write tests for markdown parsing
2. Create integration test for site building
3. Use `cargo test --doc` for doctests

### Day 36-40: Documentation

**Concepts to Learn:**
- Doc comments (`///`)
- rustdoc
- Code examples in docs
- README writing

**In JellRust:**
- Add doc comments to public functions
- Generate docs with `cargo doc --open`
- Write examples

**Exercises:**
1. Document all public APIs
2. Add code examples that compile
3. Create a user guide

## Phase 6: Performance & Production (Week 11-12)

### Day 41-45: Performance

**Concepts to Learn:**
- Benchmarking with criterion
- Profiling tools
- Rayon for parallelism
- Memory efficiency

**In JellRust:**
- Parallel file processing possibilities
- Benchmark markdown rendering
- Profile site building

**Exercises:**
1. Write a benchmark for markdown parsing
2. Parallelize post processing
3. Optimize hot paths

### Day 46-50: Production Ready

**Concepts to Learn:**
- Error messages
- Logging with tracing
- Configuration management
- Release optimization

**In JellRust:**
- Tracing setup in CLI
- Release builds with `--release`
- Error context with anyhow

**Exercises:**
1. Add structured logging
2. Improve error messages
3. Optimize binary size

## Key Rust Patterns in JellRust

### 1. Builder Pattern
```rust
let mut builder = SiteBuilder::new(source, dest, config);
builder.set_include_drafts(true);
builder.build().await?;
```

### 2. Error Propagation
```rust
pub fn load(path: &Path) -> Result<Config> {
    let content = fs::read_to_string(path)?;
    let config = serde_yaml::from_str(&content)?;
    Ok(config)
}
```

### 3. Generic Trait Bounds
```rust
pub fn load<P: AsRef<Path>>(source_dir: P) -> Result<Self> {
    // Works with &str, String, Path, PathBuf, etc.
}
```

### 4. Iterator Chains
```rust
site.posts
    .iter()
    .filter(|p| p.front_matter.published)
    .take(10)
    .collect()
```

### 5. Type-Driven Design
```rust
pub enum Error {
    Io(#[from] std::io::Error),
    Yaml(String),
    Config(String),
}
```

## Common Rust Gotchas

1. **Ownership vs Borrowing**
   - Use `&` for borrowing when you don't need ownership
   - Use `clone()` sparingly, only when necessary

2. **String vs &str**
   - `String` is owned, heap-allocated
   - `&str` is a borrowed slice, often from String or literals

3. **Error Handling**
   - Use `?` to propagate errors
   - Match on Result when you need custom handling

4. **Async Runtime**
   - Need `#[tokio::main]` for async main
   - Can't call async from sync without spawning

5. **Lifetime Elision**
   - Often lifetime annotations are inferred
   - Explicit when needed: `fn foo<'a>(x: &'a str) -> &'a str`

## Resources

### Official Documentation
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Book](https://rust-lang.github.io/async-book/)

### Video Courses
- [Rust Programming Course (freeCodeCamp)](https://www.youtube.com/watch?v=MsocPEZBd-M)
- [Crust of Rust series by Jon Gjengset](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa)

### Practice
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises
- [Exercism Rust Track](https://exercism.org/tracks/rust)

## Next Steps

After completing this learning path:

1. **Extend JellRust**
   - Add SCSS compilation
   - Implement pagination
   - Create a plugin system
   - Add RSS feed generation

2. **Build Your Own Project**
   - CLI tool
   - Web API with Axum
   - Game with Bevy
   - System utility

3. **Contribute to Open Source**
   - Find Rust projects on GitHub
   - Start with "good first issue" labels
   - Join Rust Discord/Forums

4. **Deep Dive Topics**
   - Unsafe Rust
   - Macros (declarative and procedural)
   - FFI (calling C from Rust)
   - WebAssembly

## Getting Help

- [Rust Users Forum](https://users.rust-lang.org/)
- [Rust Discord](https://discord.gg/rust-lang)
- [r/rust subreddit](https://www.reddit.com/r/rust/)
- Stack Overflow with `[rust]` tag

Happy learning! 🦀

