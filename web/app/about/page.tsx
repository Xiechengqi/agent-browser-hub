'use client';

import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { systemApi } from '@/lib/api/commands';
import { useAuth } from '@/lib/store/auth';

interface VersionData {
  current: string;
  latest: string | null;
  commit: string;
  commit_date: string;
  commit_message: string;
  build_time: string;
}

export default function AboutPage() {
  const { isAuthenticated } = useAuth();
  const router = useRouter();
  const [version, setVersion] = useState<VersionData | null>(null);

  useEffect(() => {
    if (!isAuthenticated && !localStorage.getItem('hub_token')) {
      router.push('/login');
      return;
    }
    systemApi.version().then((res) => {
      if (res.success) setVersion(res.data);
    });
  }, [isAuthenticated, router]);

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
    <div className="min-h-screen bg-gray-100 flex items-center justify-center">
      <div className="bg-white rounded-xl shadow p-10 w-[520px]">
        <h1 className="text-xl font-bold mb-6 flex items-center gap-2">
          <span className="w-7 h-7 bg-blue-600 rounded text-white flex items-center justify-center text-sm">i</span>
          版本信息
        </h1>

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

        <div className="mt-6 flex justify-center">
          <button onClick={() => router.back()} className="px-5 py-2 bg-gray-200 text-gray-600 rounded text-sm hover:bg-gray-300">
            返回
          </button>
        </div>
      </div>
    </div>
  );
}
