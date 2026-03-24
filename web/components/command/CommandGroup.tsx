'use client';

import { useState } from 'react';
import { ChevronDown, ChevronRight } from 'lucide-react';
import { Command } from '@/types/command';
import CommandCard from './CommandCard';

interface Props {
  site: string;
  commands: Command[];
}

export default function CommandGroup({ site, commands }: Props) {
  const [isOpen, setIsOpen] = useState(true);

  return (
    <div id={`site-${site}`} className="border rounded-lg overflow-hidden scroll-mt-20">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="w-full px-4 py-3 bg-gray-50 hover:bg-gray-100 flex items-center justify-between transition-colors"
      >
        <div className="flex items-center gap-2">
          {isOpen ? <ChevronDown size={20} /> : <ChevronRight size={20} />}
          <span className="font-semibold text-lg capitalize">{site}</span>
          <span className="text-sm text-gray-500">({commands.length})</span>
        </div>
      </button>
      {isOpen && (
        <div className="p-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {commands.map((cmd) => (
            <CommandCard key={`${cmd.site}/${cmd.name}`} command={cmd} />
          ))}
        </div>
      )}
    </div>
  );
}
