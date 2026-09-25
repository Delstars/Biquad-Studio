interface CrossFaderProps {
  value: number;    // 0-100: 0 = full Game, 50 = center, 100 = full Chat
  onChange: (value: number) => void;
}

export function CrossFader({ value, onChange }: CrossFaderProps) {
  return (
    <div className="bq-panel px-6 py-4">
      <div className="flex items-center justify-between mb-2">
        <span className="text-xs font-semibold uppercase tracking-wider text-indigo-400">
          🎮 Game
        </span>
        <span className="text-xs font-semibold uppercase tracking-wider text-bq-text-secondary">
          Game / Chat Balance
        </span>
        <span className="text-xs font-semibold uppercase tracking-wider text-emerald-400">
          Chat 🎙️
        </span>
      </div>
      <input
        type="range"
        min={0}
        max={100}
        value={value}
        onChange={(e) => onChange(Number(e.target.value))}
        className="w-full h-2 appearance-none bg-gradient-to-r from-indigo-500/30 via-bq-bg-tertiary to-emerald-500/30 
                   rounded-full cursor-pointer
                   [&::-webkit-slider-thumb]:appearance-none
                   [&::-webkit-slider-thumb]:w-5
                   [&::-webkit-slider-thumb]:h-5
                   [&::-webkit-slider-thumb]:rounded-full
                   [&::-webkit-slider-thumb]:bg-white
                   [&::-webkit-slider-thumb]:shadow-lg
                   [&::-webkit-slider-thumb]:border-2
                   [&::-webkit-slider-thumb]:border-bq-accent
                   [&::-webkit-slider-thumb]:hover:scale-110
                   [&::-webkit-slider-thumb]:transition-transform"
      />
      <div className="flex justify-between mt-1">
        <span className="text-[10px] font-mono text-bq-text-muted">
          {Math.round(100 - value)}%
        </span>
        <span className="text-[10px] font-mono text-bq-text-muted">
          {value === 50 ? "Balanced" : ""}
        </span>
        <span className="text-[10px] font-mono text-bq-text-muted">
          {Math.round(value)}%
        </span>
      </div>
    </div>
  );
}
