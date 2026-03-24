'use client';

import { useState, useEffect } from 'react';
import { authApi } from '@/lib/api/commands';
import { useAuth } from '@/lib/store/auth';
import { useDebug } from '@/lib/context/debug';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';

interface Props {
  open: boolean;
  onClose: () => void;
}

export default function SettingsDialog({ open, onClose }: Props) {
  const { logout } = useAuth();
  const { vncUrl: contextVncUrl, setVncUrl: setContextVncUrl } = useDebug();
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [vncUrl, setVncUrl] = useState('http://localhost:6080');
  const [msg, setMsg] = useState('');
  const [msgType, setMsgType] = useState<'ok' | 'err'>('ok');

  useEffect(() => {
    if (open) {
      setVncUrl(contextVncUrl);
      fetch('/api/settings')
        .then(res => res.json())
        .then(data => {
          if (data.success && data.data) {
            setVncUrl(data.data.vnc_url || 'http://localhost:6080');
          }
        })
        .catch(() => {});
    }
  }, [open, contextVncUrl]);

  const handleClose = () => {
    setNewPassword('');
    setConfirmPassword('');
    setMsg('');
    onClose();
  };

  const handleSaveSettings = async () => {
    setMsg('');
    try {
      const res = await fetch('/api/settings', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ vnc_url: vncUrl })
      });
      const data = await res.json();
      if (data.success) {
        setContextVncUrl(vncUrl);
        setMsg('设置已保存');
        setMsgType('ok');
      } else {
        setMsg(data.message || '保存失败');
        setMsgType('err');
      }
    } catch {
      setMsg('网络错误');
      setMsgType('err');
    }
  };

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
          handleClose();
          logout();
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
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogContent className="w-[420px]">
        <DialogHeader>
          <DialogTitle>设置</DialogTitle>
        </DialogHeader>
        <div className="space-y-6">
          <div>
            <h2 className="text-sm font-semibold text-gray-500 mb-3">VNC 设置</h2>
            <label className="block text-xs text-gray-500 mb-1">VNC Web URL</label>
            <input
              type="text"
              value={vncUrl}
              onChange={(e) => setVncUrl(e.target.value)}
              placeholder="http://localhost:6080"
              className="w-full px-3 py-2 border rounded-lg bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 mb-3 text-sm"
            />
            <button
              onClick={handleSaveSettings}
              className="px-5 py-2 bg-blue-600 text-white rounded text-sm font-semibold hover:bg-blue-700"
            >
              保存设置
            </button>
          </div>

          <div>
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
          </div>

          {msg && (
            <p className={`text-sm ${msgType === 'ok' ? 'text-green-600' : 'text-red-500'}`}>{msg}</p>
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}
