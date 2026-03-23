use axum::{
    Router, Json,
    extract::{Path, State, Request},
    http::StatusCode,
    middleware::{self, Next},
    response::{Html, Response},
    routing::{get, post},
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

const JWT_SECRET: &str = "agent_browser_hub_jwt_secret";
const DEFAULT_PASSWORD: &str = "admin123";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_COMMIT: &str = env!("GIT_COMMIT");
const GIT_COMMIT_DATE: &str = env!("GIT_COMMIT_DATE");
const GIT_COMMIT_MSG: &str = env!("GIT_COMMIT_MSG");
const BUILD_TIME: &str = env!("BUILD_TIME");
const GITHUB_REPO: &str = "Xiechengqi/agent-browser-cli";

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Clone)]
pub struct AppState {
    pub password: Arc<Mutex<String>>,
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
    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.as_ref()))
}

fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &DecodingKey::from_secret(JWT_SECRET.as_ref()), &Validation::default())
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
        return Json(ApiResponse::error("Password incorrect"));
    }

    match generate_token() {
        Ok(token) => Json(ApiResponse::success("Login successful", LoginResponse { token })),
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

// ============================================================================
// Handlers - Scripts
// ============================================================================

async fn execute_script(
    Path((site, command)): Path<(String, String)>,
    Json(params): Json<HashMap<String, Value>>,
) -> Json<ApiResponse<Value>> {
    Json(ApiResponse::success(
        format!("Executed {}/{}", site, command),
        serde_json::json!({ "site": site, "command": command, "params": params }),
    ))
}

async fn list_scripts() -> Json<ApiResponse<Vec<Value>>> {
    let scripts = vec![
        serde_json::json!({"site": "google", "command": "search", "description": "Search Google"}),
        serde_json::json!({"site": "hackernews", "command": "top", "description": "Hacker News top stories"}),
    ];
    Json(ApiResponse::success("ok", scripts))
}

// ============================================================================
// HTML Pages
// ============================================================================

async fn page_login() -> Html<String> {
    Html(format!(r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Login - Agent Browser Hub</title>
<style>
*{{margin:0;padding:0;box-sizing:border-box}}
body{{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;background:#0f172a;color:#e2e8f0;display:flex;align-items:center;justify-content:center;min-height:100vh}}
.card{{background:#1e293b;border-radius:12px;padding:40px;width:380px;box-shadow:0 25px 50px rgba(0,0,0,.3)}}
h1{{font-size:24px;margin-bottom:8px;text-align:center}}
.subtitle{{color:#94a3b8;text-align:center;margin-bottom:32px;font-size:14px}}
label{{display:block;font-size:13px;color:#94a3b8;margin-bottom:6px}}
input{{width:100%;padding:10px 14px;border:1px solid #334155;border-radius:8px;background:#0f172a;color:#e2e8f0;font-size:14px;outline:none;transition:border .2s}}
input:focus{{border-color:#3b82f6}}
.btn{{width:100%;padding:10px;border:none;border-radius:8px;background:#3b82f6;color:#fff;font-size:15px;font-weight:600;cursor:pointer;margin-top:20px;transition:background .2s}}
.btn:hover{{background:#2563eb}}
.btn:disabled{{background:#334155;cursor:not-allowed}}
.error{{color:#f87171;font-size:13px;margin-top:12px;text-align:center;display:none}}
.version{{color:#475569;font-size:12px;text-align:center;margin-top:24px}}
.version a{{color:#3b82f6;text-decoration:none}}
</style>
</head>
<body>
<div class="card">
  <h1>Agent Browser Hub</h1>
  <p class="subtitle">Browser automation scripts hub</p>
  <form id="loginForm">
    <label for="password">Password</label>
    <input type="password" id="password" placeholder="Enter password" autofocus>
    <button type="submit" class="btn" id="submitBtn">Login</button>
    <p class="error" id="errorMsg"></p>
  </form>
  <p class="version">v{version} &middot; <a href="/about">Version Info</a></p>
</div>
<script>
const form=document.getElementById('loginForm');
const errorMsg=document.getElementById('errorMsg');
const submitBtn=document.getElementById('submitBtn');

form.addEventListener('submit',async(e)=>{{
  e.preventDefault();
  errorMsg.style.display='none';
  submitBtn.disabled=true;
  submitBtn.textContent='Logging in...';
  try{{
    const res=await fetch('/api/login',{{
      method:'POST',
      headers:{{'Content-Type':'application/json'}},
      body:JSON.stringify({{password:document.getElementById('password').value}})
    }});
    const data=await res.json();
    if(data.success){{
      localStorage.setItem('hub_token',data.data.token);
      window.location.href='/dashboard';
    }}else{{
      errorMsg.textContent=data.message;
      errorMsg.style.display='block';
    }}
  }}catch(err){{
    errorMsg.textContent='Network error';
    errorMsg.style.display='block';
  }}
  submitBtn.disabled=false;
  submitBtn.textContent='Login';
}});

if(localStorage.getItem('hub_token'))window.location.href='/dashboard';
</script>
</body>
</html>"##, version = VERSION))
}

async fn page_dashboard() -> Html<String> {
    Html(format!(r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Dashboard - Agent Browser Hub</title>
<style>
*{{margin:0;padding:0;box-sizing:border-box}}
body{{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;background:#0f172a;color:#e2e8f0}}
.topbar{{background:#1e293b;border-bottom:1px solid #334155;padding:12px 24px;display:flex;justify-content:space-between;align-items:center}}
.topbar h1{{font-size:18px}}
.topbar .actions{{display:flex;gap:12px;align-items:center}}
.topbar a,.topbar button{{color:#94a3b8;text-decoration:none;font-size:13px;background:none;border:none;cursor:pointer;padding:6px 12px;border-radius:6px;transition:all .2s}}
.topbar a:hover,.topbar button:hover{{color:#e2e8f0;background:#334155}}
.logout-btn{{color:#f87171!important}}
.logout-btn:hover{{background:#451a1a!important;color:#fca5a5!important}}
.container{{max-width:900px;margin:40px auto;padding:0 24px}}
.section{{background:#1e293b;border-radius:12px;padding:24px;margin-bottom:24px}}
.section h2{{font-size:16px;margin-bottom:16px;color:#94a3b8}}
.script-list{{list-style:none}}
.script-list li{{padding:12px 16px;border:1px solid #334155;border-radius:8px;margin-bottom:8px;display:flex;justify-content:space-between;align-items:center}}
.script-list li:hover{{border-color:#3b82f6}}
.script-name{{font-weight:600}}
.script-desc{{color:#94a3b8;font-size:13px}}
.badge{{background:#1e3a5f;color:#60a5fa;padding:2px 8px;border-radius:4px;font-size:12px}}
</style>
</head>
<body>
<div class="topbar">
  <h1>Agent Browser Hub</h1>
  <div class="actions">
    <a href="/about">Version Info</a>
    <a href="/settings">Settings</a>
    <button class="logout-btn" onclick="logout()">Logout</button>
  </div>
</div>
<div class="container">
  <div class="section">
    <h2>Scripts</h2>
    <ul class="script-list" id="scriptList">
      <li><span>Loading...</span></li>
    </ul>
  </div>
</div>
<script>
const token=localStorage.getItem('hub_token');
if(!token)window.location.href='/login';

function logout(){{
  localStorage.removeItem('hub_token');
  window.location.href='/login';
}}

async function apiFetch(url){{
  const res=await fetch(url,{{headers:{{'Authorization':'Bearer '+token}}}});
  if(res.status===401){{logout();return null;}}
  return res.json();
}}

(async()=>{{
  const data=await apiFetch('/api/scripts');
  if(!data)return;
  const list=document.getElementById('scriptList');
  if(data.success&&data.data.length>0){{
    list.innerHTML=data.data.map(s=>`
      <li>
        <div>
          <span class="script-name">${{s.site}}/${{s.command}}</span>
          <span class="script-desc"> - ${{s.description}}</span>
        </div>
        <span class="badge">${{s.site}}</span>
      </li>
    `).join('');
  }}else{{
    list.innerHTML='<li>No scripts found</li>';
  }}
}})();
</script>
</body>
</html>"##))
}

async fn page_about() -> Html<String> {
    Html(format!(r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Version Info - Agent Browser Hub</title>
<style>
*{{margin:0;padding:0;box-sizing:border-box}}
body{{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;background:#0f172a;color:#e2e8f0;display:flex;align-items:center;justify-content:center;min-height:100vh}}
.card{{background:#1e293b;border-radius:12px;padding:40px;width:520px;box-shadow:0 25px 50px rgba(0,0,0,.3)}}
h1{{font-size:22px;margin-bottom:24px;display:flex;align-items:center;gap:10px}}
.icon{{width:28px;height:28px;background:#3b82f6;border-radius:6px;display:flex;align-items:center;justify-content:center;font-size:14px}}
.info-grid{{display:grid;gap:16px}}
.info-row{{display:flex;justify-content:space-between;align-items:center;padding:12px 16px;background:#0f172a;border-radius:8px}}
.info-label{{color:#94a3b8;font-size:13px}}
.info-value{{font-size:14px;font-weight:500;word-break:break-all;text-align:right;max-width:60%}}
.info-value a{{color:#3b82f6;text-decoration:none}}
.info-value a:hover{{text-decoration:underline}}
.update-section{{margin-top:24px;padding:16px;background:#0f172a;border-radius:8px;text-align:center}}
.update-section .latest{{font-size:13px;color:#94a3b8;margin-bottom:12px}}
.update-section .latest span{{color:#34d399;font-weight:600}}
.btn{{padding:8px 20px;border:none;border-radius:6px;font-size:13px;font-weight:600;cursor:pointer;transition:all .2s}}
.btn-primary{{background:#3b82f6;color:#fff}}
.btn-primary:hover{{background:#2563eb}}
.btn-primary:disabled{{background:#334155;cursor:not-allowed}}
.btn-back{{background:#334155;color:#94a3b8;margin-right:8px}}
.btn-back:hover{{background:#475569;color:#e2e8f0}}
.actions{{margin-top:24px;display:flex;justify-content:center;gap:8px}}
.status{{font-size:13px;color:#94a3b8;margin-top:8px}}
</style>
</head>
<body>
<div class="card">
  <h1><span class="icon">i</span> Version Info</h1>
  <div class="info-grid">
    <div class="info-row">
      <span class="info-label">Version</span>
      <span class="info-value">v{version}</span>
    </div>
    <div class="info-row">
      <span class="info-label">Commit</span>
      <span class="info-value"><a href="https://github.com/{repo}/commit/{commit}" target="_blank">{commit}</a></span>
    </div>
    <div class="info-row">
      <span class="info-label">Commit Date</span>
      <span class="info-value">{commit_date}</span>
    </div>
    <div class="info-row">
      <span class="info-label">Commit Message</span>
      <span class="info-value">{commit_msg}</span>
    </div>
    <div class="info-row">
      <span class="info-label">Build Time</span>
      <span class="info-value">{build_time}</span>
    </div>
  </div>
  <div class="update-section">
    <p class="latest" id="latestInfo">Checking for updates...</p>
    <button class="btn btn-primary" id="upgradeBtn" style="display:none" onclick="doUpgrade()">Upgrade</button>
    <p class="status" id="upgradeStatus"></p>
  </div>
  <div class="actions">
    <button class="btn btn-back" onclick="history.back()">Back</button>
  </div>
</div>
<script>
const token=localStorage.getItem('hub_token');

async function apiFetch(url,opts={{}}){{
  const headers={{}};
  if(token)headers['Authorization']='Bearer '+token;
  return fetch(url,{{...opts,headers:{{...headers,...(opts.headers||{{}})}}}});
}}

(async()=>{{
  try{{
    const res=await fetch('/api/version');
    const data=await res.json();
    const info=document.getElementById('latestInfo');
    if(data.success&&data.data){{
      const v=data.data;
      if(v.latest){{
        if(v.latest!=='v'+'{version}'&&v.latest!=='{version}'){{
          info.innerHTML='Latest: <span style="color:#fbbf24">'+v.latest+'</span> (update available)';
          document.getElementById('upgradeBtn').style.display='inline-block';
        }}else{{
          info.innerHTML='Latest: <span>'+v.latest+'</span> (up to date)';
        }}
      }}else{{
        info.textContent='Could not check for updates';
      }}
    }}
  }}catch(e){{
    document.getElementById('latestInfo').textContent='Failed to check updates';
  }}
}})();

async function doUpgrade(){{
  if(!token){{window.location.href='/login';return;}}
  const btn=document.getElementById('upgradeBtn');
  const status=document.getElementById('upgradeStatus');
  btn.disabled=true;
  btn.textContent='Upgrading...';
  status.textContent='Downloading and installing...';
  try{{
    const res=await apiFetch('/api/upgrade',{{method:'POST'}});
    const data=await res.json();
    if(data.success){{
      status.style.color='#34d399';
      status.textContent='Upgrade complete! Restarting... Refreshing in 3s.';
      setTimeout(()=>window.location.reload(),3000);
    }}else{{
      status.style.color='#f87171';
      status.textContent='Upgrade failed: '+data.message;
      btn.disabled=false;
      btn.textContent='Retry Upgrade';
    }}
  }}catch(e){{
    status.style.color='#f87171';
    status.textContent='Network error: '+e.message;
    btn.disabled=false;
    btn.textContent='Retry Upgrade';
  }}
}}
</script>
</body>
</html>"##,
        version = VERSION,
        repo = GITHUB_REPO,
        commit = GIT_COMMIT,
        commit_date = GIT_COMMIT_DATE,
        commit_msg = GIT_COMMIT_MSG,
        build_time = BUILD_TIME,
    ))
}

async fn page_settings() -> Html<String> {
    Html(r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Settings - Agent Browser Hub</title>
<style>
*{margin:0;padding:0;box-sizing:border-box}
body{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;background:#0f172a;color:#e2e8f0;display:flex;align-items:center;justify-content:center;min-height:100vh}
.card{background:#1e293b;border-radius:12px;padding:40px;width:420px;box-shadow:0 25px 50px rgba(0,0,0,.3)}
h1{font-size:22px;margin-bottom:24px}
.section{margin-bottom:24px}
.section h2{font-size:15px;color:#94a3b8;margin-bottom:12px}
label{display:block;font-size:13px;color:#94a3b8;margin-bottom:6px}
input{width:100%;padding:10px 14px;border:1px solid #334155;border-radius:8px;background:#0f172a;color:#e2e8f0;font-size:14px;outline:none;transition:border .2s;margin-bottom:12px}
input:focus{border-color:#3b82f6}
.btn{padding:8px 20px;border:none;border-radius:6px;font-size:13px;font-weight:600;cursor:pointer;transition:all .2s}
.btn-primary{background:#3b82f6;color:#fff}
.btn-primary:hover{background:#2563eb}
.btn-back{background:#334155;color:#94a3b8}
.btn-back:hover{background:#475569;color:#e2e8f0}
.btn-danger{background:#7f1d1d;color:#fca5a5}
.btn-danger:hover{background:#991b1b}
.msg{font-size:13px;margin-top:8px}
.msg.ok{color:#34d399}
.msg.err{color:#f87171}
.actions{display:flex;justify-content:space-between;margin-top:24px}
hr{border:none;border-top:1px solid #334155;margin:24px 0}
</style>
</head>
<body>
<div class="card">
  <h1>Settings</h1>
  <div class="section">
    <h2>Change Password</h2>
    <label>New Password</label>
    <input type="password" id="newPassword" placeholder="At least 4 characters">
    <label>Confirm Password</label>
    <input type="password" id="confirmPassword" placeholder="Confirm new password">
    <button class="btn btn-primary" onclick="changePassword()">Update Password</button>
    <p class="msg" id="pwMsg"></p>
  </div>
  <hr>
  <div class="section">
    <h2>Account</h2>
    <button class="btn btn-danger" onclick="logout()">Logout</button>
  </div>
  <div class="actions">
    <button class="btn btn-back" onclick="history.back()">Back</button>
  </div>
</div>
<script>
const token=localStorage.getItem('hub_token');
if(!token)window.location.href='/login';

function logout(){
  localStorage.removeItem('hub_token');
  window.location.href='/login';
}

async function changePassword(){
  const pw=document.getElementById('newPassword').value;
  const confirm=document.getElementById('confirmPassword').value;
  const msg=document.getElementById('pwMsg');
  msg.className='msg';
  if(pw.length<4){msg.className='msg err';msg.textContent='Password must be at least 4 characters';return;}
  if(pw!==confirm){msg.className='msg err';msg.textContent='Passwords do not match';return;}
  try{
    const res=await fetch('/api/password',{
      method:'POST',
      headers:{'Content-Type':'application/json','Authorization':'Bearer '+token},
      body:JSON.stringify({password:pw})
    });
    const data=await res.json();
    if(data.success){
      msg.className='msg ok';
      msg.textContent='Password updated. Please login again.';
      setTimeout(()=>{localStorage.removeItem('hub_token');window.location.href='/login';},1500);
    }else{
      msg.className='msg err';msg.textContent=data.message;
    }
  }catch(e){msg.className='msg err';msg.textContent='Network error';}
}
</script>
</body>
</html>"##.to_string())
}

async fn page_root() -> Response {
    axum::response::Redirect::to("/login").into_response()
}

// ============================================================================
// Server
// ============================================================================

pub async fn start(port: u16) -> anyhow::Result<()> {
    let state = AppState {
        password: Arc::new(Mutex::new(DEFAULT_PASSWORD.to_string())),
    };

    // Public routes (no auth)
    let public_routes = Router::new()
        .route("/api/login", post(login))
        .route("/api/version", get(get_version))
        .route("/", get(page_root))
        .route("/login", get(page_login))
        .route("/about", get(page_about));

    // Protected routes (require auth)
    let protected_routes = Router::new()
        .route("/api/password", post(update_password))
        .route("/api/upgrade", post(upgrade))
        .route("/api/scripts", get(list_scripts))
        .route("/api/execute/{site}/{command}", post(execute_script))
        .route_layer(middleware::from_fn(auth_middleware));

    // HTML pages (client-side auth check via JS)
    let page_routes = Router::new()
        .route("/dashboard", get(page_dashboard))
        .route("/settings", get(page_settings));

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(page_routes)
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    eprintln!("Server running on http://{}", addr);
    eprintln!("Default password: {}", DEFAULT_PASSWORD);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
