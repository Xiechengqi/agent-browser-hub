'use client';

import { useState } from 'react';
import { Play, Lock } from 'lucide-react';
import { Command } from '@/types/command';
import { Card, Badge } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import ExecuteDialog from '@/components/execute/ExecuteDialog';

interface Props {
  command: Command;
}

export default function CommandCard({ command }: Props) {
  const [showDialog, setShowDialog] = useState(false);

  const strategyColor: Record<string, string> = {
    PUBLIC: 'bg-green-100 text-green-800',
    COOKIE: 'bg-blue-100 text-blue-800',
    HEADER: 'bg-purple-100 text-purple-800',
    INTERCEPT: 'bg-orange-100 text-orange-800',
    UI: 'bg-red-100 text-red-800',
  };

  return (
    <>
      <Card className="p-4 hover:shadow-lg transition-shadow cursor-pointer" onClick={() => setShowDialog(true)}>
        <div className="flex items-start justify-between mb-2">
          <h3 className="font-semibold text-lg">{command.name}</h3>
          <Badge className={strategyColor[command.strategy] || 'bg-gray-100 text-gray-800'}>{command.strategy}</Badge>
        </div>
        <p className="text-sm text-gray-600 mb-4 line-clamp-2">{command.description || '无描述'}</p>
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2 text-xs text-gray-500">
            {command.strategy !== 'PUBLIC' && <Lock size={14} />}
            <span>{command.params.length} 个参数</span>
          </div>
          <Button size="sm" onClick={(e) => { e.stopPropagation(); setShowDialog(true); }} className="gap-2">
            <Play size={16} />执行
          </Button>
        </div>
      </Card>
      <ExecuteDialog command={command} open={showDialog} onClose={() => setShowDialog(false)} />
    </>
  );
}
