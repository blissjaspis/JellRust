use anyhow::Result;
use axum::{
    body::Body,
    extract::State,
    http::{Response, StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Router,
};
use jellrust_core::{config::Config, site::SiteBuilder};
use jellrust_types::{FileChangeChannel, ReloadFlag};
use notify::{Event as NotifyEvent, EventKind, RecursiveMode, Watcher};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::time::Duration;
use tower_http::services::ServeDir;

// ============================================================================
// Constants
// ============================================================================

/// Duration to wait for file changes to settle before rebuilding
const DEBOUNCE_DURATION_MS: u64 = 300;

/// Interval for client-side reload checks (in milliseconds)
const RELOAD_CHECK_INTERVAL_MS: u64 = 1000;

/// Endpoint for live reload status checks
const RELOAD_ENDPOINT: &str = "/__reload__";

/// HTML file extension
const HTML_EXTENSION: &str = "html";

// ============================================================================
// Server Structures
// ============================================================================

/// Development server with hot-reload capabilities
pub struct DevServer {
    source: PathBuf,
    destination: PathBuf,
    #[allow(dead_code)]
    config: Config,
    port: u16,
    host: String,
    include_drafts: bool,
}

/// Shared application state for HTTP handlers
#[derive(Clone)]
struct AppState {
    destination: PathBuf,
    reload_flag: ReloadFlag,
}

// ============================================================================
// DevServer Implementation
// ============================================================================

impl DevServer {
    pub fn new(
        source: PathBuf,
        destination: PathBuf,
        config: Config,
        port: u16,
        host: String,
        include_drafts: bool,
    ) -> Self {
        Self {
            source,
            destination,
            config,
            port,
            host,
            include_drafts,
        }
    }
    
    /// Start the development server with hot-reload capabilities
    pub async fn run(self) -> Result<()> {
        let reload_flag = Arc::new(RwLock::new(false));
        let (file_change_tx, file_change_rx) = mpsc::unbounded_channel();

        // Spawn file change handler with debouncing
        self.spawn_file_change_handler(
            file_change_rx,
            reload_flag.clone(),
        );

        // Set up file watcher
        let _watcher = self.setup_watcher(file_change_tx)?;

        // Start HTTP server
        self.start_http_server(reload_flag).await?;

        Ok(())
    }

    /// Spawn a task to handle file changes with debouncing
    fn spawn_file_change_handler(
        &self,
        rx: mpsc::UnboundedReceiver<()>,
        reload_flag: ReloadFlag,
    ) {
        let source = self.source.clone();
        let destination = self.destination.clone();
        let config = self.config.clone();
        let include_drafts = self.include_drafts;

        tokio::spawn(async move {
            handle_file_changes(rx, reload_flag, source, destination, config, include_drafts).await;
        });
    }

    /// Start the HTTP server
    async fn start_http_server(&self, reload_flag: ReloadFlag) -> Result<()> {
        let state = AppState {
            destination: self.destination.clone(),
            reload_flag,
        };

        let app = Router::new()
            .route(RELOAD_ENDPOINT, get(reload_status))
            .fallback(serve_static)
            .nest_service("/", ServeDir::new(&self.destination))
            .with_state(state);

        let addr: SocketAddr = format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Invalid socket address");

        tracing::info!("Listening on http://{}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
    
    /// Set up file system watcher for source directory
    fn setup_watcher(
        &self,
        tx: FileChangeChannel,
    ) -> Result<notify::RecommendedWatcher> {
        let destination = canonicalize_path(&self.destination);

        tracing::info!("Watching source directory, ignoring: {:?}", destination);

        let watcher = notify::recommended_watcher(move |res: notify::Result<NotifyEvent>| {
            if let Ok(event) = res {
                if should_trigger_rebuild(&event, &destination) {
                    tracing::info!("Source file change detected: {:?}", event.paths);
                    let _ = tx.send(());
                }
            }
        })?;

        let mut w = watcher;
        w.watch(&self.source, RecursiveMode::Recursive)?;

        Ok(w)
    }
}

// ============================================================================
// File Watching & Rebuild Logic
// ============================================================================

/// Handle file changes with debouncing to avoid rebuilding on every single change
async fn handle_file_changes(
    mut rx: mpsc::UnboundedReceiver<()>,
    reload_flag: ReloadFlag,
    source: PathBuf,
    destination: PathBuf,
    config: Config,
    include_drafts: bool,
) {
    let debounce_duration = Duration::from_millis(DEBOUNCE_DURATION_MS);

    loop {
        // Wait for first file change event
        if rx.recv().await.is_none() {
            break; // Channel closed
        }

        tracing::info!("File change detected, waiting for quiet period...");

        // Debounce: wait for a period of no events
        wait_for_quiet_period(&mut rx, debounce_duration).await;

        // Trigger rebuild
        trigger_reload(&reload_flag).await;
        rebuild_site_with_logging(&source, &destination, &config, include_drafts).await;
    }
}

/// Wait for a quiet period (no file changes) before proceeding
async fn wait_for_quiet_period(
    rx: &mut mpsc::UnboundedReceiver<()>,
    debounce_duration: Duration,
) {
    let quiet_start = std::time::Instant::now();

    loop {
        match tokio::time::timeout(debounce_duration, rx.recv()).await {
            Ok(Some(_)) => {
                tracing::debug!("Additional change detected, resetting timer");
                // Keep waiting - more changes are coming
                continue;
            }
            Ok(None) => {
                // Channel closed
                return;
            }
            Err(_) => {
                // Timeout reached - no events for the full debounce duration
                let quiet_time = quiet_start.elapsed();
                tracing::info!("Quiet period completed after {:?}, rebuilding...", quiet_time);
                return;
            }
        }
    }
}

/// Determine if a file system event should trigger a rebuild
fn should_trigger_rebuild(event: &NotifyEvent, destination: &Path) -> bool {
    // Filter out events from the destination directory to prevent infinite rebuild loop
    let is_destination_event = event.paths.iter().any(|path| {
        let canonical_path = canonicalize_path(path);
        canonical_path.starts_with(destination)
    });

    if is_destination_event {
        tracing::debug!("Ignoring event from destination: {:?}", event);
        return false;
    }

    // Only trigger rebuild for relevant file changes
    let is_relevant_event = matches!(
        event.kind,
        EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
    );

    if !is_relevant_event {
        tracing::debug!("Ignoring irrelevant event: {:?}", event);
        return false;
    }

    true
}

/// Canonicalize a path, falling back to the original if it fails
fn canonicalize_path(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

/// Set the reload flag to notify clients to refresh
async fn trigger_reload(reload_flag: &ReloadFlag) {
    let mut flag = reload_flag.write().await;
    *flag = true;
}

/// Rebuild the site and log the result
async fn rebuild_site_with_logging(
    source: &PathBuf,
    destination: &PathBuf,
    config: &Config,
    include_drafts: bool,
) {
    match rebuild_site(source, destination, config, include_drafts).await {
        Ok(_) => tracing::info!("✅ Site rebuilt successfully"),
        Err(e) => tracing::error!("❌ Failed to rebuild site: {}", e),
    }
}

// ============================================================================
// Site Building
// ============================================================================

/// Rebuild the site when files change
async fn rebuild_site(
    source: &PathBuf,
    destination: &PathBuf,
    config: &Config,
    include_drafts: bool,
) -> Result<()> {
    let mut builder = SiteBuilder::new(source.clone(), destination.clone(), config.clone());
    builder.set_include_drafts(include_drafts);
    builder.build().await.map_err(anyhow::Error::from)
}

// ============================================================================
// HTTP Handlers
// ============================================================================

/// Handler for reload status endpoint (for live reload client)
async fn reload_status(State(state): State<AppState>) -> impl IntoResponse {
    let mut flag = state.reload_flag.write().await;
    let should_reload = *flag;
    
    // Reset flag and notify client
    if should_reload {
        *flag = false;
        build_response(StatusCode::OK, "reload")
    } else {
        build_response(StatusCode::OK, "ok")
    }
}

/// Serve static files with live reload injection for HTML
async fn serve_static(
    State(state): State<AppState>,
    uri: Uri,
) -> impl IntoResponse {
    let file_path = resolve_file_path(&state.destination, uri.path());
    
    match serve_file(&file_path).await {
        Ok(response) => response,
        Err(status) => build_response(status, status_message(status)),
    }
}

/// Resolve URI path to file system path
fn resolve_file_path(destination: &Path, uri_path: &str) -> PathBuf {
    let path = uri_path.trim_start_matches('/');
    
    if path.is_empty() || path.ends_with('/') {
        destination.join(path).join("index.html")
    } else {
        destination.join(path)
    }
}

/// Serve a file from the file system
async fn serve_file(file_path: &Path) -> Result<Response<Body>, StatusCode> {
    if !file_path.exists() || !file_path.is_file() {
        return Err(StatusCode::NOT_FOUND);
    }

    let content = tokio::fs::read(file_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Inject live reload script for HTML files
    if is_html_file(file_path) {
        let html = String::from_utf8_lossy(&content);
        let with_reload = inject_reload_script(&html);
        Ok(build_html_response(with_reload))
    } else {
        Ok(build_response(StatusCode::OK, content))
    }
}

/// Check if a file is an HTML file
fn is_html_file(path: &Path) -> bool {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|ext| ext == HTML_EXTENSION)
        .unwrap_or(false)
}

/// Build a simple HTTP response
fn build_response<T: Into<Body>>(status: StatusCode, body: T) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(body.into())
        .unwrap()
}

/// Build an HTML HTTP response with proper content type
fn build_html_response(html: String) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(Body::from(html))
        .unwrap()
}

/// Get a human-readable message for a status code
fn status_message(status: StatusCode) -> &'static str {
    match status {
        StatusCode::NOT_FOUND => "404 Not Found",
        StatusCode::INTERNAL_SERVER_ERROR => "500 Internal Server Error",
        _ => "Error",
    }
}

// ============================================================================
// Live Reload Script Injection
// ============================================================================

/// Inject live reload script into HTML
fn inject_reload_script(html: &str) -> String {
    let script = create_reload_script();
    
    // Inject before </body> tag if present, otherwise append
    if let Some(pos) = html.rfind("</body>") {
        let mut result = html.to_string();
        result.insert_str(pos, &script);
        result
    } else {
        format!("{}{}", html, script)
    }
}

/// Create the live reload JavaScript
fn create_reload_script() -> String {
    format!(
        r#"
<script>
(function() {{
    'use strict';
    
    function checkReload() {{
        fetch('{endpoint}')
            .then(res => res.text())
            .then(data => {{
                if (data === 'reload') {{
                    console.log('🔄 Reloading page...');
                    location.reload();
                }}
            }})
            .catch(err => console.error('❌ Reload check failed:', err));
    }}
    
    setInterval(checkReload, {interval});
    console.log('✅ Live reload enabled');
}})();
</script>
"#,
        endpoint = RELOAD_ENDPOINT,
        interval = RELOAD_CHECK_INTERVAL_MS
    )
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_inject_reload_script_with_body_tag() {
        let html = "<html><body><h1>Test</h1></body></html>";
        let result = inject_reload_script(html);
        
        assert!(result.contains("<script>"));
        assert!(result.contains("checkReload"));
        assert!(result.contains("</body>"));
        
        // Script should be injected before </body>
        let script_pos = result.find("<script>").unwrap();
        let body_end_pos = result.find("</body>").unwrap();
        assert!(script_pos < body_end_pos);
    }
    
    #[test]
    fn test_inject_reload_script_without_body_tag() {
        let html = "<html><h1>Test</h1></html>";
        let result = inject_reload_script(html);
        
        assert!(result.contains("<script>"));
        assert!(result.contains("checkReload"));
    }
    
    #[test]
    fn test_is_html_file() {
        assert!(is_html_file(Path::new("index.html")));
        assert!(is_html_file(Path::new("path/to/page.html")));
        assert!(!is_html_file(Path::new("style.css")));
        assert!(!is_html_file(Path::new("script.js")));
        assert!(!is_html_file(Path::new("image.png")));
    }
    
    #[test]
    fn test_resolve_file_path() {
        let dest = PathBuf::from("/site");
        
        // Root path
        assert_eq!(
            resolve_file_path(&dest, "/"),
            PathBuf::from("/site/index.html")
        );
        
        // Empty path
        assert_eq!(
            resolve_file_path(&dest, ""),
            PathBuf::from("/site/index.html")
        );
        
        // Directory path
        assert_eq!(
            resolve_file_path(&dest, "/about/"),
            PathBuf::from("/site/about/index.html")
        );
        
        // File path
        assert_eq!(
            resolve_file_path(&dest, "/page.html"),
            PathBuf::from("/site/page.html")
        );
    }
    
    #[test]
    fn test_canonicalize_path() {
        let path = Path::new(".");
        let result = canonicalize_path(path);
        
        // Should return a valid path (either canonicalized or original)
        assert!(!result.as_os_str().is_empty());
    }
}
