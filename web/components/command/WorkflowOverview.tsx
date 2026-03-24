'use client';

import { Command, WorkflowSourceStatus } from '@/types/command';

interface Props {
  commands: Command[];
  workflowSources: WorkflowSourceStatus[];
}

function sourceTone(source?: string): string {
  if (!source) return 'bg-slate-100 text-slate-600';
  if (source.includes('external')) return 'bg-amber-100 text-amber-800';
  if (source.includes('builtin')) return 'bg-emerald-100 text-emerald-800';
  if (source.includes('native')) return 'bg-indigo-100 text-indigo-800';
  return 'bg-slate-100 text-slate-600';
}

export default function WorkflowOverview({ commands, workflowSources }: Props) {
  const siteMap = commands.reduce((acc, command) => {
    if (!acc[command.site]) {
      acc[command.site] = [];
    }
    acc[command.site].push(command);
    return acc;
  }, {} as Record<string, Command[]>);

  const topSites = Object.entries(siteMap)
    .map(([site, siteCommands]) => {
      const workflowSource = workflowSources.find((item) => item.site === site);
      const source = siteCommands.find((item) => item.source)?.source || 'yaml';
      return {
        site,
        count: siteCommands.length,
        source,
        origin: siteCommands.find((item) => item.workflow_origin)?.workflow_origin,
        workflowSource,
      };
    })
    .sort((a, b) => b.count - a.count)
    .slice(0, 8);

  const externalCount = workflowSources.filter((item) => item.resolved).length;
  const fallbackCount = workflowSources.filter(
    (item) => item.effective_origin?.fallbackActive,
  ).length;

  return (
    <section className="mb-8">
      <div className="grid gap-4 md:grid-cols-3">
        <div className="rounded-2xl border border-slate-200 bg-white p-5 shadow-soft">
          <p className="text-xs font-semibold uppercase tracking-[0.2em] text-slate-400">Workflow Sites</p>
          <p className="mt-3 text-3xl font-black text-slate-900">{Object.keys(siteMap).length}</p>
          <p className="mt-2 text-sm text-slate-500">当前命令索引中的站点包总数</p>
        </div>
        <div className="rounded-2xl border border-slate-200 bg-white p-5 shadow-soft">
          <p className="text-xs font-semibold uppercase tracking-[0.2em] text-slate-400">External Sources</p>
          <p className="mt-3 text-3xl font-black text-slate-900">{externalCount}</p>
          <p className="mt-2 text-sm text-slate-500">已解析的独立站点覆盖源</p>
        </div>
        <div className="rounded-2xl border border-slate-200 bg-white p-5 shadow-soft">
          <p className="text-xs font-semibold uppercase tracking-[0.2em] text-slate-400">Fallback Active</p>
          <p className="mt-3 text-3xl font-black text-slate-900">{fallbackCount}</p>
          <p className="mt-2 text-sm text-slate-500">当前通过 builtin 回退保活的站点源</p>
        </div>
      </div>

      <div className="mt-5 rounded-3xl border border-slate-200 bg-white p-5 shadow-soft">
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-lg font-bold text-slate-900">Workflow Center</h2>
            <p className="mt-1 text-sm text-slate-500">优先展示当前最活跃的站点包、生效来源和覆盖状态</p>
          </div>
          <div className="text-xs text-slate-400">Top 8 Sites</div>
        </div>

        <div className="mt-4 grid gap-3 lg:grid-cols-2">
          {topSites.map((item) => (
            <div key={item.site} className="rounded-2xl border border-slate-200 bg-slate-50/70 p-4">
              <div className="flex items-start justify-between gap-4">
                <div>
                  <div className="flex items-center gap-2">
                    <h3 className="text-base font-bold capitalize text-slate-900">{item.site}</h3>
                    <span className={`rounded-full px-2.5 py-1 text-[11px] font-semibold ${sourceTone(item.source)}`}>
                      {item.source || 'yaml'}
                    </span>
                    {item.origin?.fallbackActive && (
                      <span className="rounded-full bg-rose-100 px-2.5 py-1 text-[11px] font-semibold text-rose-700">
                        fallback
                      </span>
                    )}
                  </div>
                  <p className="mt-2 text-sm text-slate-600">
                    {item.workflowSource?.package_display_name || item.site}
                    {' · '}
                    {item.count} commands
                  </p>
                </div>
                <div className="text-right text-xs text-slate-500">
                  <div>{item.workflowSource?.package_version || 'builtin'}</div>
                  <div className="mt-1 capitalize">{item.origin?.kind || 'legacy'}</div>
                </div>
              </div>
              <p className="mt-3 truncate text-xs text-slate-500">
                {item.origin?.location || item.workflowSource?.configured.path || item.workflowSource?.configured.url || 'builtin package'}
              </p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
