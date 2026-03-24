import axios from 'axios';
import { Command, ExecuteRequest, ExecuteResult } from '@/types/command';

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
