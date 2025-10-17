use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

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
    /// Load configuration from _config.yml
    pub fn load<P: AsRef<Path>>(source_dir: P) -> Result<Self> {
        let config_path = source_dir.as_ref().join("_config.yml");
        
        if !config_path.exists() {
            tracing::warn!("No _config.yml found, using defaults");
            return Ok(Config::default());
        }
        
        tracing::info!("Loading config from {}", config_path.display());
        
        let content = fs::read_to_string(&config_path)
            .map_err(|e| Error::Config(format!("Failed to read config: {}", e)))?;
        
        let config: Config = serde_yaml::from_str(&content)?;
        
        Ok(config)
    }
    
    /// Check if a path should be excluded from processing
    pub fn is_excluded(&self, path: &Path) -> bool {
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
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.title, "My Site");
        assert_eq!(config.paginate, 10);
    }
    
    #[test]
    fn test_exclusion() {
        let config = Config::default();
        assert!(config.is_excluded(Path::new("node_modules/test.js")));
        assert!(config.is_excluded(Path::new("_site/index.html")));
        assert!(!config.is_excluded(Path::new("_posts/hello.md")));
    }
}

