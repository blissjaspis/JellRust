use anyhow::Result;
use jellrust_core::{config::{Config, ConfigExt}, site::SiteBuilder};
use std::path::PathBuf;

pub async fn execute(
    source: PathBuf,
    destination: PathBuf,
    drafts: bool,
    watch: bool,
) -> Result<()> {
    tracing::info!("Building site from {} to {}", source.display(), destination.display());
    
    // Load configuration
    let config = Config::load(&source)?;
    
    // Build the site
    let mut builder = SiteBuilder::new(source.clone(), destination.clone(), config);
    builder.set_include_drafts(drafts);
    
    builder.build().await?;
    
    println!("✅ Site built successfully!");
    println!("📁 Output: {}", destination.display());
    
    if watch {
        println!("\n👀 Watching for changes... (Press Ctrl+C to stop)");
        watch_and_rebuild(source, destination, drafts).await?;
    }
    
    Ok(())
}

async fn watch_and_rebuild(
    source: PathBuf,
    destination: PathBuf,
    drafts: bool,
) -> Result<()> {
    use notify::{RecursiveMode, Watcher};
    use tokio::sync::mpsc;
    
    let (tx, mut rx) = mpsc::channel(100);
    
    let mut watcher = notify::recommended_watcher(move |res| {
        if let Ok(event) = res {
            let _ = tx.blocking_send(event);
        }
    })?;
    
    // Watch the source directory
    watcher.watch(&source, RecursiveMode::Recursive)?;
    
    while let Some(_event) = rx.recv().await {
        tracing::info!("Change detected, rebuilding...");
        
        match Config::load(&source) {
            Ok(config) => {
                let mut builder = SiteBuilder::new(source.clone(), destination.clone(), config);
                builder.set_include_drafts(drafts);
                
                match builder.build().await {
                    Ok(_) => println!("✅ Site rebuilt successfully!"),
                    Err(e) => eprintln!("❌ Build failed: {}", e),
                }
            }
            Err(e) => eprintln!("❌ Failed to load config: {}", e),
        }
    }
    
    Ok(())
}

