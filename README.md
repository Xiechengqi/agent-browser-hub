# Agent Browser Hub

Browser automation scripts hub. Single binary, built with Rust + [agent-browser](https://github.com/vercel-labs/agent-browser).

## Install

### Quick Install

```bash
# AMD64
wget https://github.com/Xiechengqi/agent-browser-cli/releases/download/latest/agent-browser-hub-linux-amd64 -O agent-browser-hub && chmod +x agent-browser-hub

# ARM64
wget https://github.com/Xiechengqi/agent-browser-cli/releases/download/latest/agent-browser-hub-linux-arm64 -O agent-browser-hub && chmod +x agent-browser-hub
```

### Build from Source

```bash
git clone https://github.com/Xiechengqi/agent-browser-cli.git
cd agent-browser-cli
./build.sh
# Binary: target/release/agent-browser-hub
```

## Usage

### Web Server

```bash
agent-browser-hub serve              # http://localhost:3133
agent-browser-hub serve --port 8080  # custom port
```

Default password: `admin123`

### CLI

```bash
agent-browser-hub list       # list scripts
agent-browser-hub version    # show version
agent-browser-hub upgrade    # upgrade to latest release
```

### API

All protected endpoints require `Authorization: Bearer <token>` header.

```bash
# Login
curl -X POST http://localhost:3133/api/login \
  -H "Content-Type: application/json" \
  -d '{"password": "admin123"}'

# Execute script
curl -X POST http://localhost:3133/api/execute/google/search \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -d '{"keyword": "rust", "limit": 10}'

# List scripts
curl -H "Authorization: Bearer <token>" http://localhost:3133/api/scripts

# Version info
curl http://localhost:3133/api/version

# Change password
curl -X POST http://localhost:3133/api/password \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -d '{"password": "newpass"}'

# Upgrade
curl -X POST http://localhost:3133/api/upgrade \
  -H "Authorization: Bearer <token>"
```

## Web Pages

| Route | Description |
|-------|-------------|
| `/login` | Login page |
| `/dashboard` | Script list, logout |
| `/about` | Version info, upgrade |
| `/settings` | Change password, logout |

## API Endpoints

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | `/api/login` | No | Login, returns JWT |
| GET | `/api/version` | No | Version and build info |
| POST | `/api/password` | Yes | Change password |
| POST | `/api/upgrade` | Yes | Upgrade from GitHub release |
| GET | `/api/scripts` | Yes | List scripts |
| POST | `/api/execute/{site}/{command}` | Yes | Execute script |

## Project Structure

```
├── build.sh                           # Local build
├── build-ci.sh                        # CI cross-compilation
├── build.rs                           # Compile-time git info injection
├── .github/workflows/build-release.yml
├── scripts/
│   └── google/search.yaml             # Script definitions
└── src/
    ├── main.rs                        # Entry, CLI commands, CLI upgrade
    ├── lib.rs
    ├── cli/mod.rs                     # CLI argument parsing
    ├── core/
    │   ├── browser.rs                 # agent-browser wrapper
    │   ├── executor.rs                # Script execution engine
    │   └── script.rs                  # Data models
    └── server/mod.rs                  # Web server, API, auth, HTML pages
```

## License

Apache-2.0
