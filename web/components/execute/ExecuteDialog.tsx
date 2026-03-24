'use client';

import { useState } from 'react';
import { Command, ExecuteRequest } from '@/types/command';
import { useExecute } from '@/lib/hooks/useCommands';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import ParamForm from './ParamForm';
import FormatSelector from './FormatSelector';
import ResultDisplay from './ResultDisplay';
import { Button } from '@/components/ui/button';
import axios from 'axios';

interface Props {
  command: Command;
  open: boolean;
  onClose: () => void;
  debugMode?: boolean;
  vncUrl?: string;
}

function buildCurlCommand(site: string, name: string, params: Record<string, any>, format: string): string {
  const origin = typeof window !== 'undefined' ? window.location.origin : 'http://localhost:3133';
  const token = typeof window !== 'undefined' ? localStorage.getItem('hub_token') || '<TOKEN>' : '<TOKEN>';
  const body = JSON.stringify({ params, format });
  return `curl -X POST '${origin}/api/execute/${site}/${name}' \\\n  -H 'Content-Type: application/json' \\\n  -H 'Authorization: Bearer ${token}' \\\n  -d '${body}'`;
}

export default function ExecuteDialog({ command, open, onClose, debugMode = false, vncUrl = 'http://localhost:6080' }: Props) {
  const [params, setParams] = useState<Record<string, any>>({});
  const [format, setFormat] = useState<'json' | 'yaml' | 'table' | 'csv' | 'md'>('json');
  const [curlCommand, setCurlCommand] = useState('');
  const { mutate: execute, data: result, isPending, isSuccess, isError, error, reset } = useExecute();

  const handleExecute = () => {
    reset();
    setCurlCommand(buildCurlCommand(command.site, command.name, params, format));
    const request: ExecuteRequest = { params, format };
    execute({ site: command.site, name: command.name, request });
  };

  const handleClose = () => {
    setParams({});
    setFormat('json');
    setCurlCommand('');
    reset();
    onClose();
  };

  const getErrorMessage = () => {
    if (!isError || !error) return '';
    if (axios.isAxiosError(error)) {
      if (error.response?.status === 401) return '认证失败，请重新登录';
      const data = error.response?.data;
      if (data?.error) return data.error;
      if (data?.message) return data.message;
      return error.message || '请求失败';
    }
    return String(error);
  };

  return (
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogContent className="max-w-4xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>执行命令: {command.site}/{command.name}</DialogTitle>
        </DialogHeader>
        <div className="space-y-6">
          {debugMode && (
            <div className="border border-slate-200 rounded-lg overflow-hidden">
              <div className="bg-slate-100 px-4 py-2 text-sm font-semibold text-slate-700">VNC 调试窗口</div>
              <iframe src={vncUrl} className="w-full h-96 border-0" />
            </div>
          )}
          <ParamForm params={command.params} values={params} onChange={setParams} />
          <FormatSelector value={format} onChange={setFormat} />
          <Button onClick={handleExecute} disabled={isPending} className="w-full">
            {isPending ? '执行中...' : '执行'}
          </Button>
          {curlCommand && (
            <div className="p-4 bg-slate-50 border border-slate-200 rounded">
              <p className="text-slate-700 font-semibold mb-2">cURL 命令</p>
              <pre className="text-xs text-slate-600 overflow-x-auto">{curlCommand}</pre>
            </div>
          )}
          {isError && (
            <div className="p-4 bg-red-50 border border-red-200 rounded">
              <p className="text-red-800 font-semibold">请求失败</p>
              <p className="text-red-600 text-sm mt-1">{getErrorMessage()}</p>
            </div>
          )}
          {isSuccess && result && <ResultDisplay result={result} format={format} />}
        </div>
      </DialogContent>
    </Dialog>
  );
}
