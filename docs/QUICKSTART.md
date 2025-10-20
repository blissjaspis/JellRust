# JellRust Quick Start Guide

Get up and running with JellRust in 10 minutes!

## What You'll Build

A fully functional blog with:
- Homepage with recent posts
- Individual post pages
- About page
- Responsive design
- Live reload development server

## Step 1: Install Rust (5 minutes)

If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Restart your terminal and verify:

```bash
rustc --version
cargo --version
```

## Step 2: Build JellRust (2 minutes)

```bash
# Clone the repository
cd ~/Documents/Koding/Rust/JellRust

# Build in release mode for best performance
cargo build --release

# The binary will be at target/release/jellrust
```

Optional: Add to PATH
```bash
# Add this to your ~/.zshrc or ~/.bashrc
export PATH="$HOME/Documents/Koding/Rust/JellRust/target/release:$PATH"
```

## Step 3: Create Your First Site (1 minute)

```bash
# Create a new site
cargo run --release -- new my-blog

# Or if you added to PATH:
jellrust new my-blog
```

This creates:
```
my-blog/
├── _config.yml          # Site configuration
├── _layouts/            # HTML layouts
├── _includes/           # Reusable components
├── _posts/              # Your blog posts
├── assets/              # CSS, JS, images
├── index.md             # Homepage
└── about.md             # About page
```

## Step 4: Start the Dev Server (30 seconds)

```bash
cd my-blog

# Start the development server
cargo run --release -- serve

# Or if using jellrust binary:
jellrust serve
```

Visit `http://localhost:4000` in your browser! 🎉

## Step 5: Write Your First Post (2 minutes)

Create a new file in `_posts/`:

```bash
# Create a post with today's date
touch _posts/2024-10-17-my-first-post.md
```

Edit it with your favorite editor:

```markdown
---
layout: post
title: "My First Blog Post"
date: 2024-10-17 14:30:00 +0000
categories: [tech, rust]
tags: [rust, blogging, jellrust]
---

# Hello JellRust!

This is my first post using **JellRust**, a static site generator written in Rust.

## Why Rust?

- Fast compilation
- Memory safe
- Great tooling

## Code Example

```rust
fn main() {
    println!("Hello, JellRust!");
}
```

## Next Steps

I'm excited to:
1. Learn more Rust
2. Customize my theme
3. Add more content

Stay tuned! 🚀
```

Save and watch your browser auto-reload!

## Step 6: Customize Your Site (ongoing)

### Update Site Info

Edit `_config.yml`:

```yaml
title: My Awesome Blog
description: Thoughts on Rust, web development, and technology
url: "https://yourdomain.com"

# Your info
author:
  name: Your Name
  email: your.email@example.com
```

### Modify the Homepage

Edit `index.md`:

```markdown
---
layout: default
title: Home
---

# Welcome! 👋

I'm a developer passionate about Rust and web technologies.

## Latest Posts

{% for post in site.posts limit:5 %}
### [{{ post.title }}]({{ post.url }})
*{{ post.date | date: "%B %d, %Y" }}*

{{ post.excerpt }}

[Read more →]({{ post.url }})
{% endfor %}
```

### Update the About Page

Edit `about.md`:

```markdown
---
layout: default
title: About Me
permalink: /about/
---

# About Me

Hi! I'm learning Rust by building a blog with JellRust.

## What I Do

- Software Development
- Open Source Contributions
- Technical Writing

## Contact

- Email: your.email@example.com
- GitHub: [@yourusername](https://github.com/yourusername)
- Twitter: [@yourusername](https://twitter.com/yourusername)
```

### Customize Styles

Edit `assets/css/style.css`:

```css
:root {
    --primary-color: #ff6b6b;  /* Change to your brand color */
    --accent-color: #4ecdc4;
}

/* Add your custom styles */
.hero {
    text-align: center;
    padding: 4rem 0;
    background: linear-gradient(135deg, var(--primary-color), var(--accent-color));
    color: white;
}
```

## Step 7: Build for Production (1 minute)

When you're ready to publish:

```bash
# Build the site
cargo run --release -- build

# Or:
jellrust build
```

This creates a `_site/` directory with your complete static website!

## Step 8: Deploy (5 minutes)

### Option A: GitHub Pages

1. Create a new repository on GitHub
2. Push your site:

```bash
git init
git add .
git commit -m "Initial commit"
git branch -M main
git remote add origin https://github.com/yourusername/your-repo.git
git push -u origin main
```

3. Deploy the `_site` folder:

```bash
# Create gh-pages branch
git checkout --orphan gh-pages
git rm -rf .
cp -r _site/* .
git add .
git commit -m "Deploy site"
git push origin gh-pages
```

4. Enable GitHub Pages in repository settings

### Option B: Netlify

1. Sign up at [netlify.com](https://netlify.com)
2. Connect your Git repository
3. Build command: `cargo build --release && jellrust build`
4. Publish directory: `_site`
5. Deploy!

### Option C: Vercel

1. Sign up at [vercel.com](https://vercel.com)
2. Import your Git repository
3. Configure build:
   - Build command: `cargo build --release && jellrust build`
   - Output directory: `_site`
4. Deploy!

## Common Tasks

### Add a New Post

```bash
# Create file with format: YYYY-MM-DD-title.md
touch _posts/2024-10-18-second-post.md
```

### Add a New Page

```bash
# Create a markdown file
touch contact.md
```

Add front matter:
```markdown
---
layout: default
title: Contact
permalink: /contact/
---

# Get In Touch
```

### Include Reusable Components

Create `_includes/newsletter.html`:

```html
<div class="newsletter">
    <h3>Subscribe to My Newsletter</h3>
    <form action="/subscribe" method="post">
        <input type="email" placeholder="your@email.com">
        <button type="submit">Subscribe</button>
    </form>
</div>
```

Use in layouts or pages:
```liquid
{% include newsletter.html %}
```

### Add Images

```bash
# Place images in assets/images/
cp ~/Pictures/profile.jpg assets/images/
```

Use in markdown:
```markdown
![My Profile](/assets/images/profile.jpg)
```

### Create Custom Layouts

Create `_layouts/photo-post.html`:

```html
---
layout: default
---

<article class="photo-post">
    <div class="featured-image">
        <img src="{{ page.image }}" alt="{{ page.title }}">
    </div>
    
    <header>
        <h1>{{ page.title }}</h1>
        <time>{{ page.date | date: "%B %d, %Y" }}</time>
    </header>
    
    <div class="content">
        {{ content }}
    </div>
</article>
```

Use it:
```markdown
---
layout: photo-post
title: "My Photo Post"
image: /assets/images/featured.jpg
---
```

## Troubleshooting

### Port Already in Use

```bash
# Use a different port
jellrust serve --port 3000
```

### Build Fails

```bash
# Clean and rebuild
jellrust clean
cargo build --release
```

### Live Reload Not Working

- Check browser console for errors
- Try hard refresh (Ctrl+Shift+R or Cmd+Shift+R)
- Restart the dev server

### Layout Not Found

Make sure:
- Layout file exists in `_layouts/`
- Filename matches: `default.html`, `post.html`
- Front matter specifies correct layout name

## Next Steps

Now that you have a working site:

1. **Learn Rust** - Follow [LEARNING_PATH.md](LEARNING_PATH.md)
2. **Understand Architecture** - Read [ARCHITECTURE.md](ARCHITECTURE.md)
3. **Customize Further** - Check out Liquid template docs
4. **Add Features** - See [ROADMAP.md](ROADMAP.md) for ideas
5. **Contribute** - Read [CONTRIBUTING.md](CONTRIBUTING.md)

## Getting Help

- **Documentation**: Check the main [README.md](README.md)
- **Issues**: Search [GitHub Issues](https://github.com/yourusername/jellrust/issues)
- **Questions**: Open a [GitHub Discussion](https://github.com/yourusername/jellrust/discussions)

## Useful Commands

```bash
# Create a new site
jellrust new my-site

# Start dev server
jellrust serve

# Build for production
jellrust build

# Build and watch for changes
jellrust build --watch

# Include draft posts
jellrust serve --drafts

# Clean generated files
jellrust clean

# Check site health
jellrust doctor

# Get help
jellrust --help
jellrust serve --help
```

## Pro Tips

1. **Use `--drafts` while writing**: `jellrust serve --drafts`
2. **Keep posts in drafts folder** until ready: `_drafts/`
3. **Version control**: Commit often, use `.gitignore`
4. **Images**: Optimize before adding (use ImageOptim, etc.)
5. **Code blocks**: Specify language for syntax highlighting

## Example Sites

Check out these examples:
- `examples/blog/` - Personal blog
- `examples/docs/` - Documentation site
- `examples/portfolio/` - Portfolio site

## What's Next?

Choose your path:

### For Learners 📚
→ Follow [LEARNING_PATH.md](LEARNING_PATH.md) to learn Rust

### For Builders 🔨
→ Read [ARCHITECTURE.md](ARCHITECTURE.md) to understand the internals

### For Contributors 🤝
→ See [CONTRIBUTING.md](CONTRIBUTING.md) to help improve JellRust

### For Users 🚀
→ Just keep blogging and enjoying your fast static site!

---

**Questions?** Open an issue or discussion on GitHub!

Happy blogging with JellRust! 🦀✨

