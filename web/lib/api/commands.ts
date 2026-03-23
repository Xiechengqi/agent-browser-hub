import axios from 'axios';
import { Command, ExecuteRequest, ExecuteResult } from '@/types/command';

const api = axios.create({
  baseURL: process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3133',
});

export const commandsApi = {
  list: () => api.get<Command[]>('/api/commands').then(r => r.data),
  execute: (site: string, name: string, req: ExecuteRequest) =>
    api.post<ExecuteResult>(`/api/execute/${site}/${name}`, req).then(r => r.data),
};
