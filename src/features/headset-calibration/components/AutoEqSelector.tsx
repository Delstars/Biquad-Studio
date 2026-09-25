import React, { useState } from 'react';

export function AutoEqSelector() {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);

  const handleSearch = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const val = e.target.value;
    setQuery(val);
    if (val.length > 2) {
      setLoading(true);
      try {
        // The autoeq backend isn't fully wired via commands.rs yet, so we will stub the UI response
        // const res = await invoke('plugin:audio|search_headphones', { query: val });
        // setResults(res as any[]);
        setResults([
          { name: "Sennheiser HD 600", source: "oratory1990" },
          { name: "Sennheiser HD 800S", source: "crinacle" },
        ].filter(r => r.name.toLowerCase().includes(val.toLowerCase())));
      } catch (err) {
        console.error(err);
      }
      setLoading(false);
    } else {
      setResults([]);
    }
  };

  const handleApply = async (modelName: string) => {
    try {
      // await invoke('plugin:audio|apply_autoeq_profile', { name: modelName });
      console.log(`Applied AutoEQ profile for ${modelName}`);
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <div className="bg-bq-bg-secondary border border-bq-border rounded-xl p-4 flex flex-col gap-3">
      <div>
        <h3 className="text-white font-medium mb-1">Headset Auto-Calibration</h3>
        <p className="text-bq-text-secondary text-sm">
          Search the AutoEQ database to apply a neutral target curve for your headset.
        </p>
      </div>

      <div className="relative">
        <input
          type="text"
          value={query}
          onChange={handleSearch}
          placeholder="Search for your headset model..."
          className="w-full bg-bq-bg-tertiary border border-bq-border rounded-md px-3 py-2 text-white text-sm focus:outline-none focus:border-bq-accent"
        />
        {loading && <div className="absolute right-3 top-2.5 text-bq-text-secondary text-xs">Searching...</div>}
      </div>

      {results.length > 0 && (
        <ul className="max-h-40 overflow-y-auto border border-bq-border rounded-md bg-bq-bg-tertiary">
          {results.map((r, i) => (
            <li key={i} className="flex items-center justify-between px-3 py-2 border-b border-bq-border/50 hover:bg-bq-bg-secondary">
              <div>
                <div className="text-white text-sm font-medium">{r.name}</div>
                <div className="text-bq-text-secondary text-xs">Source: {r.source}</div>
              </div>
              <button
                onClick={() => handleApply(r.name)}
                className="px-3 py-1 bg-bq-accent/20 text-bq-accent rounded-md text-xs hover:bg-bq-accent hover:text-white transition-colors"
              >
                Apply
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
