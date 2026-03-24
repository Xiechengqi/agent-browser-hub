'use client';

import { Search } from 'lucide-react';
import { useCommandsStore } from '@/lib/store/commands';

export default function CommandSearch() {
  const { searchQuery, setSearchQuery } = useCommandsStore();

  return (
    <div className="mb-8 relative">
      <Search className="absolute left-4 top-1/2 -translate-y-1/2 text-slate-400" size={20} />
      <input
        type="text"
        placeholder="搜索命令..."
        value={searchQuery}
        onChange={(e) => setSearchQuery(e.target.value)}
        className="w-full pl-12 pr-4 py-3 bg-white border border-slate-200 rounded-lg text-slate-900 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 shadow-soft transition-all duration-200"
      />
    </div>
  );
}
