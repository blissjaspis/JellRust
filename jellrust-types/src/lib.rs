use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

// Re-export FrontMatter from jellrust-markdown
pub use jellrust_markdown::FrontMatter;

// ============================================================================
// Server Types
// ============================================================================

/// Shared flag for triggering browser reload in development server
pub type ReloadFlag = Arc<RwLock<bool>>;

/// Channel for communicating file change events
pub type FileChangeChannel = mpsc::UnboundedSender<()>;

// ============================================================================
// Content Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    /// Path to the source file
    pub path: PathBuf,
    
    /// URL path for the generated page
    pub url: String,
    
    /// Front matter data
    pub front_matter: FrontMatter,
    
    /// Raw content (Markdown or HTML)
    pub content: String,
    
    /// Rendered HTML content
    pub html: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    /// Path to the source file
    pub path: PathBuf,
    
    /// URL path for the generated post
    pub url: String,
    
    /// Post date (from filename or front matter)
    pub date: DateTime<Utc>,
    
    /// Front matter data
    pub front_matter: FrontMatter,
    
    /// Raw content (Markdown)
    pub content: String,
    
    /// Rendered HTML content
    pub html: String,
    
    /// Excerpt (first paragraph or explicit)
    pub excerpt: String,
}

impl Page {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            url: String::new(),
            front_matter: FrontMatter::default(),
            content: String::new(),
            html: String::new(),
        }
    }
}

impl Post {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            url: String::new(),
            date: Utc::now(),
            front_matter: FrontMatter::default(),
            content: String::new(),
            html: String::new(),
            excerpt: String::new(),
        }
    }
    
    /// Parse date from filename (YYYY-MM-DD-title.md)
    pub fn parse_date_from_filename(&self) -> Option<DateTime<Utc>> {
        let filename = self.path.file_name()?.to_str()?;
        let parts: Vec<&str> = filename.split('-').collect();
        
        if parts.len() < 4 {
            return None;
        }
        
        let year = parts[0].parse::<i32>().ok()?;
        let month = parts[1].parse::<u32>().ok()?;
        let day = parts[2].parse::<u32>().ok()?;
        
        use chrono::TimeZone;
        Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).single()
    }
}

#[derive(Debug)]
pub struct Site {
    /// All pages
    pub pages: Vec<Page>,
    
    /// All posts (sorted by date, newest first)
    pub posts: Vec<Post>,
    
    /// Static files (images, CSS, JS, etc.)
    pub static_files: Vec<PathBuf>,
}

impl Site {
    pub fn new() -> Self {
        Self {
            pages: Vec::new(),
            posts: Vec::new(),
            static_files: Vec::new(),
        }
    }
}

impl Default for Site {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Site title
    #[serde(default = "default_title")]
    pub title: String,
    
    /// Site description
    #[serde(default)]
    pub description: String,
    
    /// Site URL (e.g., https://example.com)
    #[serde(default)]
    pub url: String,
    
    /// Base URL path (e.g., /blog)
    #[serde(default)]
    pub baseurl: String,
    
    /// Markdown engine
    #[serde(default = "default_markdown")]
    pub markdown: String,
    
    /// Permalink structure
    #[serde(default = "default_permalink")]
    pub permalink: String,
    
    /// Posts per page for pagination
    #[serde(default = "default_paginate")]
    pub paginate: usize,
    
    /// Pagination path pattern
    #[serde(default = "default_paginate_path")]
    pub paginate_path: String,
    
    /// Files/folders to exclude
    #[serde(default = "default_exclude")]
    pub exclude: Vec<String>,
    
    /// Files/folders to include (override exclude)
    #[serde(default)]
    pub include: Vec<String>,
    
    /// Plugins to enable
    #[serde(default)]
    pub plugins: Vec<String>,
    
    /// Custom variables
    #[serde(flatten)]
    pub custom: HashMap<String, serde_yaml::Value>,
}

fn default_title() -> String {
    "My Site".to_string()
}

fn default_markdown() -> String {
    "pulldown-cmark".to_string()
}

fn default_permalink() -> String {
    "/:year/:month/:day/:title/".to_string()
}

fn default_paginate() -> usize {
    10
}

fn default_paginate_path() -> String {
    "/page:num/".to_string()
}

fn default_exclude() -> Vec<String> {
    vec![
        "Gemfile".to_string(),
        "Gemfile.lock".to_string(),
        "node_modules".to_string(),
        "vendor".to_string(),
        ".git".to_string(),
        ".gitignore".to_string(),
        "_site".to_string(),
    ]
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: default_title(),
            description: String::new(),
            url: String::new(),
            baseurl: String::new(),
            markdown: default_markdown(),
            permalink: default_permalink(),
            paginate: default_paginate(),
            paginate_path: default_paginate_path(),
            exclude: default_exclude(),
            include: Vec::new(),
            plugins: Vec::new(),
            custom: HashMap::new(),
        }
    }
}

impl Config {
    /// Check if a path should be excluded from processing
    pub fn is_excluded(&self, path: &std::path::Path) -> bool {
        let path_str = path.to_string_lossy();
        
        // Check includes first (they override excludes)
        for pattern in &self.include {
            if path_str.contains(pattern) {
                return false;
            }
        }
        
        // Then check excludes
        for pattern in &self.exclude {
            if path_str.contains(pattern) {
                return true;
            }
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_post_date() {
        let post = Post::new(PathBuf::from("_posts/2024-01-15-test-post.md"));
        let date = post.parse_date_from_filename().unwrap();
        assert_eq!(date.format("%Y-%m-%d").to_string(), "2024-01-15");
    }
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.title, "My Site");
        assert_eq!(config.paginate, 10);
    }
}

