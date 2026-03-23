import '@/app/globals.css';
import { ReactNode } from 'react';

export const metadata = {
  title: 'Agent Browser Hub',
  description: 'Browser automation command hub',
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="zh">
      <body>{children}</body>
    </html>
  );
}
