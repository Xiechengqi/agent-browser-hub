'use client';

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { useEffect, useState } from 'react';
import { useCommands } from '@/lib/hooks/useCommands';
import { useCommandsStore } from '@/lib/store/commands';
import CommandSearch from '@/components/command/CommandSearch';
import CommandList from '@/components/command/CommandList';

const queryClient = new QueryClient();

function HomePage() {
  const { data: commands, isLoading } = useCommands();
  const setCommands = useCommandsStore((state) => state.setCommands);

  useEffect(() => {
    if (commands) setCommands(commands);
  }, [commands, setCommands]);

  return (
    <div className="min-h-screen bg-gray-50">
      <header className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 py-6">
          <h1 className="text-3xl font-bold">Agent Browser Hub</h1>
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

export default function Page() {
  return (
    <QueryClientProvider client={queryClient}>
      <HomePage />
    </QueryClientProvider>
  );
}
