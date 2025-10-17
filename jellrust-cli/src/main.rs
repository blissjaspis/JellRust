use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod commands;

/// JellRust - A blazingly fast static site generator written in Rust
#[derive(Parser)]
#[command(name = "jellrust")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new JellRust site
    New {
        /// Name of the site
        name: String,
        /// Path where to create the site (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    
    /// Build the site
    Build {
        /// Source directory
        #[arg(short, long, default_value = ".")]
        source: PathBuf,
        /// Destination directory
        #[arg(short, long, default_value = "_site")]
        destination: PathBuf,
        /// Include draft posts
        #[arg(long)]
        drafts: bool,
        /// Watch for changes and rebuild
        #[arg(short, long)]
        watch: bool,
    },
    
    /// Serve the site locally with live reload
    Serve {
        /// Source directory
        #[arg(short, long, default_value = ".")]
        source: PathBuf,
        /// Port to serve on
        #[arg(short, long, default_value = "4000")]
        port: u16,
        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        /// Open browser automatically
        #[arg(short, long)]
        open: bool,
        /// Include draft posts
        #[arg(long)]
        drafts: bool,
    },
    
    /// Clean the site (remove _site directory)
    Clean {
        /// Source directory
        #[arg(short, long, default_value = ".")]
        source: PathBuf,
    },
    
    /// Doctor - Check your site for common issues
    Doctor {
        /// Source directory
        #[arg(short, long, default_value = ".")]
        source: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "jellrust=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, path } => {
            commands::new::execute(name, path)?;
        }
        Commands::Build {
            source,
            destination,
            drafts,
            watch,
        } => {
            commands::build::execute(source, destination, drafts, watch).await?;
        }
        Commands::Serve {
            source,
            port,
            host,
            open,
            drafts,
        } => {
            commands::serve::execute(source, port, host, open, drafts).await?;
        }
        Commands::Clean { source } => {
            commands::clean::execute(source)?;
        }
        Commands::Doctor { source } => {
            commands::doctor::execute(source)?;
        }
    }

    Ok(())
}

