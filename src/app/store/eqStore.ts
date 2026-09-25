import { create } from 'zustand';

export type FilterType = 'Peaking' | 'LowShelf' | 'HighShelf' | 'LowPass' | 'HighPass';

export interface ParametricFilter {
  id: string;
  enabled: boolean;
  type: FilterType;
  freq: number;
  gain: number;
  q: number;
}

interface EqStore {
  filters: ParametricFilter[];
  updateFilter: (id: string, updates: Partial<ParametricFilter>) => void;
  resetFilters: () => void;
}

const DEFAULT_FILTERS: ParametricFilter[] = Array.from({ length: 10 }).map((_, i) => ({
  id: `band-${i + 1}`,
  enabled: true,
  type: 'Peaking',
  // Distribute logarithmically between 30Hz and 16kHz
  freq: Math.round(30 * Math.pow(16000 / 30, i / 9)),
  gain: 0,
  q: 1.41,
}));

import { invoke } from '@tauri-apps/api/core';

export const useEqStore = create<EqStore>((set, get) => ({
  filters: DEFAULT_FILTERS,
  updateFilter: (id, updates) => {
    set((state) => ({
      filters: state.filters.map((f) => (f.id === id ? { ...f, ...updates } : f)),
    }));
    
    // Sync with backend
    const updatedFilter = get().filters.find(f => f.id === id);
    if (updatedFilter) {
      const band_index = parseInt(id.replace('band-', '')) - 1;
      const typeMap: Record<string, string> = {
        'Peaking': 'peaking',
        'LowShelf': 'low_shelf',
        'HighShelf': 'high_shelf',
        'LowPass': 'low_pass',
        'HighPass': 'high_pass',
      };
      
      invoke('plugin:audio|update_eq_filter', {
        params: {
          band_index,
          frequency_hz: updatedFilter.freq,
          gain_db: updatedFilter.gain,
          q: updatedFilter.q,
          filter_type: typeMap[updatedFilter.type] || 'peaking',
        }
      }).catch(console.error);
    }
  },
  resetFilters: () => set({ filters: DEFAULT_FILTERS }),
}));
