# Agent Browser Hub

基于 Rust + [agent-browser](https://github.com/vercel-labs/agent-browser) 的浏览器自动化脚本中心。

**特性：**
- 🚀 单一二进制文件，无依赖
- 🎯 38+ 内置命令，覆盖 20+ 站点
- 📝 基于 YAML 的脚本定义
- 🔧 模板引擎和过滤器
- 📊 多种输出格式（JSON、YAML、表格、CSV、Markdown）
- 🌐 Web UI + REST API
- 🔐 JWT 认证

## 快速开始

### 安装

```bash
# AMD64
wget https://github.com/Xiechengqi/agent-browser-hub/releases/download/latest/agent-browser-hub-linux-amd64 -O agent-browser-hub && chmod +x agent-browser-hub

# ARM64
wget https://github.com/Xiechengqi/agent-browser-hub/releases/download/latest/agent-browser-hub-linux-arm64 -O agent-browser-hub && chmod +x agent-browser-hub
```

### CLI 使用

```bash
# 列出所有命令
agent-browser-hub list

# 运行命令（JSON 输出）
agent-browser-hub run hackernews/top --limit 10

# 表格输出
agent-browser-hub run hackernews/top --format table

# CSV 输出
agent-browser-hub run wikipedia/search --query rust --format csv

# Markdown 输出
agent-browser-hub run stackoverflow/search --query async --format md
```

### Web 服务器

```bash
agent-browser-hub serve              # http://localhost:3133
agent-browser-hub serve --port 8080  # 自定义端口
```

默认密码：`admin123`

## 可用命令

### HackerNews (7个)
- `hackernews/top` - 热门故事
- `hackernews/best` - 最佳故事
- `hackernews/new` - 最新故事
- `hackernews/ask` - Ask HN
- `hackernews/show` - Show HN
- `hackernews/jobs` - 招聘信息
- `hackernews/search` - 搜索故事

### Wikipedia (3个)
- `wikipedia/search` - 搜索文章
- `wikipedia/summary` - 获取文章摘要
- `wikipedia/random` - 随机文章

### StackOverflow (2个)
- `stackoverflow/search` - 搜索问题
- `stackoverflow/tags` - 浏览标签

### Medium (2个)
- `medium/search` - 搜索文章
- `medium/feed` - 最新动态

### DevTo (2个)
- `devto/search` - 搜索文章
- `devto/feed` - 最新动态

### Arxiv (2个)
- `arxiv/search` - 搜索论文
- `arxiv/paper` - 获取论文详情

### Lobsters (3个)
- `lobsters/hot` - 热门故事
- `lobsters/newest` - 最新故事
- `lobsters/search` - 搜索故事

### Bloomberg (3个)
- `bloomberg/markets` - 市场新闻
- `bloomberg/opinions` - 观点文章
- `bloomberg/economics` - 经济新闻

### Substack (2个)
- `substack/search` - 搜索新闻通讯
- `substack/feed` - 最新文章

### 其他
- `google/search` - Google 搜索
- `reuters/news` - 路透社新闻
- `bbc/news` - BBC 新闻
- `hf/top` - HuggingFace 热门模型
- `apple-podcasts/search` - 搜索播客
- `v2ex/daily` - V2EX 每日（需要 cookies）
- `weibo/hot` - 微博热搜（需要 cookies）
- `reddit/read` - Reddit 帖子（需要 cookies）
- `douban/search` - 豆瓣搜索（需要 cookies）
- `xueqiu/hot` - 雪球热门（需要 cookies）
- `sinafinance/news` - 新浪财经新闻（需要 cookies）
- `weread/search` - 微信读书搜索（需要 cookies）

## 输出格式

```bash
--format json      # JSON（默认，格式化输出）
--format yaml      # YAML
--format table     # ASCII 表格
--format csv       # CSV（带转义）
--format md        # Markdown 表格
```

## API 参考

### 认证

```bash
# 登录
curl -X POST http://localhost:3133/api/login \
  -H "Content-Type: application/json" \
  -d '{"password": "admin123"}'
```

### 执行命令

```bash
curl -X POST http://localhost:3133/api/execute/hackernews/top \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -d '{"limit": 10}'
```

### 列出命令

```bash
curl -H "Authorization: Bearer <token>" \
  http://localhost:3133/api/scripts
```

## 创建自定义脚本

在 `scripts/{站点}/{命令}.yaml` 创建 YAML 文件：

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

### 模板引擎

使用 `${{ expr }}` 表示动态值：

```yaml
# 变量访问
${{ args.query }}
${{ item.title }}
${{ data.0.name }}

# 过滤器
${{ args.query | upper }}
${{ item.title | truncate(50) }}
${{ items | join(', ') }}
${{ value | default('N/A') }}

# 算术运算
${{ index + 1 }}

# 回退值
${{ item.count || 0 }}
```

### 可用过滤器

- `default(val)` - 空值时的默认值
- `join(sep)` - 连接数组
- `upper` / `lower` - 大小写转换
- `trim` - 去除空白
- `truncate(n)` - 截断到 n 个字符
- `replace(old,new)` - 替换文本
- `length` - 获取长度
- `first` / `last` - 数组访问
- `json` - JSON 字符串化
- `urlencode` - URL 编码

### Pipeline 步骤

**浏览器操作：**
- `navigate: url` - 导航到 URL
- `evaluate: js` - 执行 JavaScript
- `click: selector` - 点击元素
- `type: {selector, text}` - 输入文本
- `wait: ms` - 等待毫秒
- `press: key` - 按键
- `scroll` - 向下滚动
- `snapshot` - 捕获页面快照

**数据转换：**
- `select: path` - 提取数据路径
- `map: {key: template}` - 转换数组
- `filter: expr` - 过滤数组
- `sort: key` - 按键排序
- `limit: n` - 取前 n 项

## 从源码构建

```bash
git clone https://github.com/Xiechengqi/agent-browser-hub.git
cd agent-browser-hub
cargo build --release
# 二进制文件：target/release/agent-browser-hub
```

## 架构

```
src/
├── core/
│   ├── browser.rs      # 30+ 浏览器操作
│   ├── strategy.rs     # 5 种认证策略
│   ├── template.rs     # 模板引擎
│   ├── pipeline.rs     # Pipeline 处理器
│   ├── executor.rs     # 脚本执行器
│   ├── validation.rs   # 参数验证
│   ├── output.rs       # 输出格式化
│   └── script.rs       # 数据模型
├── commands/
│   └── bilibili/       # 原生命令
├── registry.rs         # 命令注册表
└── server/mod.rs       # Web 服务器 + API

scripts/                # YAML 脚本（38+）
```

## 许可证

Apache-2.0

