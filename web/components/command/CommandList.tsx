'use client';

import { useCommandsStore } from '@/lib/store/commands';
import CommandGroup from './CommandGroup';

export default function CommandList() {
  const filteredCommands = useCommandsStore((state) => state.filteredCommands);

  const groupedCommands = filteredCommands.reduce((acc, cmd) => {
    if (!acc[cmd.site]) acc[cmd.site] = [];
    acc[cmd.site].push(cmd);
    return acc;
  }, {} as Record<string, typeof filteredCommands>);

  const sites = Object.keys(groupedCommands).sort();

  if (sites.length === 0) {
    return <div className="text-center py-12 text-gray-500">未找到匹配的命令</div>;
  }

  return (
    <div className="space-y-4">
      {sites.map((site) => (
        <CommandGroup key={site} site={site} commands={groupedCommands[site]} />
      ))}
    </div>
  );
}
