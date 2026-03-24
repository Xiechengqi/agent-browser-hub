'use client';

import { useState, useEffect } from 'react';
import { useQueryClient } from '@tanstack/react-query';
import { authApi } from '@/lib/api/commands';
import { useWorkflowSources } from '@/lib/hooks/useCommands';
import { useAuth } from '@/lib/store/auth';
import { useDebug } from '@/lib/context/debug';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';

interface Props {
  open: boolean;
  onClose: () => void;
}

type WorkflowMode = 'builtin-only' | 'prefer-external' | 'strict-external';

type WorkflowSource = {
  type: string;
  path?: string | null;
  url?: string | null;
  ref?: string | null;
};

const defaultWorkflow = {
  mode: 'prefer-external' as WorkflowMode,
  fallback_to_builtin: true,
  cache_dir: '',
  sources: {} as Record<string, WorkflowSource>,
};

function prettyWorkflowSources(sources: Record<string, WorkflowSource>): string {
  if (!sources || Object.keys(sources).length === 0) {
    return '{}';
  }
  return JSON.stringify(sources, null, 2);
}

export default function SettingsDialog({ open, onClose }: Props) {
  const { logout, isAuthenticated } = useAuth();
  const queryClient = useQueryClient();
  const { data: workflowSources = [] } = useWorkflowSources(isAuthenticated && open);
  const {
    vncUrl: contextVncUrl,
    vncUsername: contextVncUsername,
    vncPassword: contextVncPassword,
    setVncUrl: setContextVncUrl,
    setVncAuth,
  } = useDebug();
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [vncUrl, setVncUrl] = useState('http://localhost:6080');
  const [vncUsername, setVncUsername] = useState('');
  const [vncPassword, setVncPassword] = useState('');
  const [workflowMode, setWorkflowMode] = useState<WorkflowMode>('prefer-external');
  const [workflowFallback, setWorkflowFallback] = useState(true);
  const [workflowCacheDir, setWorkflowCacheDir] = useState('');
  const [workflowSourcesText, setWorkflowSourcesText] = useState('{}');
  const [msg, setMsg] = useState('');
  const [msgType, setMsgType] = useState<'ok' | 'err'>('ok');

  useEffect(() => {
    if (open) {
      setVncUrl(contextVncUrl);
      setVncUsername(contextVncUsername);
      setVncPassword(contextVncPassword);
      const token = localStorage.getItem('hub_token');
      fetch('/api/settings', {
        headers: token ? { Authorization: `Bearer ${token}` } : {},
      })
        .then((res) => res.json())
        .then((data) => {
          if (data.success && data.data) {
            setVncUrl(data.data.vnc_url || 'http://localhost:6080');
            setVncUsername(data.data.vnc_username || '');
            setVncPassword(data.data.vnc_password || '');
            const workflow = data.data.workflow || defaultWorkflow;
            setWorkflowMode((workflow.mode || 'prefer-external') as WorkflowMode);
            setWorkflowFallback(workflow.fallback_to_builtin ?? true);
            setWorkflowCacheDir(workflow.cache_dir || '');
            setWorkflowSourcesText(prettyWorkflowSources(workflow.sources || {}));
          }
        })
        .catch(() => {});
    }
  }, [open, contextVncUrl, contextVncUsername, contextVncPassword]);

  const handleClose = () => {
    setNewPassword('');
    setConfirmPassword('');
    setMsg('');
    onClose();
  };

  const handleSaveSettings = async () => {
    setMsg('');

    let workflowSources: Record<string, WorkflowSource> = {};
    try {
      workflowSources = JSON.parse(workflowSourcesText || '{}');
      if (workflowSources === null || Array.isArray(workflowSources) || typeof workflowSources !== 'object') {
        throw new Error('invalid workflow sources');
      }
    } catch {
      setMsg('Workflow sources 必须是合法 JSON 对象');
      setMsgType('err');
      return;
    }

    try {
      const token = localStorage.getItem('hub_token');
      const res = await fetch('/api/settings', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        body: JSON.stringify({
          vnc_url: vncUrl,
          vnc_username: vncUsername || null,
          vnc_password: vncPassword || null,
          workflow: {
            mode: workflowMode,
            fallback_to_builtin: workflowFallback,
            cache_dir: workflowCacheDir,
            sources: workflowSources,
          },
        }),
      });
      const data = await res.json();
      if (data.success) {
        setContextVncUrl(vncUrl);
        setVncAuth(vncUsername, vncPassword);
        setWorkflowSourcesText(prettyWorkflowSources(workflowSources));
        await Promise.all([
          queryClient.invalidateQueries({ queryKey: ['commands'] }),
          queryClient.invalidateQueries({ queryKey: ['workflow-sources'] }),
        ]);
        setMsg('设置已保存');
        setMsgType('ok');
      } else {
        setMsg(data.message || '保存失败');
        setMsgType('err');
      }
    } catch {
      setMsg('网络错误');
      setMsgType('err');
    }
  };

  const handleChangePassword = async () => {
    setMsg('');
    if (newPassword.length < 4) {
      setMsg('密码至少 4 个字符');
      setMsgType('err');
      return;
    }
    if (newPassword !== confirmPassword) {
      setMsg('两次密码不一致');
      setMsgType('err');
      return;
    }
    try {
      const res = await authApi.changePassword(newPassword);
      if (res.success) {
        setMsg('密码已更新，请重新登录');
        setMsgType('ok');
        setTimeout(() => {
          handleClose();
          logout();
        }, 1500);
      } else {
        setMsg(res.message);
        setMsgType('err');
      }
    } catch {
      setMsg('网络错误');
      setMsgType('err');
    }
  };

  return (
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogContent className="w-[720px] max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>设置</DialogTitle>
        </DialogHeader>
        <div className="space-y-8">
          <div>
            <h2 className="text-sm font-semibold text-gray-500 mb-3">VNC 设置</h2>
            <label className="block text-xs text-gray-500 mb-1">VNC Web URL</label>
            <input
              type="text"
              value={vncUrl}
              onChange={(e) => setVncUrl(e.target.value)}
              placeholder="http://localhost:6080"
              className="w-full px-3 py-2 border rounded-lg bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 mb-3 text-sm"
            />
            <label className="block text-xs text-gray-500 mb-1">VNC 用户名（可选）</label>
            <input
              type="text"
              value={vncUsername}
              onChange={(e) => setVncUsername(e.target.value)}
              placeholder="留空表示无需认证"
              className="w-full px-3 py-2 border rounded-lg bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 mb-3 text-sm"
            />
            <label className="block text-xs text-gray-500 mb-1">VNC 密码（可选）</label>
            <input
              type="password"
              value={vncPassword}
              onChange={(e) => setVncPassword(e.target.value)}
              placeholder="留空表示无需认证"
              className="w-full px-3 py-2 border rounded-lg bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm"
            />
          </div>

          <div>
            <h2 className="text-sm font-semibold text-gray-500 mb-3">Workflow Package</h2>
            <label className="block text-xs text-gray-500 mb-1">模式</label>
            <select
              value={workflowMode}
              onChange={(e) => setWorkflowMode(e.target.value as WorkflowMode)}
              className="w-full px-3 py-2 border rounded-lg bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 mb-3 text-sm"
            >
              <option value="builtin-only">builtin-only</option>
              <option value="prefer-external">prefer-external</option>
              <option value="strict-external">strict-external</option>
            </select>
            <label className="block text-xs text-gray-500 mb-1">缓存目录</label>
            <input
              type="text"
              value={workflowCacheDir}
              onChange={(e) => setWorkflowCacheDir(e.target.value)}
              placeholder="~/.cache/agent-browser-hub/workflows"
              className="w-full px-3 py-2 border rounded-lg bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 mb-3 text-sm"
            />
            <label className="flex items-center gap-2 text-sm text-gray-700 mb-3">
              <input
                type="checkbox"
                checked={workflowFallback}
                onChange={(e) => setWorkflowFallback(e.target.checked)}
                className="rounded border-gray-300"
              />
              外部 workflow 失效时回退到 builtin
            </label>
            <label className="block text-xs text-gray-500 mb-1">站点 override sources JSON</label>
            <textarea
              value={workflowSourcesText}
              onChange={(e) => setWorkflowSourcesText(e.target.value)}
              placeholder='{"twitter": {"type": "path", "path": "/data/workflows/twitter"}}'
              className="w-full min-h-[220px] px-3 py-2 border rounded-lg bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 font-mono text-xs"
            />
            <p className="text-xs text-gray-500 mt-2 leading-5">
              支持 `path` 和 `git`。示例：
              {' '}
              <span className="font-mono">{'{"twitter": {"type": "git", "url": "https://...", "ref": "main"}}'}</span>
            </p>

            {workflowSources.length > 0 && (
              <div className="mt-4 rounded-xl border border-slate-200 bg-slate-50 p-4">
                <div className="flex items-center justify-between">
                  <h3 className="text-sm font-semibold text-slate-700">当前 Source 状态</h3>
                  <span className="text-xs text-slate-400">{workflowSources.length} sites</span>
                </div>
                <div className="mt-3 space-y-2">
                  {workflowSources.map((item) => (
                    <div key={item.site} className="rounded-lg border border-slate-200 bg-white px-3 py-2">
                      <div className="flex items-center justify-between gap-3">
                        <div className="flex items-center gap-2">
                          <span className="font-medium capitalize text-slate-900">{item.site}</span>
                          <span className={`rounded-full px-2 py-0.5 text-[11px] font-semibold ${item.resolved ? 'bg-emerald-100 text-emerald-700' : 'bg-rose-100 text-rose-700'}`}>
                            {item.resolved ? 'resolved' : 'error'}
                          </span>
                          {item.effective_origin?.fallbackActive && (
                            <span className="rounded-full bg-amber-100 px-2 py-0.5 text-[11px] font-semibold text-amber-700">
                              fallback
                            </span>
                          )}
                        </div>
                        <div className="text-[11px] text-slate-500">
                          {item.effective_origin?.kind || item.configured.type}
                        </div>
                      </div>
                      <div className="mt-1 text-[11px] text-slate-500 truncate">
                        {item.effective_origin?.location || item.configured.path || item.configured.url || '-'}
                      </div>
                      {item.error && (
                        <div className="mt-1 text-[11px] text-rose-600 line-clamp-2">{item.error}</div>
                      )}
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>

          <div className="flex items-center gap-3">
            <button
              onClick={handleSaveSettings}
              className="px-5 py-2 bg-blue-600 text-white rounded text-sm font-semibold hover:bg-blue-700"
            >
              保存设置
            </button>
            {msg && (
              <p className={`text-sm ${msgType === 'ok' ? 'text-green-600' : 'text-red-500'}`}>{msg}</p>
            )}
          </div>

          <div>
            <h2 className="text-sm font-semibold text-gray-500 mb-3">修改密码</h2>
            <label className="block text-xs text-gray-500 mb-1">新密码</label>
            <input
              type="password"
              value={newPassword}
              onChange={(e) => setNewPassword(e.target.value)}
              placeholder="至少 4 个字符"
              className="w-full px-3 py-2 border rounded-lg bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 mb-3 text-sm"
            />
            <label className="block text-xs text-gray-500 mb-1">确认密码</label>
            <input
              type="password"
              value={confirmPassword}
              onChange={(e) => setConfirmPassword(e.target.value)}
              placeholder="再次输入新密码"
              className="w-full px-3 py-2 border rounded-lg bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 mb-3 text-sm"
            />
            <button
              onClick={handleChangePassword}
              className="px-5 py-2 bg-blue-600 text-white rounded text-sm font-semibold hover:bg-blue-700"
            >
              更新密码
            </button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
