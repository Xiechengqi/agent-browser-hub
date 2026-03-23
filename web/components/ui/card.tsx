import { HTMLAttributes } from 'react';

export const Card = ({ className = '', ...props }: HTMLAttributes<HTMLDivElement>) => (
  <div className={`bg-white border rounded-lg shadow ${className}`} {...props} />
);

export const Badge = ({ className = '', ...props }: HTMLAttributes<HTMLSpanElement>) => (
  <span className={`px-2 py-1 text-xs rounded ${className}`} {...props} />
);
