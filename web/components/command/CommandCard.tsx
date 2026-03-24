'use client';

import { useState } from 'react';
import { Play, Lock } from 'lucide-react';
import { Command } from '@/types/command';
import { useDebug } from '@/lib/context/debug';
import ExecuteDialog from '@/components/execute/ExecuteDialog';

interface Props {
  command: Command;
}

export default function CommandCard({ command }: Props) {
  const [showDialog, setShowDialog] = useState(false);
  const { debugMode, vncUrl, vncUsername, vncPassword } = useDebug();

  const strategyColor: Record<string, string> = {
    PUBLIC: 'bg-emerald-100 text-emerald-700',
    COOKIE: 'bg-indigo-100 text-indigo-700',
    HEADER: 'bg-violet-100 text-violet-700',
    INTERCEPT: 'bg-orange-100 text-orange-700',
    UI: 'bg-rose-100 text-rose-700',
  };
  const sourceColor =
    command.source?.includes('external')
      ? 'bg-amber-100 text-amber-800'
      : command.source?.includes('builtin')
        ? 'bg-emerald-100 text-emerald-800'
        : command.source?.includes('native')
          ? 'bg-indigo-100 text-indigo-800'
          : 'bg-slate-100 text-slate-700';

  return (
    <>
      <div
        onClick={() => setShowDialog(true)}
        className="group bg-white rounded-xl border border-slate-200 p-6 shadow-soft hover:shadow-hover hover:-translate-y-1 transition-all duration-200 cursor-pointer"
      >
        <div className="flex items-start justify-between mb-3 gap-3">
          <div>
            <h3 className="font-bold text-lg text-slate-900">{command.name}</h3>
            <div className="mt-2 flex flex-wrap gap-2">
              <span className={`px-3 py-1 text-xs font-semibold rounded-full ${strategyColor[command.strategy] || 'bg-slate-100 text-slate-700'}`}>
                {command.strategy}
              </span>
              {command.source && (
                <span className={`px-3 py-1 text-xs font-semibold rounded-full ${sourceColor}`}>
                  {command.source}
                </span>
              )}
              {command.workflow_origin?.fallbackActive && (
                <span className="px-3 py-1 text-xs font-semibold rounded-full bg-rose-100 text-rose-700">
                  fallback
                </span>
              )}
            </div>
          </div>
        </div>
        <p className="text-sm text-slate-600 mb-4 line-clamp-2">{command.description || '无描述'}</p>
        {command.workflow_origin && (
          <div className="mb-4 rounded-xl bg-slate-50 px-3 py-2 text-xs text-slate-500">
            <div className="font-semibold uppercase tracking-wide text-slate-400">{command.workflow_origin.kind}</div>
            <div className="mt-1 truncate">{command.workflow_origin.location}</div>
          </div>
        )}
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2 text-xs text-slate-500">
            {command.strategy !== 'PUBLIC' && <Lock size={14} />}
            <span>{command.params.length} 参数</span>
          </div>
          <button
            onClick={(e) => { e.stopPropagation(); setShowDialog(true); }}
            className="flex items-center gap-2 px-4 py-2 text-sm font-semibold text-white bg-gradient-to-r from-indigo-600 to-violet-600 rounded-full hover:-translate-y-0.5 shadow-md hover:shadow-hover transition-all duration-200"
          >
            <Play size={16} />执行
          </button>
        </div>
      </div>
      <ExecuteDialog command={command} open={showDialog} onClose={() => setShowDialog(false)} debugMode={debugMode} vncUrl={vncUrl} vncUsername={vncUsername} vncPassword={vncPassword} />
    </>
  );
}
