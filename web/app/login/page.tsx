'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useAuth } from '@/lib/store/auth';

export default function LoginPage() {
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);
  const { login } = useAuth();
  const router = useRouter();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setLoading(true);
    const ok = await login(password);
    if (ok) {
      router.push('/');
    } else {
      setError('密码错误');
    }
    setLoading(false);
  };

  return (
    <div className="min-h-screen bg-slate-50 flex items-center justify-center relative overflow-hidden">
      {/* Background blobs */}
      <div className="absolute top-0 right-0 w-96 h-96 bg-gradient-to-br from-indigo-200 to-violet-200 rounded-full blur-3xl opacity-30" />
      <div className="absolute bottom-0 left-0 w-96 h-96 bg-gradient-to-tr from-violet-200 to-indigo-200 rounded-full blur-3xl opacity-20" />

      <div className="relative bg-white rounded-xl shadow-hover border border-slate-200 p-10 w-full max-w-md">
        <h1 className="text-3xl font-extrabold text-center mb-2 bg-gradient-to-r from-indigo-600 to-violet-600 bg-clip-text text-transparent">
          Agent Browser Hub
        </h1>
        <p className="text-slate-600 text-center text-sm mb-8">浏览器自动化脚本管理平台</p>

        <form onSubmit={handleSubmit}>
          <label className="block text-sm font-semibold text-slate-700 mb-2">密码</label>
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            placeholder="默认密码 admin123"
            autoFocus
            className="w-full px-4 py-3 border border-slate-200 rounded-lg bg-white focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 mb-6 transition-all duration-200"
          />
          <button
            type="submit"
            disabled={loading}
            className="w-full py-3 bg-gradient-to-r from-indigo-600 to-violet-600 text-white rounded-full font-semibold hover:-translate-y-0.5 shadow-md hover:shadow-hover disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200"
          >
            {loading ? '登录中...' : '登录'}
          </button>
          {error && <p className="text-rose-600 text-sm text-center mt-4">{error}</p>}
        </form>
      </div>
    </div>
  );
}
