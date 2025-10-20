use crate::config::Config;
use crate::content::{Page, Post, Site};
use crate::error::Result;
use jellrust_markdown::MarkdownProcessor;
use jellrust_template::TemplateEngine;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct SiteBuilder {
    source: PathBuf,
    destination: PathBuf,
    config: Config,
    include_drafts: bool,
    markdown_processor: MarkdownProcessor,
    template_engine: TemplateEngine,
}

impl SiteBuilder {
    pub fn new(source: PathBuf, destination: PathBuf, config: Config) -> Self {
        let markdown_processor = MarkdownProcessor::new();
        let template_engine = TemplateEngine::new(source.clone());
        
        Self {
            source,
            destination,
            config,
            include_drafts: false,
            markdown_processor,
            template_engine,
        }
    }
    
    pub fn set_include_drafts(&mut self, include: bool) {
        self.include_drafts = include;
    }
    
    /// Build the entire site
    pub async fn build(&mut self) -> Result<()> {
        tracing::info!("Starting site build...");
        
        // Create destination directory
        fs::create_dir_all(&self.destination)?;
        
        // Collect all content
        let mut site = Site::new();
        
        // Process posts
        let posts_dir = self.source.join("_posts");
        if posts_dir.exists() {
            tracing::info!("Processing posts...");
            site.posts = self.process_posts(&posts_dir)?;
        }
        
        // Process drafts if enabled
        if self.include_drafts {
            let drafts_dir = self.source.join("_drafts");
            if drafts_dir.exists() {
                tracing::info!("Processing drafts...");
                let mut drafts = self.process_posts(&drafts_dir)?;
                site.posts.append(&mut drafts);
            }
        }
        
        // Sort posts by date (newest first)
        site.posts.sort_by(|a, b| b.date.cmp(&a.date));
        
        // Process pages
        tracing::info!("Processing pages...");
        site.pages = self.process_pages()?;
        
        // Copy static files
        tracing::info!("Copying static files...");
        self.copy_static_files()?;
        
        // Render all content
        tracing::info!("Rendering content...");
        self.render_posts(&site).await?;
        self.render_pages(&site).await?;
        
        tracing::info!("Build complete!");
        Ok(())
    }
    
    /// Process all posts in a directory
    fn process_posts(&mut self, dir: &Path) -> Result<Vec<Post>> {
        let mut posts = Vec::new();
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if !path.is_file() {
                continue;
            }
            
            let ext = path.extension().and_then(|s| s.to_str());
            if !matches!(ext, Some("md") | Some("markdown")) {
                continue;
            }
            
            tracing::debug!("Processing post: {}", path.display());
            
            let content = fs::read_to_string(&path)?;
            let (front_matter, body) = self.markdown_processor.parse_front_matter(&content)?;
            
            // Skip unpublished posts
            if !front_matter.published {
                tracing::debug!("Skipping unpublished post: {}", path.display());
                continue;
            }
            
            let mut post = Post::new(path.clone());
            post.front_matter = front_matter;
            post.content = body.to_string();
            
            // Parse date from filename
            if let Some(date) = post.parse_date_from_filename() {
                post.date = date;
            }
            
            // Generate URL
            post.url = self.generate_post_url(&post);
            
            // Render markdown to HTML
            post.html = self.markdown_processor.render(&post.content)?;
            
            // Extract excerpt
            post.excerpt = self.extract_excerpt(&post.html);
            
            posts.push(post);
        }
        
        Ok(posts)
    }
    
    /// Process all pages (non-post content)
    fn process_pages(&mut self) -> Result<Vec<Page>> {
        let mut pages = Vec::new();
        
        for entry in WalkDir::new(&self.source)
            .follow_links(true)
            .into_iter()
            .filter_entry(|e| !self.is_special_directory(e.path()))
        {
            let entry = entry?;
            let path = entry.path();
            
            if !path.is_file() {
                continue;
            }
            
            // Skip if excluded
            if self.config.is_excluded(path) {
                continue;
            }
            
            let ext = path.extension().and_then(|s| s.to_str());
            if !matches!(ext, Some("md") | Some("markdown") | Some("html")) {
                continue;
            }
            
            // Skip posts directories
            if path.starts_with(self.source.join("_posts"))
                || path.starts_with(self.source.join("_drafts"))
            {
                continue;
            }
            
            tracing::debug!("Processing page: {}", path.display());
            
            let content = fs::read_to_string(path)?;
            let (front_matter, body) = self.markdown_processor.parse_front_matter(&content)?;
            
            let mut page = Page::new(path.to_path_buf());
            page.front_matter = front_matter;
            page.content = body.to_string();
            
            // Generate URL
            page.url = self.generate_page_url(&page);
            
            // Render content
            if matches!(ext, Some("md") | Some("markdown")) {
                page.html = self.markdown_processor.render(&page.content)?;
            } else {
                page.html = page.content.clone();
            }
            
            pages.push(page);
        }
        
        Ok(pages)
    }
    
    /// Check if a path is a special Jekyll directory
    fn is_special_directory(&self, path: &Path) -> bool {
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            matches!(
                name,
                "_site" | "_layouts" | "_includes" | "_data" | "node_modules" | ".git"
            )
        } else {
            false
        }
    }
    
    /// Generate URL for a post based on permalink pattern
    fn generate_post_url(&self, post: &Post) -> String {
        if let Some(permalink) = &post.front_matter.permalink {
            return permalink.clone();
        }
        
        let mut url = self.config.permalink.clone();
        
        url = url.replace(":year", &post.date.format("%Y").to_string());
        url = url.replace(":month", &post.date.format("%m").to_string());
        url = url.replace(":day", &post.date.format("%d").to_string());
        
        // Extract title from filename
        if let Some(filename) = post.path.file_stem().and_then(|s| s.to_str()) {
            // Remove date prefix (YYYY-MM-DD-)
            let title = filename
                .split('-')
                .skip(3)
                .collect::<Vec<_>>()
                .join("-");
            url = url.replace(":title", &title);
        }
        
        url
    }
    
    /// Generate URL for a page
    fn generate_page_url(&self, page: &Page) -> String {
        if let Some(permalink) = &page.front_matter.permalink {
            return permalink.clone();
        }
        
        let rel_path = page
            .path
            .strip_prefix(&self.source)
            .unwrap_or(&page.path);
        
        let url = rel_path.with_extension("html");
        
        // Convert to string and make it web-friendly
        url.to_string_lossy()
            .replace("\\", "/")
            .trim_start_matches('/')
            .to_string()
    }
    
    /// Extract excerpt from HTML content
    fn extract_excerpt(&self, html: &str) -> String {
        // Simple excerpt: first paragraph or first 200 characters
        if let Some(start) = html.find("<p>") {
            if let Some(end) = html[start..].find("</p>") {
                let excerpt = &html[start + 3..start + end];
                return excerpt.to_string();
            }
        }
        
        html.chars().take(200).collect::<String>() + "..."
    }
    
    /// Copy static files (CSS, JS, images, etc.)
    fn copy_static_files(&self) -> Result<()> {
        let assets_dir = self.source.join("assets");
        if assets_dir.exists() {
            let dest_assets = self.destination.join("assets");
            self.copy_directory(&assets_dir, &dest_assets)?;
        }
        
        Ok(())
    }
    
    /// Recursively copy a directory
    fn copy_directory(&self, src: &Path, dest: &Path) -> Result<()> {
        fs::create_dir_all(dest)?;
        
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name();
            let dest_path = dest.join(&file_name);
            
            if path.is_dir() {
                self.copy_directory(&path, &dest_path)?;
            } else {
                fs::copy(&path, &dest_path)?;
                tracing::debug!("Copied: {} -> {}", path.display(), dest_path.display());
            }
        }
        
        Ok(())
    }
    
    /// Render all posts with their layouts
    async fn render_posts(&mut self, site: &Site) -> Result<()> {
        for post in &site.posts {
            let output_path = self.destination.join(post.url.trim_start_matches('/'));
            
            // Ensure parent directory exists
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Render with template
            let html = self.template_engine.render_post(post, site, &self.config)?;
            
            fs::write(&output_path, html)?;
            tracing::debug!("Rendered post: {}", output_path.display());
        }
        
        Ok(())
    }
    
    /// Render all pages with their layouts
    async fn render_pages(&mut self, site: &Site) -> Result<()> {
        for page in &site.pages {
            let output_path = self.destination.join(page.url.trim_start_matches('/'));
            
            // Ensure parent directory exists
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Render with template
            let html = self.template_engine.render_page(page, site, &self.config)?;
            
            fs::write(&output_path, html)?;
            tracing::debug!("Rendered page: {}", output_path.display());
        }
        
        Ok(())
    }
}

