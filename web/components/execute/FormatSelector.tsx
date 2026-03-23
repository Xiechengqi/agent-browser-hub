'use client';

interface Props {
  value: string;
  onChange: (value: 'json' | 'yaml' | 'table' | 'csv' | 'md') => void;
}

export default function FormatSelector({ value, onChange }: Props) {
  const formats = ['json', 'yaml', 'table', 'csv', 'md'];

  return (
    <div>
      <label className="block text-sm font-medium mb-2">输出格式</label>
      <div className="flex gap-2">
        {formats.map((fmt) => (
          <button
            key={fmt}
            onClick={() => onChange(fmt as any)}
            className={`px-3 py-1 text-sm rounded ${
              value === fmt
                ? 'bg-blue-600 text-white'
                : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
            }`}
          >
            {fmt.toUpperCase()}
          </button>
        ))}
      </div>
    </div>
  );
}
