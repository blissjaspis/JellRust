use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn execute(name: String, path: Option<PathBuf>) -> Result<()> {
    let site_path = path.unwrap_or_else(|| PathBuf::from(&name));
    
    tracing::info!("Creating new JellRust site: {}", name);
    tracing::info!("Destination: {}", site_path.display());
    
    // Create directory structure
    fs::create_dir_all(&site_path)
        .context("Failed to create site directory")?;
    
    create_directory_structure(&site_path)?;
    create_default_files(&site_path, &name)?;
    
    println!("\n✅ New JellRust site created successfully!");
    println!("📁 Location: {}", site_path.display());
    println!("\n🚀 Next steps:");
    println!("   cd {}", name);
    println!("   jellrust serve");
    
    Ok(())
}

fn create_directory_structure(base: &PathBuf) -> Result<()> {
    let dirs = [
        "_layouts",
        "_includes",
        "_posts",
        "_drafts",
        "_data",
        "assets/css",
        "assets/js",
        "assets/images",
    ];
    
    for dir in dirs {
        let path = base.join(dir);
        fs::create_dir_all(&path)
            .with_context(|| format!("Failed to create directory: {}", path.display()))?;
        tracing::debug!("Created directory: {}", path.display());
    }
    
    Ok(())
}

fn create_default_files(base: &PathBuf, site_name: &str) -> Result<()> {
    // _config.yml
    let config = format!(
r#"# Site settings
title: {}
description: A blog about technology and life
url: ""
baseurl: ""

# Build settings
markdown: pulldown-cmark
permalink: /:year/:month/:day/:title/

# Pagination
paginate: 10
paginate_path: "/blog/page:num/"

# Exclude from processing
exclude:
  - Gemfile
  - Gemfile.lock
  - node_modules
  - vendor
  - README.md
"#,
        site_name
    );
    fs::write(base.join("_config.yml"), config)?;
    
    // index.md
    let index = r#"---
layout: default
title: Home
---

# Welcome to JellRust!

This is your new JellRust site. Edit `index.md` to customize this page.

## Recent Posts

{% for post in site.posts limit:5 %}
- [{{ post.title }}]({{ post.url }}) - {{ post.date | date: "%B %d, %Y" }}
{% endfor %}
"#;
    fs::write(base.join("index.md"), index)?;
    
    // about.md
    let about = r#"---
layout: default
title: About
permalink: /about/
---

# About

This is the about page. Edit `about.md` to tell people about yourself!
"#;
    fs::write(base.join("about.md"), about)?;
    
    // _layouts/default.html
    let default_layout = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ page.title }} | {{ site.title }}</title>
    <link rel="stylesheet" href="{{ '/assets/css/style.css' | relative_url }}">
</head>
<body>
    {% include header.html %}
    
    <main class="container">
        {{ content }}
    </main>
    
    {% include footer.html %}
</body>
</html>
"#;
    fs::write(base.join("_layouts/default.html"), default_layout)?;
    
    // _layouts/post.html
    let post_layout = r#"---
layout: default
---

<article class="post">
    <header class="post-header">
        <h1 class="post-title">{{ page.title }}</h1>
        <p class="post-meta">
            <time datetime="{{ page.date | date_to_xmlschema }}">
                {{ page.date | date: "%B %d, %Y" }}
            </time>
            {% if page.author %}
            by {{ page.author }}
            {% endif %}
        </p>
    </header>
    
    <div class="post-content">
        {{ content }}
    </div>
    
    {% if page.tags %}
    <div class="post-tags">
        Tags: 
        {% for tag in page.tags %}
        <span class="tag">{{ tag }}</span>
        {% endfor %}
    </div>
    {% endif %}
</article>
"#;
    fs::write(base.join("_layouts/post.html"), post_layout)?;
    
    // _includes/header.html
    let header = r#"<header class="site-header">
    <div class="container">
        <h1 class="site-title">
            <a href="{{ '/' | relative_url }}">{{ site.title }}</a>
        </h1>
        <nav class="site-nav">
            <a href="{{ '/' | relative_url }}">Home</a>
            <a href="{{ '/about/' | relative_url }}">About</a>
        </nav>
    </div>
</header>
"#;
    fs::write(base.join("_includes/header.html"), header)?;
    
    // _includes/footer.html
    let footer = r#"<footer class="site-footer">
    <div class="container">
        <p>&copy; {{ 'now' | date: '%Y' }} {{ site.title }}. Built with JellRust.</p>
    </div>
</footer>
"#;
    fs::write(base.join("_includes/footer.html"), footer)?;
    
    // Sample post
    let post = r#"---
layout: post
title: "Welcome to JellRust!"
date: 2024-01-01 10:00:00 +0000
categories: [general]
tags: [welcome, first-post]
---

# Hello World!

Welcome to your new JellRust site! This is your first blog post.

## Getting Started

You can edit this post in `_posts/2024-01-01-welcome-to-jellrust.md`.

To create a new post:
1. Create a new file in `_posts/` with the format `YYYY-MM-DD-title.md`
2. Add front matter with layout, title, and date
3. Write your content in Markdown

## Features

JellRust supports:
- **Markdown** - Write content in plain Markdown
- **Liquid templates** - Powerful templating
- **Code highlighting** - Syntax highlighting for code blocks
- **Live reload** - See changes instantly

```rust
fn main() {
    println!("Hello, JellRust!");
}
```

Happy blogging! 🚀
"#;
    fs::write(base.join("_posts/2024-01-01-welcome-to-jellrust.md"), post)?;
    
    // assets/css/style.css
    let css = r#"/* JellRust Default Styles */

:root {
    --primary-color: #2c3e50;
    --accent-color: #3498db;
    --text-color: #333;
    --bg-color: #fff;
    --gray-light: #ecf0f1;
}

* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
    line-height: 1.6;
    color: var(--text-color);
    background-color: var(--bg-color);
}

.container {
    max-width: 800px;
    margin: 0 auto;
    padding: 0 20px;
}

/* Header */
.site-header {
    background-color: var(--primary-color);
    color: white;
    padding: 1.5rem 0;
    margin-bottom: 2rem;
}

.site-title a {
    color: white;
    text-decoration: none;
    font-size: 1.8rem;
}

.site-nav {
    margin-top: 1rem;
}

.site-nav a {
    color: white;
    text-decoration: none;
    margin-right: 1.5rem;
    padding: 0.5rem;
    border-radius: 4px;
    transition: background 0.3s;
}

.site-nav a:hover {
    background-color: rgba(255, 255, 255, 0.1);
}

/* Main content */
main {
    min-height: 60vh;
    padding: 2rem 0;
}

h1, h2, h3, h4, h5, h6 {
    margin: 1.5rem 0 1rem;
    color: var(--primary-color);
}

h1 { font-size: 2.5rem; }
h2 { font-size: 2rem; }
h3 { font-size: 1.5rem; }

p {
    margin-bottom: 1rem;
}

a {
    color: var(--accent-color);
    text-decoration: none;
}

a:hover {
    text-decoration: underline;
}

/* Posts */
.post-header {
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 2px solid var(--gray-light);
}

.post-title {
    margin: 0;
}

.post-meta {
    color: #7f8c8d;
    font-size: 0.9rem;
}

.post-content {
    margin-bottom: 2rem;
}

.post-tags {
    margin-top: 2rem;
}

.tag {
    display: inline-block;
    background-color: var(--gray-light);
    padding: 0.3rem 0.8rem;
    border-radius: 15px;
    font-size: 0.85rem;
    margin-right: 0.5rem;
}

/* Code blocks */
pre {
    background-color: #f8f8f8;
    border: 1px solid #ddd;
    border-radius: 4px;
    padding: 1rem;
    overflow-x: auto;
    margin: 1rem 0;
}

code {
    font-family: 'Courier New', monospace;
    font-size: 0.9rem;
}

/* Footer */
.site-footer {
    background-color: var(--gray-light);
    padding: 2rem 0;
    margin-top: 4rem;
    text-align: center;
    color: #7f8c8d;
}
"#;
    fs::write(base.join("assets/css/style.css"), css)?;
    
    // .gitignore
    let gitignore = r#"_site/
.jellrust-cache/
.DS_Store
*.swp
"#;
    fs::write(base.join(".gitignore"), gitignore)?;
    
    tracing::debug!("Created all default files");
    Ok(())
}

