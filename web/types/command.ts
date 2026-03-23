export interface Command {
  site: string;
  name: string;
  description?: string;
  strategy: 'PUBLIC' | 'COOKIE' | 'HEADER' | 'INTERCEPT' | 'UI';
  params: Param[];
}

export interface Param {
  name: string;
  type: string;
  required?: boolean;
  default?: any;
  description?: string;
}

export interface ExecuteRequest {
  params: Record<string, any>;
  format?: 'json' | 'yaml' | 'table' | 'csv' | 'md';
}

export interface ExecuteResult {
  success: boolean;
  data?: any;
  error?: string;
}
