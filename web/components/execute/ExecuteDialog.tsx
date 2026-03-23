'use client';

import { useState } from 'react';
import { Command, ExecuteRequest } from '@/types/command';
import { useExecute } from '@/lib/hooks/useExecute';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import ParamForm from './ParamForm';
import FormatSelector from './FormatSelector';
import ResultDisplay from './ResultDisplay';
import { Button } from '@/components/ui/button';

interface Props {
  command: Command;
  open: boolean;
  onClose: () => void;
}

export default function ExecuteDialog({ command, open, onClose }: Props) {
  const [params, setParams] = useState<Record<string, any>>({});
  const [format, setFormat] = useState<'json' | 'yaml' | 'table' | 'csv' | 'md'>('json');
  const { mutate: execute, data: result, isPending, isSuccess } = useExecute();

  const handleExecute = () => {
    const request: ExecuteRequest = { params, format };
    execute({ site: command.site, name: command.name, request });
  };

  const handleClose = () => {
    setParams({});
    setFormat('json');
    onClose();
  };

  return (
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogContent className="max-w-4xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>执行命令: {command.site}/{command.name}</DialogTitle>
        </DialogHeader>
        <div className="space-y-6">
          <ParamForm params={command.params} values={params} onChange={setParams} />
          <FormatSelector value={format} onChange={setFormat} />
          <Button onClick={handleExecute} disabled={isPending} className="w-full">
            {isPending ? '执行中...' : '执行'}
          </Button>
          {isSuccess && result && <ResultDisplay result={result} format={format} />}
        </div>
      </DialogContent>
    </Dialog>
  );
}
