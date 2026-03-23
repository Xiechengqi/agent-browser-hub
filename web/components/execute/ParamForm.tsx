'use client';

import { Param } from '@/types/command';
import { Input } from '@/components/ui/input';

interface Props {
  params: Param[];
  values: Record<string, any>;
  onChange: (values: Record<string, any>) => void;
}

export default function ParamForm({ params, values, onChange }: Props) {
  if (params.length === 0) {
    return <p className="text-sm text-gray-500">此命令无需参数</p>;
  }

  const handleChange = (name: string, value: any) => {
    onChange({ ...values, [name]: value });
  };

  return (
    <div className="space-y-4">
      {params.map((param) => (
        <div key={param.name}>
          <label className="block text-sm font-medium mb-1">
            {param.name}
            {param.required && <span className="text-red-500 ml-1">*</span>}
          </label>
          {param.description && (
            <p className="text-xs text-gray-500 mb-2">{param.description}</p>
          )}
          <Input
            type={param.type === 'number' ? 'number' : 'text'}
            value={values[param.name] ?? param.default ?? ''}
            onChange={(e) => handleChange(param.name, e.target.value)}
            placeholder={param.default?.toString()}
          />
        </div>
      ))}
    </div>
  );
}
