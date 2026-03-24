'use client';

import { createContext, useContext, useState, useEffect, ReactNode } from 'react';

interface DebugContextType {
  debugMode: boolean;
  vncUrl: string;
  vncUsername: string;
  vncPassword: string;
  setDebugMode: (mode: boolean) => void;
  setVncUrl: (url: string) => void;
  setVncAuth: (username: string, password: string) => void;
}

const DebugContext = createContext<DebugContextType | undefined>(undefined);

export function DebugProvider({ children }: { children: ReactNode }) {
  const [debugMode, setDebugModeState] = useState(false);
  const [vncUrl, setVncUrl] = useState('http://localhost:6080');
  const [vncUsername, setVncUsername] = useState('');
  const [vncPassword, setVncPassword] = useState('');

  useEffect(() => {
    const stored = localStorage.getItem('debug_mode');
    if (stored === 'true') setDebugModeState(true);
  }, []);

  useEffect(() => {
    const token = localStorage.getItem('hub_token');
    fetch('/api/settings', {
      headers: token ? { 'Authorization': `Bearer ${token}` } : {}
    })
      .then(res => res.json())
      .then(data => {
        if (data.success && data.data) {
          if (data.data.vnc_url) setVncUrl(data.data.vnc_url);
          if (data.data.vnc_username) setVncUsername(data.data.vnc_username);
          if (data.data.vnc_password) setVncPassword(data.data.vnc_password);
        }
      })
      .catch(() => {});
  }, []);

  const setDebugMode = (mode: boolean) => {
    setDebugModeState(mode);
    localStorage.setItem('debug_mode', String(mode));
  };

  const setVncAuth = (username: string, password: string) => {
    setVncUsername(username);
    setVncPassword(password);
  };

  return (
    <DebugContext.Provider value={{ debugMode, vncUrl, vncUsername, vncPassword, setDebugMode, setVncUrl, setVncAuth }}>
      {children}
    </DebugContext.Provider>
  );
}

export function useDebug() {
  const context = useContext(DebugContext);
  if (!context) throw new Error('useDebug must be used within DebugProvider');
  return context;
}
