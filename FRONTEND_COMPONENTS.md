# 核心组件设计（续）

## 9. 命令列表组件

### components/command/CommandList.tsx
```typescript
'use client';

import { useCommandsStore } from '@/lib/store/commands';
import CommandGroup from './CommandGroup';

export default function CommandList() {
  const filteredCommands = useCommandsStore((state) => state.filteredCommands);

  // 按站点分组
  const groupedCommands = filteredCommands.reduce((acc, cmd) => {
    if (!acc[cmd.site]) {
      acc[cmd.site] = [];
    }
    acc[cmd.site].push(cmd);
    return acc;
  }, {} as Record<string, typeof filteredCommands>);

  const sites = Object.keys(groupedCommands).sort();

  if (sites.length === 0) {
    return (
      <div className="text-center py-12 text-gray-500">
        未找到匹配的命令
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {sites.map((site) => (
        <CommandGroup
          key={site}
          site={site}
          commands={groupedCommands[site]}
        />
      ))}
    </div>
  );
}
```

### components/command/CommandGroup.tsx
```typescript
'use client';

import { useState } from 'react';
import { ChevronDown, ChevronRight } from 'lucide-react';
import { Command } from '@/types/command';
import CommandCard from './CommandCard';

interface Props {
  site: string;
  commands: Command[];
}

export default function CommandGroup({ site, commands }: Props) {
  const [isOpen, setIsOpen] = useState(true);

  return (
    <div className="border rounded-lg overflow-hidden">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="w-full px-4 py-3 bg-gray-50 hover:bg-gray-100 flex items-center justify-between transition-colors"
      >
        <div className="flex items-center gap-2">
          {isOpen ? <ChevronDown size={20} /> : <ChevronRight size={20} />}
          <span className="font-semibold text-lg capitalize">{site}</span>
          <span className="text-sm text-gray-500">({commands.length})</span>
        </div>
      </button>

      {isOpen && (
        <div className="p-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {commands.map((cmd) => (
            <CommandCard key={`${cmd.site}/${cmd.name}`} command={cmd} />
          ))}
        </div>
      )}
    </div>
  );
}
```

### components/command/CommandCard.tsx
```typescript
'use client';

import { useState } from 'react';
import { Play, Lock } from 'lucide-react';
import { Command } from '@/types/command';
import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import ExecuteDialog from '@/components/execute/ExecuteDialog';

interface Props {
  command: Command;
}

export default function CommandCard({ command }: Props) {
  const [showDialog, setShowDialog] = useState(false);

  const strategyColor = {
    PUBLIC: 'bg-green-100 text-green-800',
    COOKIE: 'bg-blue-100 text-blue-800',
    HEADER: 'bg-purple-100 text-purple-800',
    INTERCEPT: 'bg-orange-100 text-orange-800',
    UI: 'bg-red-100 text-red-800',
  };

  return (
    <>
      <Card className="p-4 hover:shadow-lg transition-shadow">
        <div className="flex items-start justify-between mb-2">
          <h3 className="font-semibold text-lg">{command.name}</h3>
          <Badge className={strategyColor[command.strategy]}>
            {command.strategy}
          </Badge>
        </div>

        <p className="text-sm text-gray-600 mb-4 line-clamp-2">
          {command.description || '无描述'}
        </p>

        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2 text-xs text-gray-500">
            {command.strategy !== 'PUBLIC' && (
              <Lock size={14} />
            )}
            <span>{command.params.length} 个参数</span>
          </div>

          <Button
            size="sm"
            onClick={() => setShowDialog(true)}
            className="gap-2"
          >
            <Play size={16} />
            执行
          </Button>
        </div>
      </Card>

      <ExecuteDialog
        command={command}
        open={showDialog}
        onClose={() => setShowDialog(false)}
      />
    </>
  );
}
```

### components/command/CommandSearch.tsx
```typescript
'use client';

import { Search } from 'lucide-react';
import { Input } from '@/components/ui/input';
import { useCommandsStore } from '@/lib/store/commands';

export default function CommandSearch() {
  const { searchQuery, setSearchQuery } = useCommandsStore();

  return (
    <div className="mb-6">
      <div className="relative">
        <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" size={20} />
        <Input
          type="text"
          placeholder="搜索命令..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="pl-10"
        />
      </div>
    </div>
  );
}
```

## 10. 执行对话框组件

### components/execute/ExecuteDialog.tsx
```typescript
'use client';

import { useState } from 'react';
import { Command, ExecuteRequest } from '@/types/command';
import { useExecute } from '@/lib/hooks/useExecute';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
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
          <DialogTitle>
            执行命令: {command.site}/{command.name}
          </DialogTitle>
        </DialogHeader>

        <div className="space-y-6">
          {/* 参数表单 */}
          <ParamForm
            params={command.params}
            values={params}
            onChange={setParams}
          />

          {/* 格式选择 */}
          <FormatSelector value={format} onChange={setFormat} />

          {/* 执行按钮 */}
          <Button
            onClick={handleExecute}
            disabled={isPending}
            className="w-full"
          >
            {isPending ? '执行中...' : '执行'}
          </Button>

          {/* 结果展示 */}
          {isSuccess && result && (
            <ResultDisplay result={result} format={format} />
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}
```

待续...需要我继续完成剩余组件设计吗？
