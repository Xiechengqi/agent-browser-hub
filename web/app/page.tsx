'use client';

import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import { useCommands } from '@/lib/hooks/useCommands';
import { useCommandsStore } from '@/lib/store/commands';
import { useAuth } from '@/lib/store/auth';
import { systemApi } from '@/lib/api/commands';
import CommandSearch from '@/components/command/CommandSearch';
import CommandList from '@/components/command/CommandList';
import LogViewer from '@/components/layout/LogViewer';

export default function Page() {
  const { isAuthenticated, logout } = useAuth();
  const router = useRouter();
  const { data: commands, isLoading } = useCommands();
  const setCommands = useCommandsStore((state) => state.setCommands);
  const [upgrading, setUpgrading] = useState(false);
  const [showLogs, setShowLogs] = useState(false);

  useEffect(() => {
    if (!isAuthenticated) {
      const stored = localStorage.getItem('hub_token');
      if (!stored) router.push('/login');
    }
  }, [isAuthenticated, router]);

  useEffect(() => {
    if (commands) setCommands(commands);
  }, [commands, setCommands]);

  const handleUpgrade = async () => {
    if (!confirm('确认强制升级到最新版本？')) return;
    setUpgrading(true);
    try {
      const res = await systemApi.upgrade();
      if (res.success) {
        alert('升级完成，3秒后刷新页面');
        setTimeout(() => window.location.reload(), 3000);
      } else {
        alert('升级失败: ' + res.message);
        setUpgrading(false);
      }
    } catch {
      alert('网络错误');
      setUpgrading(false);
    }
  };

  if (!isAuthenticated) return null;

  return (
    <div className="min-h-screen bg-gray-50">
      <header className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 py-4 flex justify-between items-center">
          <h1 className="text-2xl font-bold">Agent Browser Hub</h1>
          <div className="flex items-center gap-4">
            <Link href="/about" className="text-sm text-gray-500 hover:text-gray-800">版本信息</Link>
            <button onClick={() => setShowLogs(true)} className="text-sm text-gray-500 hover:text-gray-800">日志</button>
            <button
              onClick={handleUpgrade}
              disabled={upgrading}
              className="text-sm text-orange-500 hover:text-orange-700 disabled:text-gray-300"
            >
              {upgrading ? '升级中...' : '强制升级'}
            </button>
            <Link href="/settings" className="text-sm text-gray-500 hover:text-gray-800">设置</Link>
            <button onClick={logout} className="text-sm text-red-400 hover:text-red-600">退出登录</button>
          </div>
        </div>
      </header>
      <main className="max-w-7xl mx-auto px-4 py-8">
        {isLoading ? (
          <div className="text-center py-12">加载中...</div>
        ) : (
          <>
            <CommandSearch />
            <CommandList />
          </>
        )}
      </main>
      <LogViewer open={showLogs} onClose={() => setShowLogs(false)} />
    </div>
  );
}
