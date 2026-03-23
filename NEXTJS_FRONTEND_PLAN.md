# Next.js + Tailwind 前端重构方案

## 架构

### 后端（Rust）
- 保持现有 API 不变
- 移除内嵌 HTML
- 纯 REST API 服务

### 前端（Next.js）
- 独立的 Next.js 应用
- Tailwind CSS 样式
- TypeScript
- 调用后端 API

## 目录结构

```
agent-browser-hub/
├── src/                    # Rust 后端
│   └── server/mod.rs       # 移除 HTML，保留 API
├── web/                    # Next.js 前端
│   ├── app/
│   │   ├── layout.tsx
│   │   ├── page.tsx        # 首页
│   │   ├── login/
│   │   ├── dashboard/
│   │   └── api/            # API 路由（可选）
│   ├── components/
│   │   ├── CommandCard.tsx
│   │   ├── ExecuteDialog.tsx
│   │   └── ResultDisplay.tsx
│   ├── lib/
│   │   ├── api.ts          # API 客户端
│   │   └── types.ts        # TypeScript 类型
│   ├── public/
│   ├── package.json
│   ├── tsconfig.json
│   └── tailwind.config.js
└── README.md
```

## 页面设计

### 1. 登录页 (/login)
- 密码输入
- JWT token 存储

### 2. Dashboard (/dashboard)
- 命令列表（按站点分组）
- 搜索过滤
- 执行对话框

### 3. 执行对话框
- 参数表单
- 格式选择
- 结果展示

## 技术栈

- Next.js 15
- React 19
- TypeScript
- Tailwind CSS
- shadcn/ui（可选组件库）

## 开发模式

```bash
# 后端
cargo run -- serve --port 3133

# 前端
cd web
npm run dev  # http://localhost:3000
```

## 生产部署

### 方案1：分离部署
- 后端：agent-browser-hub 二进制
- 前端：Vercel/Netlify

### 方案2：集成部署
- 前端构建后嵌入 Rust
- 单一二进制文件

## 实施步骤

1. 创建 Next.js 项目
2. 设计组件
3. 实现 API 客户端
4. 开发页面
5. 集成测试
