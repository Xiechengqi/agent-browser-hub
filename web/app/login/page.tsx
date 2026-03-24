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
    <div className="min-h-screen bg-gray-100 flex items-center justify-center">
      <div className="bg-white rounded-xl shadow p-10 w-96">
        <h1 className="text-2xl font-bold text-center mb-2">Agent Browser Hub</h1>
        <p className="text-gray-500 text-center text-sm mb-8">浏览器自动化脚本管理平台</p>
        <form onSubmit={handleSubmit}>
          <label className="block text-sm text-gray-500 mb-1">密码</label>
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            placeholder="默认密码 admin123"
            autoFocus
            className="w-full px-3 py-2 border rounded-lg bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 mb-4"
          />
          <button
            type="submit"
            disabled={loading}
            className="w-full py-2 bg-blue-600 text-white rounded-lg font-semibold hover:bg-blue-700 disabled:bg-gray-300"
          >
            {loading ? '登录中...' : '登录'}
          </button>
          {error && <p className="text-red-500 text-sm text-center mt-3">{error}</p>}
        </form>
      </div>
    </div>
  );
}
