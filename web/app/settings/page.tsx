'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { authApi } from '@/lib/api/commands';

export default function SettingsPage() {
  const router = useRouter();
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [msg, setMsg] = useState('');
  const [msgType, setMsgType] = useState<'ok' | 'err'>('ok');

  useEffect(() => {
    if (!localStorage.getItem('hub_token')) {
      router.push('/login');
    }
  }, [router]);

  const handleChangePassword = async () => {
    setMsg('');
    if (newPassword.length < 4) {
      setMsg('密码至少 4 个字符');
      setMsgType('err');
      return;
    }
    if (newPassword !== confirmPassword) {
      setMsg('两次密码不一致');
      setMsgType('err');
      return;
    }
    try {
      const res = await authApi.changePassword(newPassword);
      if (res.success) {
        setMsg('密码已更新，请重新登录');
        setMsgType('ok');
        setTimeout(() => {
          localStorage.removeItem('hub_token');
          router.push('/login');
        }, 1500);
      } else {
        setMsg(res.message);
        setMsgType('err');
      }
    } catch {
      setMsg('网络错误');
      setMsgType('err');
    }
  };

  return (
    <div className="min-h-screen bg-gray-100 flex items-center justify-center">
      <div className="bg-white rounded-xl shadow p-10 w-[420px]">
        <h1 className="text-xl font-bold mb-6">设置</h1>

        <div className="mb-6">
          <h2 className="text-sm font-semibold text-gray-500 mb-3">修改密码</h2>
          <label className="block text-xs text-gray-500 mb-1">新密码</label>
          <input
            type="password"
            value={newPassword}
            onChange={(e) => setNewPassword(e.target.value)}
            placeholder="至少 4 个字符"
            className="w-full px-3 py-2 border rounded-lg bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 mb-3 text-sm"
          />
          <label className="block text-xs text-gray-500 mb-1">确认密码</label>
          <input
            type="password"
            value={confirmPassword}
            onChange={(e) => setConfirmPassword(e.target.value)}
            placeholder="再次输入新密码"
            className="w-full px-3 py-2 border rounded-lg bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 mb-3 text-sm"
          />
          <button
            onClick={handleChangePassword}
            className="px-5 py-2 bg-blue-600 text-white rounded text-sm font-semibold hover:bg-blue-700"
          >
            更新密码
          </button>
          {msg && (
            <p className={`text-sm mt-2 ${msgType === 'ok' ? 'text-green-600' : 'text-red-500'}`}>{msg}</p>
          )}
        </div>

        <div className="flex justify-end">
          <button onClick={() => router.back()} className="px-5 py-2 bg-gray-200 text-gray-600 rounded text-sm hover:bg-gray-300">
            返回
          </button>
        </div>
      </div>
    </div>
  );
}
