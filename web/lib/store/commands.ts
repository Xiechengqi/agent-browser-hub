import { create } from 'zustand';
import { Command } from '@/types/command';

interface CommandsState {
  commands: Command[];
  searchQuery: string;
  filteredCommands: Command[];
  setCommands: (commands: Command[]) => void;
  setSearchQuery: (query: string) => void;
}

export const useCommandsStore = create<CommandsState>((set, get) => ({
  commands: [],
  searchQuery: '',
  filteredCommands: [],
  setCommands: (commands) => {
    set({ commands, filteredCommands: commands });
  },
  setSearchQuery: (query) => {
    const { commands } = get();
    const needle = query.toLowerCase();
    const filtered = query
      ? commands.filter(c =>
          c.name.toLowerCase().includes(needle) ||
          c.site.toLowerCase().includes(needle) ||
          c.description?.toLowerCase().includes(needle) ||
          c.source?.toLowerCase().includes(needle) ||
          c.workflow_origin?.kind?.toLowerCase().includes(needle) ||
          c.workflow_origin?.location?.toLowerCase().includes(needle)
        )
      : commands;
    set({ searchQuery: query, filteredCommands: filtered });
  },
}));
