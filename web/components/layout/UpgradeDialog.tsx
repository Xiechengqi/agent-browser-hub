'use client';

import { useState } from 'react';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { systemApi } from '@/lib/api/commands';

interface Props {
  open: boolean;
  onClose: () => void;
}

export default function UpgradeDialog({ open, onClose }: Props) {
  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [upgrading, setUpgrading] = useState(false);
  const [results, setResults] = useState<{ name: string; ok: boolean; msg: string }[]>([]);

  const toggle = (name: string) => {
    setSelected((prev) => {
      const next = new Set(prev);
      if (next.has(name)) next.delete(name);
      else next.add(name);
      return next;
    });
  };

  const handleUpgrade = async () => {
    if (selected.size === 0) return;
    setUpgrading(true);
    setResults([]);
    const items = Array.from(selected);

    for (const name of items) {
      try {
        const res = await systemApi.upgradeComponent(name);
        setResults((prev) => [...prev, { name, ok: res.success, msg: res.success ? '升级成功' : (res.message || '未知错误') }]);
      } catch (err: any) {
        const msg = err?.response?.data?.message || err?.message || '网络错误';
        setResults((prev) => [...prev, { name, ok: false, msg }]);
      }
    }

    setUpgrading(false);

    if (items.includes('agent-browser-hub')) {
      setTimeout(() => window.location.reload(), 3000);
    }
  };

  const handleClose = () => {
    if (!upgrading) {
      setSelected(new Set());
      setResults([]);
      onClose();
    }
  };

  return (
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>强制升级</DialogTitle>
        </DialogHeader>
        <p className="text-sm text-gray-500 mb-4">选择要升级的组件：</p>
        <div className="space-y-3">
          {['agent-browser', 'agent-browser-hub'].map((name) => (
            <label
              key={name}
              className={`flex items-center gap-3 p-3 border rounded-lg cursor-pointer transition-colors ${
                selected.has(name) ? 'border-blue-500 bg-blue-50' : 'border-gray-200 hover:bg-gray-50'
              }`}
            >
              <input
                type="checkbox"
                checked={selected.has(name)}
                onChange={() => toggle(name)}
                disabled={upgrading}
                className="rounded"
              />
              <div>
                <span className="text-sm font-medium">{name}</span>
                <p className="text-xs text-gray-400">
                  {name === 'agent-browser' ? '替换浏览器二进制' : '替换并重启服务'}
                </p>
              </div>
            </label>
          ))}
        </div>

        {results.length > 0 && (
          <div className="mt-4 space-y-2">
            {results.map((r, i) => (
              <div key={i} className={`text-sm px-3 py-2 rounded ${r.ok ? 'bg-green-50 text-green-700' : 'bg-red-50 text-red-600'}`}>
                {r.name}: {r.msg}
              </div>
            ))}
          </div>
        )}

        <div className="mt-4 flex justify-end gap-2">
          <button
            onClick={handleClose}
            disabled={upgrading}
            className="px-4 py-2 text-sm text-gray-600 bg-gray-100 rounded hover:bg-gray-200 disabled:opacity-50"
          >
            取消
          </button>
          <button
            onClick={handleUpgrade}
            disabled={upgrading || selected.size === 0}
            className="px-4 py-2 text-sm text-white bg-orange-500 rounded hover:bg-orange-600 disabled:opacity-50"
          >
            {upgrading ? '升级中...' : '开始升级'}
          </button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
