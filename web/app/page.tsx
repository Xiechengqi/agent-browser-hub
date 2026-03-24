'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import { useCommands } from '@/lib/hooks/useCommands';
import { useCommandsStore } from '@/lib/store/commands';
import { useAuth } from '@/lib/store/auth';
import CommandSearch from '@/components/command/CommandSearch';
import CommandList from '@/components/command/CommandList';

export default function Page() {
  const { isAuthenticated, logout } = useAuth();
  const router = useRouter();
  const { data: commands, isLoading } = useCommands();
  const setCommands = useCommandsStore((state) => state.setCommands);

  useEffect(() => {
    if (!isAuthenticated) {
      const stored = localStorage.getItem('hub_token');
      if (!stored) router.push('/login');
    }
  }, [isAuthenticated, router]);

  useEffect(() => {
    if (commands) setCommands(commands);
  }, [commands, setCommands]);

  if (!isAuthenticated) return null;

  return (
    <div className="min-h-screen bg-gray-50">
      <header className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 py-4 flex justify-between items-center">
          <h1 className="text-2xl font-bold">Agent Browser Hub</h1>
          <div className="flex items-center gap-4">
            <Link href="/about" className="text-sm text-gray-500 hover:text-gray-800">版本信息</Link>
            <Link href="/settings" className="text-sm text-gray-500 hover:text-gray-800">设置</Link>
            <button
              onClick={logout}
              className="text-sm text-red-400 hover:text-red-600"
            >
              退出登录
            </button>
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
    </div>
  );
}
