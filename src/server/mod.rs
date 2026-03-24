use axum::{
    Router, Json,
    extract::{Path, Query, State, Request},
    http::{StatusCode, header},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, LazyLock};
use tokio::sync::Mutex;

#[cfg(feature = "embed-frontend")]
use rust_embed::RustEmbed;

#[cfg(feature = "embed-frontend")]
#[derive(RustEmbed)]
#[folder = "web/out"]
struct Assets;

static JWT_SECRET: LazyLock<String> = LazyLock::new(|| {
    std::env::var("ABH_JWT_SECRET").unwrap_or_else(|_| "agent_browser_hub_jwt_secret".to_string())
});
const DEFAULT_PASSWORD: &str = "admin123";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_COMMIT: &str = env!("GIT_COMMIT");
const GIT_COMMIT_DATE: &str = env!("GIT_COMMIT_DATE");
const GIT_COMMIT_MSG: &str = env!("GIT_COMMIT_MSG");
const BUILD_TIME: &str = env!("BUILD_TIME");

use crate::GITHUB_REPO;

// ============================================================================
// In-Memory Log Buffer
// ============================================================================

const MAX_LOG_LINES: usize = 500;

#[derive(Clone, Serialize)]
struct LogEntry {
    time: String,
    level: String,
    message: String,
}

#[derive(Clone, Default)]
pub struct LogBuffer {
    entries: Arc<Mutex<VecDeque<LogEntry>>>,
}

impl LogBuffer {
    async fn push(&self, level: &str, message: String) {
        let entry = LogEntry {
            time: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            level: level.to_string(),
            message: message.clone(),
        };
        eprintln!("[{}] [{}] {}", entry.time, level, message);
        let mut entries = self.entries.lock().await;
        if entries.len() >= MAX_LOG_LINES {
            entries.pop_front();
        }
        entries.push_back(entry);
    }

    async fn get_entries(&self, limit: usize) -> Vec<LogEntry> {
        let entries = self.entries.lock().await;
        let start = if entries.len() > limit { entries.len() - limit } else { 0 };
        entries.iter().skip(start).cloned().collect()
    }
}

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Clone)]
pub struct AppState {
    pub password: Arc<Mutex<String>>,
    pub logs: LogBuffer,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    fn success(message: impl Into<String>, data: T) -> Self {
        Self { success: true, message: message.into(), data: Some(data) }
    }

    fn success_no_data(message: impl Into<String>) -> Self {
        Self { success: true, message: message.into(), data: None }
    }

    fn error(message: impl Into<String>) -> Self {
        Self { success: false, message: message.into(), data: None }
    }
}

#[derive(Deserialize)]
struct LoginRequest {
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[derive(Deserialize)]
struct PasswordChangeRequest {
    password: String,
}

#[derive(Serialize)]
struct VersionInfo {
    current: String,
    latest: Option<String>,
    commit: String,
    commit_date: String,
    commit_message: String,
    build_time: String,
}

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

// ============================================================================
// JWT
// ============================================================================

fn generate_token() -> Result<String, jsonwebtoken::errors::Error> {
    let exp = Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims { sub: "admin".to_string(), exp };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.as_bytes()))
}

fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &DecodingKey::from_secret(JWT_SECRET.as_bytes()), &Validation::default())
        .map(|data| data.claims)
}

// ============================================================================
// Auth Middleware
// ============================================================================

async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get("Authorization").and_then(|h| h.to_str().ok());

    if let Some(auth) = auth_header {
        if let Some(token) = auth.strip_prefix("Bearer ") {
            if verify_token(token).is_ok() {
                return Ok(next.run(req).await);
            }
        }
    }

    if let Some(query) = req.uri().query() {
        for part in query.split('&') {
            if let Some(value) = part.strip_prefix("token=") {
                if !value.is_empty() && verify_token(value).is_ok() {
                    return Ok(next.run(req).await);
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

// ============================================================================
// Handlers - Auth
// ============================================================================

async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Json<ApiResponse<LoginResponse>> {
    let password = state.password.lock().await;

    if req.password != *password {
        state.logs.push("WARN", "Login failed: incorrect password".into()).await;
        return Json(ApiResponse::error("Password incorrect"));
    }

    match generate_token() {
        Ok(token) => {
            state.logs.push("INFO", "Login successful".into()).await;
            Json(ApiResponse::success("Login successful", LoginResponse { token }))
        }
        Err(_) => Json(ApiResponse::error("Failed to generate token")),
    }
}

async fn update_password(
    State(state): State<AppState>,
    Json(req): Json<PasswordChangeRequest>,
) -> Json<ApiResponse<()>> {
    let password = req.password.trim().to_string();
    if password.len() < 4 {
        return Json(ApiResponse::error("Password must be at least 4 characters"));
    }

    let mut current = state.password.lock().await;
    *current = password;

    state.logs.push("INFO", "Password updated".into()).await;
    Json(ApiResponse::success_no_data("Password updated"))
}

// ============================================================================
// Handlers - Version
// ============================================================================

async fn get_version() -> Json<ApiResponse<VersionInfo>> {
    let current = format!("v{}", VERSION);

    let latest = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
    {
        Ok(client) => {
            match client
                .get(format!("https://api.github.com/repos/{}/releases/latest", GITHUB_REPO))
                .header("User-Agent", "agent-browser-hub")
                .send()
                .await
            {
                Ok(r) => r.json::<GitHubRelease>().await.ok().map(|rel| rel.tag_name),
                Err(_) => None,
            }
        }
        Err(_) => None,
    };

    Json(ApiResponse::success("ok", VersionInfo {
        current,
        latest,
        commit: GIT_COMMIT.to_string(),
        commit_date: GIT_COMMIT_DATE.to_string(),
        commit_message: GIT_COMMIT_MSG.to_string(),
        build_time: BUILD_TIME.to_string(),
    }))
}

// ============================================================================
// Handlers - Upgrade
// ============================================================================

async fn upgrade() -> Json<ApiResponse<String>> {
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
    {
        Ok(c) => c,
        Err(e) => return Json(ApiResponse::error(format!("Failed to create HTTP client: {}", e))),
    };

    let release: GitHubRelease = match client
        .get(format!("https://api.github.com/repos/{}/releases/latest", GITHUB_REPO))
        .header("User-Agent", "agent-browser-hub")
        .send()
        .await
    {
        Ok(r) => match r.json().await {
            Ok(rel) => rel,
            Err(e) => return Json(ApiResponse::error(format!("Failed to parse release info: {}", e))),
        },
        Err(e) => return Json(ApiResponse::error(format!("Failed to fetch release info: {}", e))),
    };

    let asset_name = if cfg!(target_arch = "x86_64") {
        "agent-browser-hub-linux-amd64"
    } else if cfg!(target_arch = "aarch64") {
        "agent-browser-hub-linux-arm64"
    } else {
        return Json(ApiResponse::error("Unsupported architecture"));
    };

    let download_url = match release.assets.iter().find(|a| a.name == asset_name) {
        Some(a) => a.browser_download_url.clone(),
        None => return Json(ApiResponse::error("No binary found for current architecture")),
    };

    eprintln!("[upgrade] Downloading from: {}", download_url);
    let download_client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()
    {
        Ok(c) => c,
        Err(e) => return Json(ApiResponse::error(format!("Failed to create download client: {}", e))),
    };

    let binary_data = match download_client
        .get(&download_url)
        .header("User-Agent", "agent-browser-hub")
        .send()
        .await
    {
        Ok(r) => {
            if !r.status().is_success() {
                return Json(ApiResponse::error(format!("Download failed: {}", r.status())));
            }
            match r.bytes().await {
                Ok(b) => b,
                Err(e) => return Json(ApiResponse::error(format!("Failed to download binary: {}", e))),
            }
        }
        Err(e) => return Json(ApiResponse::error(format!("Failed to download: {}", e))),
    };

    let temp_path = "/tmp/agent-browser-hub-new";
    if let Err(e) = std::fs::write(temp_path, &binary_data) {
        return Json(ApiResponse::error(format!("Failed to write temp file: {}", e)));
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Err(e) = std::fs::set_permissions(temp_path, std::fs::Permissions::from_mode(0o755)) {
            return Json(ApiResponse::error(format!("Failed to set permissions: {}", e)));
        }
    }

    let verify = tokio::process::Command::new(temp_path).arg("--help").output().await;
    if verify.is_err() {
        let _ = std::fs::remove_file(temp_path);
        return Json(ApiResponse::error("New binary verification failed"));
    }

    let current_exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => return Json(ApiResponse::error(format!("Failed to get current exe: {}", e))),
    };

    let backup_path = format!("{}.bak", current_exe.display());
    if let Err(e) = std::fs::copy(&current_exe, &backup_path) {
        return Json(ApiResponse::error(format!("Failed to backup: {}", e)));
    }

    if let Err(e) = std::fs::remove_file(&current_exe) {
        return Json(ApiResponse::error(format!("Failed to remove old binary: {}", e)));
    }

    if let Err(e) = std::fs::copy(temp_path, &current_exe) {
        let _ = std::fs::copy(&backup_path, &current_exe);
        return Json(ApiResponse::error(format!("Failed to copy new binary: {}", e)));
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Err(e) = std::fs::set_permissions(&current_exe, std::fs::Permissions::from_mode(0o755)) {
            let _ = std::fs::remove_file(&current_exe);
            let _ = std::fs::copy(&backup_path, &current_exe);
            return Json(ApiResponse::error(format!("Failed to set permissions: {}", e)));
        }
    }

    let _ = std::fs::remove_file(temp_path);
    let new_version = release.tag_name.clone();
    eprintln!("[upgrade] Upgrade successful! Restarting...");

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            let args: Vec<String> = std::env::args().collect();
            let err = std::process::Command::new(&current_exe).args(&args[1..]).exec();
            eprintln!("[upgrade] Failed to exec new binary: {}", err);

            if std::fs::remove_file(&current_exe).is_ok() {
                if std::fs::copy(&backup_path, &current_exe).is_ok() {
                    use std::os::unix::fs::PermissionsExt;
                    let _ = std::fs::set_permissions(&current_exe, std::fs::Permissions::from_mode(0o755));
                    let _ = std::process::Command::new(&current_exe).args(&args[1..]).exec();
                }
            }
            std::process::exit(1);
        }
    });

    Json(ApiResponse::success("Upgrade complete, restarting...", new_version))
}

async fn upgrade_component(
    State(state): State<AppState>,
    Path(component): Path<String>,
) -> Json<ApiResponse<String>> {
    match component.as_str() {
        "agent-browser-hub" => {
            state.logs.push("INFO", "Upgrading agent-browser-hub...".into()).await;
            upgrade().await
        }
        "agent-browser" => {
            state.logs.push("INFO", "Upgrading agent-browser...".into()).await;
            upgrade_agent_browser(state).await
        }
        _ => Json(ApiResponse::error(format!("Unknown component: {}", component))),
    }
}

async fn upgrade_agent_browser(state: AppState) -> Json<ApiResponse<String>> {
    let download_url = "https://github.com/Xiechengqi/agent-browser/releases/download/latest/agent-browser-linux-x64";

    state.logs.push("INFO", format!("Downloading agent-browser from: {}", download_url)).await;

    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            state.logs.push("ERROR", format!("HTTP client error: {}", e)).await;
            return Json(ApiResponse::error(format!("HTTP client error: {}", e)));
        }
    };

    let binary_data = match client
        .get(download_url)
        .header("User-Agent", "agent-browser-hub")
        .send()
        .await
    {
        Ok(r) => {
            if !r.status().is_success() {
                let msg = format!("Download failed: {}", r.status());
                state.logs.push("ERROR", msg.clone()).await;
                return Json(ApiResponse::error(msg));
            }
            match r.bytes().await {
                Ok(b) => b,
                Err(e) => {
                    state.logs.push("ERROR", format!("Download error: {}", e)).await;
                    return Json(ApiResponse::error(format!("Download error: {}", e)));
                }
            }
        }
        Err(e) => {
            state.logs.push("ERROR", format!("Request error: {}", e)).await;
            return Json(ApiResponse::error(format!("Request error: {}", e)));
        }
    };

    state.logs.push("INFO", format!("Downloaded {} bytes", binary_data.len())).await;

    // Find agent-browser binary path
    let ab_path = which_agent_browser();
    let temp_path = "/tmp/agent-browser-new";

    if let Err(e) = std::fs::write(temp_path, &binary_data) {
        state.logs.push("ERROR", format!("Write temp file failed: {}", e)).await;
        return Json(ApiResponse::error(format!("Write failed: {}", e)));
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(temp_path, std::fs::Permissions::from_mode(0o755));
    }

    // Backup and replace
    let backup_path = format!("{}.bak", ab_path);
    if std::path::Path::new(&ab_path).exists() {
        if let Err(e) = std::fs::copy(&ab_path, &backup_path) {
            state.logs.push("WARN", format!("Backup failed: {}", e)).await;
        }
    }

    if let Err(e) = std::fs::copy(temp_path, &ab_path) {
        state.logs.push("ERROR", format!("Replace binary failed: {}", e)).await;
        let _ = std::fs::copy(&backup_path, &ab_path);
        return Json(ApiResponse::error(format!("Replace failed: {}", e)));
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&ab_path, std::fs::Permissions::from_mode(0o755));
    }

    let _ = std::fs::remove_file(temp_path);
    state.logs.push("INFO", "agent-browser upgraded successfully".into()).await;
    Json(ApiResponse::success("agent-browser upgraded", "ok".into()))
}

fn which_agent_browser() -> String {
    if let Ok(output) = std::process::Command::new("which").arg("agent-browser").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return path;
            }
        }
    }
    "/usr/local/bin/agent-browser".to_string()
}

// ============================================================================
// Handlers - Scripts
// ============================================================================

async fn execute_script(
    State(state): State<AppState>,
    Path((site, name)): Path<(String, String)>,
    Json(req): Json<HashMap<String, Value>>,
) -> Json<Value> {
    state.logs.push("INFO", format!("Execute: {}/{}", site, name)).await;

    if !site.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        state.logs.push("ERROR", format!("Invalid script path: {}/{}", site, name)).await;
        return Json(serde_json::json!({"success": false, "error": "Invalid script path"}));
    }

    let script_path = format!("scripts/{}/{}.yaml", site, name);
    if !std::path::Path::new(&script_path).exists() {
        state.logs.push("ERROR", format!("Script not found: {}", script_path)).await;
        return Json(serde_json::json!({"success": false, "error": format!("Script not found: {}/{}", site, name)}));
    }

    let yaml_content = match std::fs::read_to_string(&script_path) {
        Ok(c) => c,
        Err(e) => {
            state.logs.push("ERROR", format!("Read failed: {}", e)).await;
            return Json(serde_json::json!({"success": false, "error": format!("Read failed: {}", e)}));
        }
    };

    let script: crate::core::Script = match serde_yaml::from_str(&yaml_content) {
        Ok(s) => s,
        Err(e) => {
            state.logs.push("ERROR", format!("Parse YAML failed: {}", e)).await;
            return Json(serde_json::json!({"success": false, "error": format!("Parse failed: {}", e)}));
        }
    };

    let executor = match crate::core::Executor::new().await {
        Ok(e) => e,
        Err(e) => {
            state.logs.push("ERROR", format!("Executor init failed: {}", e)).await;
            return Json(serde_json::json!({"success": false, "error": format!("Executor failed: {}", e)}));
        }
    };

    let params = req.get("params").and_then(|v| v.as_object()).map(|m| {
        m.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }).unwrap_or_default();

    state.logs.push("DEBUG", format!("Params: {:?}", params)).await;

    match executor.execute(&script, params).await {
        Ok(result) => {
            state.logs.push("INFO", format!("Execute OK: {}/{}", site, name)).await;
            Json(serde_json::json!({"success": true, "data": result}))
        }
        Err(e) => {
            state.logs.push("ERROR", format!("Execute failed {}/{}: {}", site, name, e)).await;
            Json(serde_json::json!({"success": false, "error": format!("Execution failed: {}", e)}))
        }
    }
}

async fn list_scripts() -> Json<Vec<Value>> {
    let mut scripts = Vec::new();
    let scripts_dir = std::path::Path::new("scripts");

    if let Ok(sites) = std::fs::read_dir(scripts_dir) {
        for site_entry in sites.flatten() {
            if !site_entry.path().is_dir() { continue; }
            let site = site_entry.file_name().to_string_lossy().to_string();

            if let Ok(files) = std::fs::read_dir(site_entry.path()) {
                for file_entry in files.flatten() {
                    let path = file_entry.path();
                    if path.extension().and_then(|e| e.to_str()) != Some("yaml") { continue; }

                    let name = path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string();

                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if let Ok(yaml) = serde_yaml::from_str::<Value>(&content) {
                            let description = yaml["meta"]["description"].as_str()
                                .or_else(|| yaml["description"].as_str())
                                .unwrap_or("")
                                .to_string();
                            let strategy = yaml["config"]["strategy"].as_str()
                                .or_else(|| yaml["strategy"].as_str())
                                .unwrap_or("PUBLIC")
                                .to_string();

                            // Parse params from "args" (map) or "params" (array)
                            let mut params = Vec::new();
                            if let Some(args) = yaml["args"].as_object() {
                                for (arg_name, arg_val) in args {
                                    params.push(serde_json::json!({
                                        "name": arg_name,
                                        "type": arg_val["type"].as_str().unwrap_or("string"),
                                        "required": arg_val["required"].as_bool().unwrap_or(false),
                                        "default": arg_val.get("default"),
                                        "description": arg_val["description"].as_str().unwrap_or(""),
                                    }));
                                }
                            } else if let Some(p_arr) = yaml["params"].as_array() {
                                for p in p_arr {
                                    params.push(serde_json::json!({
                                        "name": p["name"].as_str().unwrap_or(""),
                                        "type": p["type"].as_str().unwrap_or("string"),
                                        "required": p["required"].as_bool().unwrap_or(false),
                                        "default": p.get("default"),
                                        "description": p["description"].as_str().unwrap_or(""),
                                    }));
                                }
                            }

                            scripts.push(serde_json::json!({
                                "site": site,
                                "name": name,
                                "description": description,
                                "strategy": strategy,
                                "params": params,
                            }));
                        }
                    }
                }
            }
        }
    }

    scripts.sort_by(|a, b| {
        let key = |v: &Value| format!("{}/{}", v["site"].as_str().unwrap_or(""), v["name"].as_str().unwrap_or(""));
        key(a).cmp(&key(b))
    });

    Json(scripts)
}

// ============================================================================
// Handlers - Logs
// ============================================================================

#[derive(Deserialize)]
struct LogsQuery {
    limit: Option<usize>,
}

async fn get_logs(
    State(state): State<AppState>,
    Query(query): Query<LogsQuery>,
) -> Json<Vec<LogEntry>> {
    let limit = query.limit.unwrap_or(200);
    let entries = state.logs.get_entries(limit).await;
    Json(entries)
}

// ============================================================================
// Static File Serving
// ============================================================================

async fn serve_static(path: &str) -> Response {
    #[cfg(feature = "embed-frontend")]
    {
        let path = path.trim_start_matches('/');
        let path = if path.is_empty() { "index.html" } else { path };
        match Assets::get(path) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => {
                // SPA fallback: non-file paths serve index.html
                if !path.contains('.') {
                    if let Some(index) = Assets::get("index.html") {
                        let mime = mime_guess::from_path("index.html").first_or_octet_stream();
                        return ([(header::CONTENT_TYPE, mime.as_ref())], index.data).into_response();
                    }
                }
                StatusCode::NOT_FOUND.into_response()
            }
        }
    }
    #[cfg(not(feature = "embed-frontend"))]
    {
        let _ = path;
        StatusCode::NOT_FOUND.into_response()
    }
}

async fn static_handler(req: Request) -> Response {
    serve_static(req.uri().path()).await
}

// ============================================================================
// Server
// ============================================================================

pub async fn start(port: u16) -> anyhow::Result<()> {
    let state = AppState {
        password: Arc::new(Mutex::new(DEFAULT_PASSWORD.to_string())),
        logs: LogBuffer::default(),
    };

    state.logs.push("INFO", format!("Server starting on port {}", port)).await;

    // Public routes (no auth)
    let public_routes = Router::new()
        .route("/api/login", post(login))
        .route("/api/version", get(get_version))
        .route("/api/commands", get(list_scripts));

    // Protected routes (require auth)
    let protected_routes = Router::new()
        .route("/api/password", post(update_password))
        .route("/api/upgrade", post(upgrade))
        .route("/api/upgrade/{component}", post(upgrade_component))
        .route("/api/logs", get(get_logs))
        .route("/api/execute/{site}/{command}", post(execute_script))
        .route_layer(middleware::from_fn(auth_middleware));

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
        .fallback(static_handler);

    let addr = format!("0.0.0.0:{}", port);
    eprintln!("Server running on http://{}", addr);
    eprintln!("Default password: {}", DEFAULT_PASSWORD);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
