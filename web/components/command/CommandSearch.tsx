'use client';

import { Search } from 'lucide-react';
import { Input } from '@/components/ui/input';
import { useCommandsStore } from '@/lib/store/commands';

export default function CommandSearch() {
  const { searchQuery, setSearchQuery } = useCommandsStore();

  return (
    <div className="mb-6 relative">
      <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" size={20} />
      <Input
        type="text"
        placeholder="搜索命令..."
        value={searchQuery}
        onChange={(e) => setSearchQuery(e.target.value)}
        className="pl-10"
      />
    </div>
  );
}
