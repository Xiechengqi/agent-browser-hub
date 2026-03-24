# Agent Browser Hub

Browser automation scripts hub powered by Rust + [agent-browser](https://github.com/vercel-labs/agent-browser).

**Features:**
- 🚀 Single binary, no dependencies
- 🎯 260+ built-in commands across 44 sites
- 📦 Workflow-package-first command metadata
- 📝 YAML retained as a compatibility execution asset
- 🔧 Template engine with filters
- 📊 Multiple output formats (JSON, YAML, Table, CSV, Markdown)
- 🌐 Web UI + REST API
- 🔐 JWT authentication

## Quick Start

### Install

```bash
# AMD64
wget https://github.com/Xiechengqi/agent-browser-hub/releases/download/latest/agent-browser-hub-linux-amd64 -O agent-browser-hub && chmod +x agent-browser-hub

# ARM64
wget https://github.com/Xiechengqi/agent-browser-hub/releases/download/latest/agent-browser-hub-linux-arm64 -O agent-browser-hub && chmod +x agent-browser-hub
```

### CLI Usage

```bash
# List all commands
agent-browser-hub list

# Run command (JSON output)
agent-browser-hub run hackernews/top --limit 10

# Table output
agent-browser-hub run hackernews/top --format table

# CSV output
agent-browser-hub run wikipedia/search --query rust --format csv

# Markdown output
agent-browser-hub run stackoverflow/search --query async --format md
```

### Web Server

```bash
agent-browser-hub serve              # http://localhost:3133
agent-browser-hub serve --port 8080  # custom port
```

Default password: `admin123`

## Available Commands

### HackerNews (7)
- `hackernews/top` - Top stories
- `hackernews/best` - Best stories
- `hackernews/new` - Newest stories
- `hackernews/ask` - Ask HN
- `hackernews/show` - Show HN
- `hackernews/jobs` - Jobs
- `hackernews/search` - Search stories

### Wikipedia (3)
- `wikipedia/search` - Search articles
- `wikipedia/summary` - Get article summary
- `wikipedia/random` - Random article

### StackOverflow (2)
- `stackoverflow/search` - Search questions
- `stackoverflow/tags` - Browse tags

### Medium (2)
- `medium/search` - Search articles
- `medium/feed` - Latest feed

### DevTo (2)
- `devto/search` - Search articles
- `devto/feed` - Latest feed

### Arxiv (2)
- `arxiv/search` - Search papers
- `arxiv/paper` - Get paper details

### Lobsters (3)
- `lobsters/hot` - Hot stories
- `lobsters/newest` - Newest stories
- `lobsters/search` - Search stories

### Bloomberg (3)
- `bloomberg/markets` - Markets news
- `bloomberg/opinions` - Opinion articles
- `bloomberg/economics` - Economics news

### Substack (2)
- `substack/search` - Search newsletters
- `substack/feed` - Latest posts

### Others
- `google/search` - Google search
- `reuters/news` - Reuters news
- `bbc/news` - BBC news
- `hf/top` - HuggingFace top models
- `apple-podcasts/search` - Search podcasts
- `v2ex/daily` - V2EX daily (requires cookies)
- `weibo/hot` - Weibo hot topics (requires cookies)
- `reddit/read` - Reddit posts (requires cookies)
- `douban/search` - Douban search (requires cookies)
- `xueqiu/hot` - Xueqiu hot (requires cookies)
- `sinafinance/news` - Sina Finance news (requires cookies)
- `weread/search` - WeRead search (requires cookies)

## Output Formats

```bash
--format json      # JSON (default, pretty-printed)
--format yaml      # YAML
--format table     # ASCII table
--format csv       # CSV with proper escaping
--format md        # Markdown table
```

## API Reference

### Authentication

```bash
# Login
curl -X POST http://localhost:3133/api/login \
  -H "Content-Type: application/json" \
  -d '{"password": "admin123"}'
```

### Execute Command

```bash
curl -X POST http://localhost:3133/api/execute/hackernews/top \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -d '{"limit": 10}'
```

### List Commands

```bash
curl -H "Authorization: Bearer <token>" \
  http://localhost:3133/api/commands
```

## Creating Custom Commands

New commands must start in `workflows/{site}/commands/{command}.toml`.

During migration, the execution asset may still point to `scripts/{site}/{command}.yaml`:

```toml
name = "mycommand"
description = "My custom command"
strategy = "PUBLIC"

[[params]]
name = "query"
type = "string"
required = true
positional = true

[execution]
entry = "pipeline"
pipeline = "../../scripts/mysite/mycommand.yaml"
```

Example YAML execution asset:

```yaml
site: mysite
name: mycommand
strategy: PUBLIC
browser: false
args:
  query:
    type: string
    required: true
  limit:
    type: int
    default: 10

pipeline:
  - navigate: https://example.com/search?q=${{ args.query }}
  - wait: 2000
  - evaluate: |
      (() => {
        const items = [];
        document.querySelectorAll('.result').forEach(el => {
          items.push({
            title: el.textContent.trim()
          });
        });
        return items;
      })()
  - limit: ${{ args.limit }}
```

See [docs/workflow-governance.md](docs/workflow-governance.md) and [docs/workflow-package-spec.md](docs/workflow-package-spec.md) for the migration rules and package shape.

## Pre-Release Check

```bash
python3 tools/validate_workflows.py
```

See [docs/release-checklist.md](docs/release-checklist.md) for the release checklist.

### Template Engine

Use `${{ expr }}` for dynamic values:

```yaml
# Variable access
${{ args.query }}
${{ item.title }}
${{ data.0.name }}

# Filters
${{ args.query | upper }}
${{ item.title | truncate(50) }}
${{ items | join(', ') }}
${{ value | default('N/A') }}

# Arithmetic
${{ index + 1 }}

# Fallback
${{ item.count || 0 }}
```

### Available Filters

- `default(val)` - Default value if empty
- `join(sep)` - Join array
- `upper` / `lower` - Case conversion
- `trim` - Trim whitespace
- `truncate(n)` - Truncate to n chars
- `replace(old,new)` - Replace text
- `length` - Get length
- `first` / `last` - Array access
- `json` - JSON stringify
- `urlencode` - URL encode

### Pipeline Steps

**Browser:**
- `navigate: url` - Navigate to URL
- `evaluate: js` - Execute JavaScript
- `click: selector` - Click element
- `type: {selector, text}` - Type text
- `wait: ms` - Wait milliseconds
- `press: key` - Press key
- `scroll` - Scroll down
- `snapshot` - Capture page snapshot

**Transform:**
- `select: path` - Extract data path
- `map: {key: template}` - Transform array
- `filter: expr` - Filter array
- `sort: key` - Sort by key
- `limit: n` - Take first n items

## Build from Source

```bash
git clone https://github.com/Xiechengqi/agent-browser-hub.git
cd agent-browser-hub
cargo build --release
# Binary: target/release/agent-browser-hub
```

## Architecture

```
src/
├── core/
│   ├── browser.rs      # 30+ browser operations
│   ├── strategy.rs     # 5 auth strategies
│   ├── template.rs     # Template engine
│   ├── pipeline.rs     # Pipeline processor
│   ├── executor.rs     # Script executor
│   ├── validation.rs   # Param validation
│   ├── output.rs       # Output formatters
│   └── script.rs       # Data models
├── commands/
│   └── bilibili/       # Native commands
├── registry.rs         # Command registry
└── server/mod.rs       # Web server + API

scripts/                # compatibility execution assets
```

## License

Apache-2.0
