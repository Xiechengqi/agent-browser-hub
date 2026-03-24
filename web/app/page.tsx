'use client';

import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { useCommands } from '@/lib/hooks/useCommands';
import { useCommandsStore } from '@/lib/store/commands';
import { useAuth } from '@/lib/store/auth';
import CommandSearch from '@/components/command/CommandSearch';
import CommandList from '@/components/command/CommandList';
import CommandOutline from '@/components/command/CommandOutline';
import ScrollToTop from '@/components/layout/ScrollToTop';
import LogViewer from '@/components/layout/LogViewer';
import UpgradeDialog from '@/components/layout/UpgradeDialog';
import VersionDialog from '@/components/layout/VersionDialog';
import SettingsDialog from '@/components/layout/SettingsDialog';

export default function Page() {
  const { isAuthenticated, logout } = useAuth();
  const router = useRouter();
  const { data: commands, isLoading } = useCommands();
  const setCommands = useCommandsStore((state) => state.setCommands);
  const [showLogs, setShowLogs] = useState(false);
  const [showUpgrade, setShowUpgrade] = useState(false);
  const [showVersion, setShowVersion] = useState(false);
  const [showSettings, setShowSettings] = useState(false);

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
    <div className="min-h-screen bg-slate-50">
      {/* Atmospheric background blobs */}
      <div className="fixed inset-0 overflow-hidden pointer-events-none">
        <div className="absolute top-0 right-0 w-96 h-96 bg-gradient-to-br from-indigo-200 to-violet-200 rounded-full blur-3xl opacity-30" />
        <div className="absolute bottom-0 left-0 w-96 h-96 bg-gradient-to-tr from-violet-200 to-indigo-200 rounded-full blur-3xl opacity-20" />
      </div>

      <header className="relative bg-white border-b border-slate-200 shadow-soft">
        <div className="max-w-7xl mx-auto px-6 py-4 flex justify-between items-center">
          <h1 className="text-2xl font-extrabold bg-gradient-to-r from-indigo-600 to-violet-600 bg-clip-text text-transparent">
            Agent Browser Hub
          </h1>
          <nav className="flex items-center gap-2">
            <button onClick={() => setShowVersion(true)} className="px-3 py-2 text-sm font-medium text-slate-600 hover:text-indigo-600 hover:bg-slate-50 rounded-lg transition-all duration-200">版本</button>
            <button onClick={() => setShowLogs(true)} className="px-3 py-2 text-sm font-medium text-slate-600 hover:text-indigo-600 hover:bg-slate-50 rounded-lg transition-all duration-200">日志</button>
            <button onClick={() => setShowUpgrade(true)} className="px-3 py-2 text-sm font-medium text-orange-600 hover:text-orange-700 hover:bg-orange-50 rounded-lg transition-all duration-200">升级</button>
            <button onClick={() => setShowSettings(true)} className="px-3 py-2 text-sm font-medium text-slate-600 hover:text-indigo-600 hover:bg-slate-50 rounded-lg transition-all duration-200">设置</button>
            <button onClick={logout} className="px-4 py-2 text-sm font-semibold text-white bg-gradient-to-r from-indigo-600 to-violet-600 hover:-translate-y-0.5 rounded-full shadow-md hover:shadow-hover transition-all duration-200">退出</button>
          </nav>
        </div>
      </header>

      <main className="relative max-w-7xl mx-auto px-6 py-12">
        {isLoading ? (
          <div className="text-center py-20">
            <div className="inline-block w-8 h-8 border-4 border-indigo-200 border-t-indigo-600 rounded-full animate-spin" />
          </div>
        ) : (
          <>
            <CommandSearch />
            <CommandList />
            <CommandOutline />
          </>
        )}
      </main>

      <LogViewer open={showLogs} onClose={() => setShowLogs(false)} />
      <UpgradeDialog open={showUpgrade} onClose={() => setShowUpgrade(false)} />
      <VersionDialog open={showVersion} onClose={() => setShowVersion(false)} />
      <SettingsDialog open={showSettings} onClose={() => setShowSettings(false)} />
      <ScrollToTop />
    </div>
  );
}
