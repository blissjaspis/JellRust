use crate::error::{Error, Result};
use std::fs;
use std::path::Path;

// Re-export Config from jellrust-types
pub use jellrust_types::Config;

/// Extension trait for loading Config
pub trait ConfigExt {
    fn load<P: AsRef<Path>>(source_dir: P) -> Result<Self>
    where
        Self: Sized;
}

impl ConfigExt for Config {
    /// Load configuration from _config.yml
    fn load<P: AsRef<Path>>(source_dir: P) -> Result<Self> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
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

