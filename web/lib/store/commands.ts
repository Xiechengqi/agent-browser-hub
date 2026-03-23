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
    const filtered = query
      ? commands.filter(c =>
          c.name.toLowerCase().includes(query.toLowerCase()) ||
          c.site.toLowerCase().includes(query.toLowerCase()) ||
          c.description?.toLowerCase().includes(query.toLowerCase())
        )
      : commands;
    set({ searchQuery: query, filteredCommands: filtered });
  },
}));
