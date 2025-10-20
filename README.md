# JellRust 🦀

A blazingly fast static site generator written in Rust, inspired by Jekyll.

## What is JellRust?

JellRust is a modern static site generator that transforms your Markdown content and templates into a complete static website. Perfect for blogs, documentation sites, and portfolios.

## Features

- ⚡ **Fast** - Written in Rust for maximum performance
- 📝 **Markdown Support** - Write content in Markdown with YAML front matter
- 🎨 **Powerful Templates** - Liquid-compatible templating engine
- 🔥 **Live Reload** - Instant preview with built-in dev server
- 🎯 **Plugin System** - Extensible architecture for custom functionality
- 📦 **Asset Processing** - Built-in SCSS compilation and syntax highlighting
- 🌐 **GitHub Pages Ready** - Deploy anywhere, optimized for GitHub Pages

## Quick Start

### Installation

```bash
cargo install jellrust
```

### Create a New Site

```bash
jellrust new my-blog
cd my-blog
```

### Start Development Server

```bash
jellrust serve
```

Visit `http://localhost:4000` to see your site!

### Build for Production

```bash
jellrust build
```

## Project Structure

```
my-blog/
├── _config.yml          # Site configuration
├── _layouts/            # Page layouts
│   ├── default.html
│   └── post.html
├── _includes/           # Reusable components
│   ├── header.html
│   └── footer.html
├── _posts/              # Blog posts
│   └── 2024-01-01-hello-world.md
├── _drafts/             # Unpublished posts
├── _data/               # Data files (YAML, JSON)
├── assets/              # CSS, JS, images
│   ├── css/
│   ├── js/
│   └── images/
├── _site/               # Generated static site (output)
└── index.md             # Homepage
```

## Configuration (_config.yml)

```yaml
title: My Awesome Blog
description: A blog about technology and life
url: https://myblog.com
baseurl: ""
author:
  name: Your Name
  email: your.email@example.com

# Build settings
markdown: pulldown-cmark
theme: minima
plugins:
  - syntax-highlighting
  - sitemap
  - seo

# Collections
collections:
  projects:
    output: true
    permalink: /projects/:path/

# Pagination
paginate: 10
paginate_path: "/blog/page:num/"
```

## Front Matter Example

```markdown
---
layout: post
title: "My First Post"
date: 2024-01-01 10:00:00 +0000
categories: [rust, programming]
tags: [tutorial, beginners]
author: John Doe
---

# Hello World

Your content goes here...
```

## Commands

### `jellrust new <name>`
Create a new JellRust site

```bash
jellrust new my-site
```

### `jellrust build`
Build your site to `_site/` directory

```bash
jellrust build
jellrust build --drafts  # Include draft posts
jellrust build --watch   # Rebuild on file changes
```

### `jellrust serve`
Start development server with live reload

```bash
jellrust serve
jellrust serve --port 3000
jellrust serve --host 0.0.0.0
```

### `jellrust clean`
Remove generated `_site/` directory

```bash
jellrust clean
```

## Roadmap

### Phase 1: Core Features ✅
- [x] Basic CLI structure
- [ ] Configuration parser
- [ ] Markdown rendering
- [ ] Front matter parsing
- [ ] Template engine
- [ ] Site generator

### Phase 2: Advanced Features
- [ ] Live reload server
- [ ] Asset processing (SCSS)
- [ ] Syntax highlighting
- [ ] Pagination
- [ ] Collections
- [ ] Data files

### Phase 3: Plugins & Ecosystem
- [ ] Plugin system
- [ ] SEO plugin
- [ ] Sitemap generator
- [ ] RSS feed generator
- [ ] Theme support

### Phase 4: Production Ready
- [ ] Performance optimization
- [ ] Comprehensive testing
- [ ] Full documentation
- [ ] GitHub Pages integration

## Learning Rust Through This Project

This project covers essential Rust concepts:

1. **Ownership & Borrowing** - File handling and string manipulation
2. **Error Handling** - Using Result and custom error types
3. **Traits** - Plugin system and extensibility
4. **Async Programming** - Dev server with Tokio
5. **Concurrency** - Parallel file processing with Rayon
6. **Testing** - Unit and integration tests
7. **CLI Tools** - Building user-friendly command-line interfaces

## Architecture

```
┌─────────────────┐
│   CLI (clap)    │
└────────┬────────┘
         │
    ┌────▼────────────────┐
    │   JellRust Core     │
    │  - Config Parser    │
    │  - Site Generator   │
    │  - File Watcher     │
    └──┬──┬──┬──┬────────┘
       │  │  │  │
   ┌───▼  │  │  └──────┐
   │      │  │         │
┌──▼───┐ │  │   ┌─────▼─────┐
│ MD   │ │  │   │  Template │
│Parser│ │  │   │  Engine   │
└──────┘ │  │   └───────────┘
         │  │
    ┌────▼──▼────┐
    │ Dev Server │
    │ (Hot Reload)│
    └────────────┘
```

## Contributing

Contributions are welcome! Please read our [Contributing Guide](docs/CONTRIBUTING.md) first.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Credits

Inspired by Jekyll and built with ❤️ in Rust.

