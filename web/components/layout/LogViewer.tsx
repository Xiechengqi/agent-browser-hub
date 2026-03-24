'use client';

import { useEffect, useRef, useState } from 'react';
import { systemApi } from '@/lib/api/commands';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';

interface LogEntry {
  time: string;
  level: string;
  message: string;
}

interface Props {
  open: boolean;
  onClose: () => void;
}

const levelColor: Record<string, string> = {
  ERROR: 'text-red-500',
  WARN: 'text-yellow-500',
  INFO: 'text-blue-500',
  DEBUG: 'text-gray-400',
};

export default function LogViewer({ open, onClose }: Props) {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [autoRefresh, setAutoRefresh] = useState(true);
  const bottomRef = useRef<HTMLDivElement>(null);

  const fetchLogs = async () => {
    try {
      const data = await systemApi.logs(500);
      setLogs(data);
    } catch { /* ignore */ }
  };

  useEffect(() => {
    if (!open) return;
    fetchLogs();
    if (!autoRefresh) return;
    const timer = setInterval(fetchLogs, 3000);
    return () => clearInterval(timer);
  }, [open, autoRefresh]);

  useEffect(() => {
    if (open && bottomRef.current) {
      bottomRef.current.scrollIntoView({ behavior: 'smooth' });
    }
  }, [logs, open]);

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="max-w-4xl max-h-[85vh] flex flex-col">
        <DialogHeader>
          <div className="flex items-center justify-between">
            <DialogTitle>进程日志</DialogTitle>
            <div className="flex items-center gap-3">
              <label className="flex items-center gap-1 text-sm text-gray-500 cursor-pointer">
                <input
                  type="checkbox"
                  checked={autoRefresh}
                  onChange={(e) => setAutoRefresh(e.target.checked)}
                  className="rounded"
                />
                自动刷新
              </label>
              <button onClick={fetchLogs} className="text-sm text-blue-500 hover:text-blue-700">
                刷新
              </button>
            </div>
          </div>
        </DialogHeader>
        <div className="flex-1 overflow-auto bg-gray-900 rounded p-4 font-mono text-xs leading-5 min-h-[400px]">
          {logs.length === 0 ? (
            <p className="text-gray-500">暂无日志</p>
          ) : (
            logs.map((log, i) => (
              <div key={i} className="flex gap-2">
                <span className="text-gray-500 shrink-0">{log.time}</span>
                <span className={`shrink-0 w-12 ${levelColor[log.level] || 'text-gray-400'}`}>{log.level}</span>
                <span className="text-gray-200 break-all">{log.message}</span>
              </div>
            ))
          )}
          <div ref={bottomRef} />
        </div>
      </DialogContent>
    </Dialog>
  );
}
