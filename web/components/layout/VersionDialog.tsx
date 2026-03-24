'use client';

import { useEffect, useState } from 'react';
import { systemApi } from '@/lib/api/commands';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';

interface VersionData {
  current: string;
  latest: string | null;
  commit: string;
  commit_date: string;
  commit_message: string;
  build_time: string;
}

interface Props {
  open: boolean;
  onClose: () => void;
}

export default function VersionDialog({ open, onClose }: Props) {
  const [version, setVersion] = useState<VersionData | null>(null);

  useEffect(() => {
    if (open) {
      setVersion(null);
      systemApi.version().then((res) => {
        if (res.success) setVersion(res.data);
      });
    }
  }, [open]);

  const rows = version
    ? [
        ['版本', version.latest || version.current],
        ['提交', version.commit],
        ['提交日期', version.commit_date],
        ['提交信息', version.commit_message],
        ['构建时间', version.build_time],
      ]
    : [];

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="w-[520px]">
        <DialogHeader>
          <DialogTitle>
            <span className="flex items-center gap-2">
              <span className="w-7 h-7 bg-blue-600 rounded text-white flex items-center justify-center text-sm">i</span>
              版本信息
            </span>
          </DialogTitle>
        </DialogHeader>
        {!version ? (
          <p className="text-gray-500 text-center py-8">加载中...</p>
        ) : (
          <div className="space-y-3">
            {rows.map(([label, value]) => (
              <div key={label} className="flex justify-between items-center px-4 py-3 bg-gray-50 rounded-lg">
                <span className="text-sm text-gray-500">{label}</span>
                <span className="text-sm font-medium text-right max-w-[60%] break-all">{value}</span>
              </div>
            ))}
          </div>
        )}
      </DialogContent>
    </Dialog>
  );
}
