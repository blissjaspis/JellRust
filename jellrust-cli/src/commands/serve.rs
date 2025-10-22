use anyhow::Result;
use jellrust_core::{config::{Config, ConfigExt}, site::SiteBuilder};
use jellrust_server::DevServer;
use std::path::PathBuf;

pub async fn execute(
    source: PathBuf,
    port: u16,
    host: String,
    open: bool,
    drafts: bool,
) -> Result<()> {
    tracing::info!("Starting development server...");
    
    // Load configuration
    let config = Config::load(&source)?;
    
    // Build the site first
    let destination = source.join("_site");
    let mut builder = SiteBuilder::new(source.clone(), destination.clone(), config.clone());
    builder.set_include_drafts(drafts);
    builder.build().await?;
    
    println!("✅ Initial build complete!");
    
    // Start the dev server
    let server = DevServer::new(source, destination, config, port, host.clone(), drafts);
    
    let url = format!("http://{}:{}", host, port);
    println!("\n🚀 Server running at {}", url);
    println!("👀 Watching for changes...");
    println!("   Press Ctrl+C to stop\n");
    
    if open {
        if let Err(e) = open::that(&url) {
            tracing::warn!("Failed to open browser: {}", e);
        }
    }
    
    server.run().await?;
    
    Ok(())
}

