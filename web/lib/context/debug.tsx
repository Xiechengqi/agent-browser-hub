'use client';

import { createContext, useContext, useState, useEffect, ReactNode } from 'react';

interface DebugContextType {
  debugMode: boolean;
  vncUrl: string;
  setDebugMode: (mode: boolean) => void;
  setVncUrl: (url: string) => void;
}

const DebugContext = createContext<DebugContextType | undefined>(undefined);

export function DebugProvider({ children }: { children: ReactNode }) {
  const [debugMode, setDebugModeState] = useState(false);
  const [vncUrl, setVncUrl] = useState('http://localhost:6080');

  useEffect(() => {
    const stored = localStorage.getItem('debug_mode');
    if (stored === 'true') setDebugModeState(true);
  }, []);

  useEffect(() => {
    fetch('/api/settings')
      .then(res => res.json())
      .then(data => {
        if (data.success && data.data?.vnc_url) {
          setVncUrl(data.data.vnc_url);
        }
      })
      .catch(() => {});
  }, []);

  const setDebugMode = (mode: boolean) => {
    setDebugModeState(mode);
    localStorage.setItem('debug_mode', String(mode));
  };

  return (
    <DebugContext.Provider value={{ debugMode, vncUrl, setDebugMode, setVncUrl }}>
      {children}
    </DebugContext.Provider>
  );
}

export function useDebug() {
  const context = useContext(DebugContext);
  if (!context) throw new Error('useDebug must be used within DebugProvider');
  return context;
}
