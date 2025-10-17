use crate::error::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FrontMatter {
    /// Page/post title
    pub title: Option<String>,
    
    /// Layout to use
    pub layout: Option<String>,
    
    /// Publication date
    pub date: Option<String>,
    
    /// Author name
    pub author: Option<String>,
    
    /// Categories
    #[serde(default)]
    pub categories: Vec<String>,
    
    /// Tags
    #[serde(default)]
    pub tags: Vec<String>,
    
    /// Permalink override
    pub permalink: Option<String>,
    
    /// Whether this is published
    #[serde(default = "default_true")]
    pub published: bool,
    
    /// Custom front matter fields
    #[serde(flatten)]
    pub custom: HashMap<String, serde_yaml::Value>,
}

fn default_true() -> bool {
    true
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_post_date() {
        let post = Post::new(PathBuf::from("_posts/2024-01-15-test-post.md"));
        let date = post.parse_date_from_filename().unwrap();
        assert_eq!(date.format("%Y-%m-%d").to_string(), "2024-01-15");
    }
}

