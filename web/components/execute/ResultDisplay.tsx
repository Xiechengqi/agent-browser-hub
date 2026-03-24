'use client';

import { useState } from 'react';
import { ExecuteResult } from '@/types/command';
import { Button } from '@/components/ui/button';

interface Props {
  result: ExecuteResult;
  format: string;
}

export default function ResultDisplay({ result, format }: Props) {
  const [copied, setCopied] = useState(false);

  if (!result.success) {
    return (
      <div className="p-4 bg-red-50 border border-red-200 rounded">
        <p className="text-red-800 font-semibold">执行失败</p>
        <p className="text-red-600 text-sm mt-1">{result.error}</p>
      </div>
    );
  }

  const content = typeof result.data === 'string' ? result.data : JSON.stringify(result.data, null, 2);

  const handleCopy = () => {
    navigator.clipboard.writeText(content);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div>
      <div className="flex justify-between items-center mb-2">
        <h3 className="font-semibold">执行结果</h3>
        <Button size="sm" onClick={handleCopy}>
          {copied ? '已复制' : '复制'}
        </Button>
      </div>
      <pre className="bg-slate-50 p-4 rounded border border-slate-200 overflow-auto max-h-96 text-sm">
        {content}
      </pre>
    </div>
  );
}
