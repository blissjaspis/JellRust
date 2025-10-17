# JellRust Architecture

This document explains the architecture and design decisions behind JellRust.

## High-Level Overview

```
┌─────────────────────────────────────────────────────────┐
│                      User Input                         │
│              (jellrust new/build/serve)                 │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│                  jellrust-cli                           │
│  • Command parsing (clap)                               │
│  • Command dispatch                                     │
│  • User interaction                                     │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│                 jellrust-core                           │
│  ┌──────────────────────────────────────────────┐      │
│  │  Config Parser (YAML)                        │      │
│  └────────────────┬─────────────────────────────┘      │
│                   │                                     │
│  ┌────────────────▼─────────────────────────────┐      │
│  │  Site Builder (Orchestrator)                 │      │
│  │  • Collect content                           │      │
│  │  • Process files                             │      │
│  │  • Generate URLs                             │      │
│  │  • Coordinate rendering                      │      │
│  └───┬──────────────────────┬──────────────┬────┘      │
└──────┼──────────────────────┼──────────────┼───────────┘
       │                      │              │
       ▼                      ▼              ▼
┌──────────────┐    ┌──────────────┐   ┌──────────────┐
│ jellrust-    │    │ jellrust-    │   │ jellrust-    │
│ markdown     │    │ template     │   │ server       │
│              │    │              │   │              │
│ • Parse MD   │    │ • Liquid     │   │ • Axum HTTP  │
│ • Front      │    │   templates  │   │ • Live       │
│   matter     │    │ • Layouts    │   │   reload     │
│ • Syntax     │    │ • Includes   │   │ • File watch │
│   highlight  │    │ • Variables  │   │ • Static     │
└──────────────┘    └──────────────┘   │   serving    │
                                       └──────────────┘
```

## Crate Structure

### 1. jellrust-cli

**Purpose:** User-facing command-line interface

**Responsibilities:**
- Parse command-line arguments using clap
- Dispatch to appropriate commands
- Handle user input/output
- Initialize logging/tracing

**Key Files:**
- `main.rs` - Entry point, CLI definition
- `commands/new.rs` - Create new site
- `commands/build.rs` - Build site
- `commands/serve.rs` - Development server
- `commands/clean.rs` - Clean output
- `commands/doctor.rs` - Health check

**Dependencies:**
- clap (CLI parsing)
- tokio (async runtime)
- tracing (logging)

### 2. jellrust-core

**Purpose:** Core site generation logic

**Responsibilities:**
- Load and parse configuration
- Walk file system and collect content
- Coordinate between markdown and template processing
- Generate output files
- Manage site structure (pages, posts, static files)

**Key Files:**
- `config.rs` - Configuration management
- `site.rs` - Site builder (main orchestrator)
- `content.rs` - Content models (Page, Post, Site)
- `error.rs` - Error types

**Design Patterns:**
- **Builder Pattern:** `SiteBuilder` accumulates configuration before building
- **Type-Driven Design:** Strong types for Page, Post, Config
- **Error Propagation:** Custom Error enum with `?` operator

### 3. jellrust-markdown

**Purpose:** Markdown parsing and rendering

**Responsibilities:**
- Parse YAML front matter
- Convert Markdown to HTML
- Syntax highlighting for code blocks
- Extract excerpts

**Key Dependencies:**
- pulldown-cmark (Markdown parser)
- syntect (syntax highlighting)
- serde_yaml (front matter)

**Key Functions:**
```rust
parse_front_matter(&str) -> Result<(FrontMatter, &str)>
render(&str) -> Result<String>
```

**Design Notes:**
- Uses pulldown-cmark's event-based parser
- Injects syntax highlighting during event processing
- Stateless design - can be used concurrently

### 4. jellrust-template

**Purpose:** Template rendering with Liquid

**Responsibilities:**
- Load layout files
- Process Liquid templates
- Handle nested layouts
- Provide template variables (site, page, content)
- Include partials

**Key Functions:**
```rust
render_post(post, site, config) -> Result<String>
render_page(page, site, config) -> Result<String>
```

**Template Context:**
```
site:
  - title
  - description
  - posts (array)
  - pages (array)

page:
  - title
  - url
  - date (for posts)
  - tags
  - categories
  
content: (rendered HTML)
```

**Design Notes:**
- Supports nested layouts (layout can have layout)
- Parses front matter in layout files
- Converts Rust structs to Liquid Value types

### 5. jellrust-server

**Purpose:** Development server with live reload

**Responsibilities:**
- Serve static files via HTTP
- Watch source directory for changes
- Inject live reload script
- Handle file change events

**Key Components:**
- Axum web framework
- notify for file watching
- Tower middleware for static files

**Endpoints:**
- `GET /` - Serve site content
- `GET /__reload__` - Live reload status

**Live Reload Flow:**
```
1. File changes detected (notify)
2. Set reload flag
3. Client polls /__reload__
4. Server responds with "reload"
5. Client refreshes page
```

## Data Flow

### Build Process

```
1. Load _config.yml
   ↓
2. Scan source directory
   ↓
3. Collect posts from _posts/
   • Parse front matter
   • Extract date from filename
   • Render Markdown → HTML
   ↓
4. Collect pages (*.md, *.html)
   • Parse front matter
   • Render Markdown → HTML
   ↓
5. Sort posts by date
   ↓
6. For each post/page:
   • Load layout
   • Render with Liquid
   • Write to _site/
   ↓
7. Copy static files (assets/)
   ↓
8. Done!
```

### Serve Process

```
1. Build site (same as above)
   ↓
2. Start file watcher
   ↓
3. Start HTTP server
   ↓
4. On request:
   • Serve file from _site/
   • Inject reload script in HTML
   ↓
5. On file change:
   • Rebuild site
   • Set reload flag
   ↓
6. Client checks reload endpoint
   • Refresh if flag set
```

## Configuration

### _config.yml Structure

```yaml
# Required
title: string

# Optional
description: string
url: string
baseurl: string
markdown: string
permalink: string
paginate: number
exclude: [string]
plugins: [string]

# Custom (any YAML)
author:
  name: string
  email: string
```

### Front Matter Structure

```yaml
---
layout: string
title: string
date: string (ISO 8601)
author: string
categories: [string]
tags: [string]
permalink: string
published: boolean
---
```

## URL Generation

### Posts

Pattern: `/:year/:month/:day/:title/`

Example:
```
_posts/2024-01-15-hello-world.md
→ /2024/01/15/hello-world/index.html
```

Variables:
- `:year` - 4-digit year (2024)
- `:month` - 2-digit month (01)
- `:day` - 2-digit day (15)
- `:title` - Post slug from filename

### Pages

Pages keep their directory structure:

```
about.md → /about.html
projects/index.md → /projects/index.html
```

Override with `permalink` in front matter.

## Template Variables

### Global Variables

**site**
```liquid
{{ site.title }}
{{ site.description }}
{{ site.posts }} - array of all posts
{{ site.pages }} - array of all pages
```

**page**
```liquid
{{ page.title }}
{{ page.url }}
{{ page.date }}
{{ page.categories }}
{{ page.tags }}
```

**content**
```liquid
{{ content }} - rendered HTML of post/page
```

### Filters

Liquid standard filters plus custom:
- `date: "%B %d, %Y"` - Format date
- `relative_url` - Add baseurl
- `absolute_url` - Add url + baseurl

### Loops

```liquid
{% for post in site.posts %}
  <h2>{{ post.title }}</h2>
{% endfor %}
```

## Error Handling

Custom error types with thiserror:

```rust
pub enum Error {
    Io(#[from] std::io::Error),
    Yaml(String),
    Config(String),
    Template(String),
    Markdown(String),
    FileNotFound(String),
    Other(String),
}
```

Error propagation with `?` operator throughout.

User-facing errors from CLI use anyhow for context.

## Performance Considerations

### Current

- Sequential file processing
- Single-threaded rendering
- No caching

### Future Optimizations

1. **Parallel Processing**
   ```rust
   use rayon::prelude::*;
   posts.par_iter().map(|p| process(p)).collect()
   ```

2. **Incremental Builds**
   - Track file timestamps
   - Only rebuild changed files
   - Cache rendered content

3. **Template Compilation**
   - Pre-compile Liquid templates
   - Cache parsed layouts

4. **Asset Pipeline**
   - Minify CSS/JS
   - Optimize images
   - Generate WebP versions

## Testing Strategy

### Unit Tests

Each module has `#[cfg(test)]` section:
- Config parsing
- Front matter parsing
- URL generation
- Markdown rendering

### Integration Tests

In `tests/` directory:
- Build entire site
- Serve and request pages
- Watch for changes

### Benchmarks

With criterion:
- Markdown rendering speed
- Site build time
- Template rendering

## Extension Points

### 1. Plugins

Future plugin system:

```rust
pub trait Plugin {
    fn name(&self) -> &str;
    fn process_post(&self, post: &mut Post) -> Result<()>;
    fn process_page(&self, page: &mut Page) -> Result<()>;
}
```

### 2. Custom Filters

Add Liquid filters:

```rust
template_engine.register_filter("uppercase", |input| {
    input.to_uppercase()
});
```

### 3. Collections

Custom content types beyond posts/pages:

```yaml
collections:
  projects:
    output: true
    permalink: /projects/:path/
```

### 4. Generators

Dynamic page generation:

```rust
pub trait Generator {
    fn generate(&self, site: &Site) -> Vec<Page>;
}
```

## Security Considerations

1. **Path Traversal**
   - Validate all file paths
   - Check for `../` in URLs
   - Canonicalize paths

2. **XSS Prevention**
   - Liquid escapes by default
   - Use `| safe` filter carefully

3. **Configuration Validation**
   - Validate YAML parsing
   - Sanitize user input
   - Limit resource usage

## Production Deployment

### Building

```bash
cargo build --release
```

Produces optimized binary in `target/release/`.

### Distribution

Options:
1. Publish to crates.io: `cargo publish`
2. GitHub releases with binaries
3. Package managers (brew, apt, etc.)

### CI/CD

GitHub Actions workflow:
```yaml
- Run tests
- Run lints (clippy)
- Build release binaries
- Publish to crates.io
```

## Comparison with Jekyll

### Similarities

- Front matter in YAML
- Liquid templates
- Markdown content
- Layout system
- Posts and pages
- Dev server

### Differences

| Feature | Jekyll | JellRust |
|---------|--------|----------|
| Language | Ruby | Rust |
| Speed | Moderate | Very Fast |
| Plugins | Ruby gems | Rust crates (future) |
| Themes | Ruby gems | Local files |
| Dependencies | Ruby + gems | Single binary |

### Advantages of JellRust

1. **Speed** - Rust is much faster than Ruby
2. **Single Binary** - No Ruby/gem installation needed
3. **Memory Safety** - Rust's guarantees
4. **Concurrent** - Easy parallelization

### What's Missing (Future Work)

- Pagination
- Collections
- Data files support
- Sass compilation
- Plugin ecosystem
- Theme system
- RSS/Atom feeds
- Sitemap generation
- SEO optimization

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Code style guide
- Testing requirements
- PR process
- Community guidelines

