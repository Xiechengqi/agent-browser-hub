# Agent Browser Hub - Next.js 前端完整架构

## 1. 技术栈

### 核心框架
- **Next.js 15** (App Router)
- **React 19**
- **TypeScript 5**
- **Tailwind CSS 4**

### UI 组件
- **shadcn/ui** - 高质量组件库
- **Radix UI** - 无样式组件基础
- **Lucide Icons** - 图标库

### 状态管理
- **Zustand** - 轻量级状态管理
- **React Query** - 服务端状态管理

### 工具库
- **axios** - HTTP 客户端
- **zod** - 类型验证
- **date-fns** - 日期处理

## 2. 目录结构

```
web/
├── app/                          # Next.js App Router
│   ├── layout.tsx                # 根布局
│   ├── page.tsx                  # 首页（重定向到 dashboard）
│   ├── login/
│   │   └── page.tsx              # 登录页
│   ├── dashboard/
│   │   ├── layout.tsx            # Dashboard 布局
│   │   └── page.tsx              # 命令列表
│   └── globals.css               # 全局样式
│
├── components/                   # React 组件
│   ├── ui/                       # shadcn/ui 组件
│   │   ├── button.tsx
│   │   ├── card.tsx
│   │   ├── dialog.tsx
│   │   ├── input.tsx
│   │   ├── select.tsx
│   │   ├── table.tsx
│   │   └── tabs.tsx
│   │
│   ├── layout/                   # 布局组件
│   │   ├── Header.tsx
│   │   ├── Sidebar.tsx
│   │   └── Footer.tsx
│   │
│   ├── command/                  # 命令相关组件
│   │   ├── CommandList.tsx       # 命令列表
│   │   ├── CommandCard.tsx       # 命令卡片
│   │   ├── CommandGroup.tsx      # 命令分组
│   │   └── CommandSearch.tsx     # 搜索框
│   │
│   ├── execute/                  # 执行相关组件
│   │   ├── ExecuteDialog.tsx     # 执行对话框
│   │   ├── ParamForm.tsx         # 参数表单
│   │   ├── FormatSelector.tsx    # 格式选择器
│   │   └── ResultDisplay.tsx     # 结果展示
│   │
│   └── auth/                     # 认证组件
│       └── LoginForm.tsx         # 登录表单
│
├── lib/                          # 工具库
│   ├── api/                      # API 客户端
│   │   ├── client.ts             # Axios 实例
│   │   ├── auth.ts               # 认证 API
│   │   ├── commands.ts           # 命令 API
│   │   └── types.ts              # API 类型定义
│   │
│   ├── store/                    # 状态管理
│   │   ├── auth.ts               # 认证状态
│   │   ├── commands.ts           # 命令状态
│   │   └── ui.ts                 # UI 状态
│   │
│   ├── hooks/                    # 自定义 Hooks
│   │   ├── useAuth.ts
│   │   ├── useCommands.ts
│   │   └── useExecute.ts
│   │
│   └── utils/                    # 工具函数
│       ├── format.ts             # 格式化工具
│       ├── validation.ts         # 验证工具
│       └── storage.ts            # 本地存储
│
├── types/                        # TypeScript 类型
│   ├── command.ts
│   ├── api.ts
│   └── index.ts
│
├── public/                       # 静态资源
│   └── favicon.ico
│
├── .env.local                    # 环境变量
├── next.config.js                # Next.js 配置
├── tailwind.config.ts            # Tailwind 配置
├── tsconfig.json                 # TypeScript 配置
├── package.json
└── README.md
```

## 3. 核心类型定义

### types/command.ts
```typescript
export interface Command {
  site: string;
  name: string;
  description: string;
  strategy: 'PUBLIC' | 'COOKIE' | 'HEADER' | 'INTERCEPT' | 'UI';
  params: ParamDef[];
}

export interface ParamDef {
  name: string;
  type: 'string' | 'int' | 'number' | 'boolean';
  required: boolean;
  default?: any;
  description?: string;
}

export interface ExecuteRequest {
  params: Record<string, any>;
  format?: 'json' | 'yaml' | 'table' | 'csv' | 'md';
}

export interface ExecuteResult {
  execution_id: string;
  status: string;
  duration_ms: number;
  result: any;
}
```

### types/api.ts
```typescript
export interface ApiResponse<T> {
  success: boolean;
  message: string;
  data?: T;
}

export interface LoginRequest {
  password: string;
}

export interface LoginResponse {
  token: string;
}
```

## 4. API 客户端

### lib/api/client.ts
```typescript
import axios from 'axios';

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3133';

export const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// 请求拦截器：添加 token
apiClient.interceptors.request.use((config) => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// 响应拦截器：处理错误
apiClient.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('token');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);
```

### lib/api/auth.ts
```typescript
import { apiClient } from './client';
import { LoginRequest, LoginResponse } from '@/types/api';

export const authApi = {
  login: async (password: string) => {
    const { data } = await apiClient.post<LoginResponse>('/api/login', { password });
    return data;
  },

  changePassword: async (password: string) => {
    await apiClient.post('/api/password', { password });
  },
};
```

### lib/api/commands.ts
```typescript
import { apiClient } from './client';
import { Command, ExecuteRequest, ExecuteResult } from '@/types/command';

export const commandsApi = {
  list: async () => {
    const { data } = await apiClient.get<Command[]>('/api/scripts');
    return data;
  },

  execute: async (site: string, name: string, request: ExecuteRequest) => {
    const { data } = await apiClient.post<ExecuteResult>(
      `/api/execute/${site}/${name}`,
      request
    );
    return data;
  },
};
```

## 5. 状态管理

### lib/store/auth.ts
```typescript
import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface AuthState {
  token: string | null;
  isAuthenticated: boolean;
  login: (token: string) => void;
  logout: () => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      token: null,
      isAuthenticated: false,
      login: (token) => set({ token, isAuthenticated: true }),
      logout: () => set({ token: null, isAuthenticated: false }),
    }),
    { name: 'auth-storage' }
  )
);
```

### lib/store/commands.ts
```typescript
import { create } from 'zustand';
import { Command } from '@/types/command';

interface CommandsState {
  commands: Command[];
  filteredCommands: Command[];
  searchQuery: string;
  selectedSite: string | null;
  setCommands: (commands: Command[]) => void;
  setSearchQuery: (query: string) => void;
  setSelectedSite: (site: string | null) => void;
}

export const useCommandsStore = create<CommandsState>((set, get) => ({
  commands: [],
  filteredCommands: [],
  searchQuery: '',
  selectedSite: null,

  setCommands: (commands) => set({ commands, filteredCommands: commands }),

  setSearchQuery: (query) => {
    const { commands, selectedSite } = get();
    const filtered = commands.filter((cmd) => {
      const matchesSearch =
        cmd.name.toLowerCase().includes(query.toLowerCase()) ||
        cmd.description.toLowerCase().includes(query.toLowerCase());
      const matchesSite = !selectedSite || cmd.site === selectedSite;
      return matchesSearch && matchesSite;
    });
    set({ searchQuery: query, filteredCommands: filtered });
  },

  setSelectedSite: (site) => {
    const { commands, searchQuery } = get();
    const filtered = commands.filter((cmd) => {
      const matchesSearch =
        cmd.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        cmd.description.toLowerCase().includes(searchQuery.toLowerCase());
      const matchesSite = !site || cmd.site === site;
      return matchesSearch && matchesSite;
    });
    set({ selectedSite: site, filteredCommands: filtered });
  },
}));
```

## 6. 自定义 Hooks

### lib/hooks/useCommands.ts
```typescript
import { useQuery } from '@tanstack/react-query';
import { commandsApi } from '@/lib/api/commands';

export const useCommands = () => {
  return useQuery({
    queryKey: ['commands'],
    queryFn: commandsApi.list,
    staleTime: 5 * 60 * 1000, // 5 分钟
  });
};
```

### lib/hooks/useExecute.ts
```typescript
import { useMutation } from '@tanstack/react-query';
import { commandsApi } from '@/lib/api/commands';
import { ExecuteRequest } from '@/types/command';

export const useExecute = () => {
  return useMutation({
    mutationFn: ({ site, name, request }: {
      site: string;
      name: string;
      request: ExecuteRequest;
    }) => commandsApi.execute(site, name, request),
  });
};
```

## 7. 页面组件

### app/login/page.tsx
```typescript
'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useAuthStore } from '@/lib/store/auth';
import { authApi } from '@/lib/api/auth';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card } from '@/components/ui/card';

export default function LoginPage() {
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const router = useRouter();
  const login = useAuthStore((state) => state.login);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError('');

    try {
      const { token } = await authApi.login(password);
      login(token);
      router.push('/dashboard');
    } catch (err) {
      setError('密码错误');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <Card className="w-full max-w-md p-8">
        <h1 className="text-2xl font-bold mb-6 text-center">
          Agent Browser Hub
        </h1>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <Input
              type="password"
              placeholder="密码"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              disabled={loading}
            />
          </div>
          {error && (
            <p className="text-sm text-red-500">{error}</p>
          )}
          <Button type="submit" className="w-full" disabled={loading}>
            {loading ? '登录中...' : '登录'}
          </Button>
        </form>
      </Card>
    </div>
  );
}
```

### app/dashboard/page.tsx
```typescript
'use client';

import { useEffect } from 'react';
import { useCommands } from '@/lib/hooks/useCommands';
import { useCommandsStore } from '@/lib/store/commands';
import CommandList from '@/components/command/CommandList';
import CommandSearch from '@/components/command/CommandSearch';

export default function DashboardPage() {
  const { data: commands, isLoading } = useCommands();
  const setCommands = useCommandsStore((state) => state.setCommands);

  useEffect(() => {
    if (commands) {
      setCommands(commands);
    }
  }, [commands, setCommands]);

  if (isLoading) {
    return <div className="p-8">加载中...</div>;
  }

  return (
    <div className="p-8">
      <div className="mb-6">
        <h1 className="text-3xl font-bold mb-2">命令中心</h1>
        <p className="text-gray-600">选择并执行浏览器自动化命令</p>
      </div>

      <CommandSearch />
      <CommandList />
    </div>
  );
}
```

## 8. 核心组件（待续）

下一部分将详细设计：
- CommandList 组件
- CommandCard 组件
- ExecuteDialog 组件
- ResultDisplay 组件

需要我继续详细设计这些组件吗？
