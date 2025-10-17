use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn execute(source: PathBuf) -> Result<()> {
    let site_dir = source.join("_site");
    
    if !site_dir.exists() {
        println!("✅ Nothing to clean - _site directory doesn't exist");
        return Ok(());
    }
    
    tracing::info!("Removing {}", site_dir.display());
    
    fs::remove_dir_all(&site_dir)
        .context("Failed to remove _site directory")?;
    
    println!("✅ Successfully removed _site directory");
    
    Ok(())
}

