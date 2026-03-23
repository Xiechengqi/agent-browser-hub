# Next.js 前端实施指南

## 快速开始

### 1. 创建项目

```bash
cd /data/projects/agent-browser-hub
npx create-next-app@latest web --typescript --tailwind --app --eslint
```

选项：
- TypeScript: Yes
- Tailwind CSS: Yes
- App Router: Yes
- Import alias: @/*

### 2. 安装依赖

```bash
cd web
npm install axios zustand @tanstack/react-query lucide-react
npm install -D @types/node
```

### 3. 创建目录结构

```bash
mkdir -p {lib/{api,store,hooks,utils},components/{ui,layout,command,execute,auth},types}
```

### 4. 复制代码

按照以下文档复制代码：
- FRONTEND_ARCHITECTURE.md - 核心架构
- FRONTEND_COMPONENTS.md - 组件实现

### 5. 配置环境变量

创建 `.env.local`:
```
NEXT_PUBLIC_API_URL=http://localhost:3133
```

### 6. 运行开发服务器

```bash
npm run dev
```

访问 http://localhost:3000

## 文件清单

所有代码已在规划文档中提供：
- ✅ FRONTEND_ARCHITECTURE.md
- ✅ FRONTEND_COMPONENTS.md
- ✅ NEXTJS_FRONTEND_PLAN.md

## 后续步骤

1. 按文档创建所有文件
2. 测试功能
3. 构建生产版本
4. 部署

## 生产构建

```bash
npm run build
npm run start
```
