'use client';

import { useCommandsStore } from '@/lib/store/commands';

export default function CommandOutline() {
  const filteredCommands = useCommandsStore((state) => state.filteredCommands);

  const groupedCommands = filteredCommands.reduce((acc, cmd) => {
    if (!acc[cmd.site]) acc[cmd.site] = [];
    acc[cmd.site].push(cmd);
    return acc;
  }, {} as Record<string, typeof filteredCommands>);

  const sites = Object.keys(groupedCommands).sort();

  const scrollToSite = (site: string) => {
    const el = document.getElementById(`site-${site}`);
    if (el) el.scrollIntoView({ behavior: 'smooth', block: 'start' });
  };

  if (sites.length === 0) return null;

  return (
    <div className="hidden xl:block fixed right-8 top-32 w-48 max-h-[calc(100vh-10rem)] overflow-y-auto">
      <div className="bg-white rounded-xl border border-slate-200 shadow-soft p-4">
        <h3 className="text-sm font-bold text-slate-900 mb-3">Workflow Sites</h3>
        <nav className="space-y-1">
          {sites.map((site) => (
            <button
              key={site}
              onClick={() => scrollToSite(site)}
              className="w-full text-left px-3 py-2 text-sm text-slate-600 hover:text-indigo-600 hover:bg-slate-50 rounded-lg transition-all duration-200 capitalize"
            >
              {site}
              <span className="ml-2 text-xs text-slate-400">({groupedCommands[site].length})</span>
            </button>
          ))}
        </nav>
      </div>
    </div>
  );
}
