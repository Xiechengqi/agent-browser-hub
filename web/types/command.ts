export interface Command {
  site: string;
  name: string;
  description?: string;
  strategy: 'PUBLIC' | 'COOKIE' | 'HEADER' | 'INTERCEPT' | 'UI';
  params: Param[];
  source?: string;
  workflow_origin?: WorkflowOrigin;
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

export interface WorkflowOrigin {
  kind: string;
  location: string;
  fallbackActive?: boolean;
}

export interface WorkflowSourceConfig {
  type: string;
  path?: string | null;
  url?: string | null;
  ref?: string | null;
}

export interface WorkflowSourceStatus {
  site: string;
  configured: WorkflowSourceConfig;
  mode: string;
  fallback_to_builtin: boolean;
  builtin_available: boolean;
  resolved: boolean;
  effective_origin?: WorkflowOrigin | null;
  package_version?: string | null;
  package_display_name?: string | null;
  command_count: number;
  error?: string | null;
}
