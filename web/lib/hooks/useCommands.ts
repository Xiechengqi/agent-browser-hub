import { useQuery, useMutation } from '@tanstack/react-query';
import { commandsApi, systemApi } from '@/lib/api/commands';
import { ExecuteRequest } from '@/types/command';

export const useCommands = () => useQuery({
  queryKey: ['commands'],
  queryFn: commandsApi.list,
});

export const useExecute = () => useMutation({
  mutationFn: ({ site, name, request }: { site: string; name: string; request: ExecuteRequest }) =>
    commandsApi.execute(site, name, request),
});

export const useWorkflowSources = (enabled = true) => useQuery({
  queryKey: ['workflow-sources'],
  queryFn: systemApi.workflowSources,
  enabled,
});
