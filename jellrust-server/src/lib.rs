use anyhow::Result;
use axum::{
    body::Body,
    extract::State,
    http::{Response, StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Router,
};
use jellrust_core::config::Config;
use notify::{RecursiveMode, Watcher};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

pub struct DevServer {
    source: PathBuf,
    destination: PathBuf,
    #[allow(dead_code)]
    config: Config,
    port: u16,
    host: String,
}

#[derive(Clone)]
struct AppState {
    destination: PathBuf,
    reload_flag: Arc<RwLock<bool>>,
}

impl DevServer {
    pub fn new(
        source: PathBuf,
        destination: PathBuf,
        config: Config,
        port: u16,
        host: String,
    ) -> Self {
        Self {
            source,
            destination,
            config,
            port,
            host,
        }
    }
    
    pub async fn run(self) -> Result<()> {
        let reload_flag = Arc::new(RwLock::new(false));
        
        // Set up file watcher for live reload
        let _watcher = self.setup_watcher(reload_flag.clone())?;
        
        // Create app state
        let state = AppState {
            destination: self.destination.clone(),
            reload_flag,
        };
        
        // Build router
        let app = Router::new()
            .route("/__reload__", get(reload_status))
            .fallback(serve_static)
            .nest_service("/", ServeDir::new(&self.destination))
            .with_state(state);
        
        // Create socket address
        let addr: SocketAddr = format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Invalid address");
        
        tracing::info!("Listening on http://{}", addr);
        
        // Start server
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        
        Ok(())
    }
    
    fn setup_watcher(
        &self,
        reload_flag: Arc<RwLock<bool>>,
    ) -> Result<notify::RecommendedWatcher> {
        
        let source = self.source.clone();
        
        let watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                tracing::debug!("File change detected: {:?}", event);
                
                // Set reload flag
                let flag = reload_flag.clone();
                tokio::spawn(async move {
                    let mut flag = flag.write().await;
                    *flag = true;
                });
                
                // TODO: Rebuild site here
                // For now, just log the change
                tracing::info!("Change detected, site should be rebuilt");
            }
        })?;
        
        // Watch source directory
        let mut w = watcher;
        w.watch(&source, RecursiveMode::Recursive)?;
        
        Ok(w)
    }
}

/// Handler for reload status endpoint (for live reload client)
async fn reload_status(State(state): State<AppState>) -> impl IntoResponse {
    let mut flag = state.reload_flag.write().await;
    let should_reload = *flag;
    
    if should_reload {
        *flag = false;
        Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("reload"))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("ok"))
            .unwrap()
    }
}

/// Serve static files
async fn serve_static(
    State(state): State<AppState>,
    uri: Uri,
) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    
    // Handle root path
    let file_path = if path.is_empty() || path.ends_with('/') {
        state.destination.join(path).join("index.html")
    } else {
        state.destination.join(path)
    };
    
    // Try to serve the file
    if file_path.exists() && file_path.is_file() {
        match tokio::fs::read(&file_path).await {
            Ok(content) => {
                // Inject live reload script for HTML files
                if file_path.extension().and_then(|s| s.to_str()) == Some("html") {
                    let html = String::from_utf8_lossy(&content);
                    let with_reload = inject_reload_script(&html);
                    return Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "text/html; charset=utf-8")
                        .body(Body::from(with_reload))
                        .unwrap();
                }
                
                // Serve other files as-is
                Response::builder()
                    .status(StatusCode::OK)
                    .body(Body::from(content))
                    .unwrap()
            }
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal Server Error"))
                .unwrap(),
        }
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("404 Not Found"))
            .unwrap()
    }
}

/// Inject live reload script into HTML
fn inject_reload_script(html: &str) -> String {
    let script = r#"
<script>
(function() {
    let lastCheck = Date.now();
    
    function checkReload() {
        fetch('/__reload__')
            .then(res => res.text())
            .then(data => {
                if (data === 'reload') {
                    console.log('Reloading page...');
                    location.reload();
                }
            })
            .catch(err => console.error('Reload check failed:', err));
    }
    
    setInterval(checkReload, 1000);
    console.log('Live reload enabled');
})();
</script>
"#;
    
    // Inject before </body> or at the end
    if let Some(pos) = html.rfind("</body>") {
        let mut result = html.to_string();
        result.insert_str(pos, script);
        result
    } else {
        format!("{}{}", html, script)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_inject_reload_script() {
        let html = "<html><body><h1>Test</h1></body></html>";
        let result = inject_reload_script(html);
        
        assert!(result.contains("<script>"));
        assert!(result.contains("checkReload"));
    }
}

