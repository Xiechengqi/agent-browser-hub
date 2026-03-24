import axios from 'axios';
import { Command, ExecuteRequest, ExecuteResult, WorkflowSourceStatus } from '@/types/command';

const api = axios.create({
  baseURL: '',
});

api.interceptors.request.use((config) => {
  const token = typeof window !== 'undefined' ? localStorage.getItem('hub_token') : null;
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

export const commandsApi = {
  list: () => api.get<Command[]>('/api/commands').then(r => r.data),
  execute: (site: string, name: string, req: ExecuteRequest) =>
    api.post<ExecuteResult>(`/api/execute/${site}/${name}`, req).then(r => r.data),
};

export const authApi = {
  login: (password: string) =>
    api.post('/api/login', { password }).then(r => r.data),
  changePassword: (password: string) =>
    api.post('/api/password', { password }).then(r => r.data),
};

export const systemApi = {
  version: () => api.get('/api/version').then(r => r.data),
  upgrade: () => api.post('/api/upgrade').then(r => r.data),
  upgradeComponent: (name: string) => api.post(`/api/upgrade/${name}`).then(r => r.data),
  logs: (limit = 200) => api.get(`/api/logs?limit=${limit}`).then(r => r.data),
  workflowSources: () => api.get<WorkflowSourceStatus[]>('/api/workflow/sources').then(r => r.data),
};
