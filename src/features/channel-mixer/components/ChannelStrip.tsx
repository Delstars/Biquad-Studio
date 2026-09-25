interface ChannelStripProps {
  label: string;
  icon: string;
  colorClass: string;
  volume: number;      // 0-100
  muted: boolean;
  onVolumeChange: (value: number) => void;
  onMuteToggle: () => void;
  engineRunning: boolean;
}

export function ChannelStrip({
  label,
  icon,
  colorClass,
  volume,
  muted,
  onVolumeChange,
  onMuteToggle,
}: ChannelStripProps) {
  /** Convert a 0-100 volume to a dB display string */
  function volumeToDb(vol: number): string {
    if (vol === 0) return "-∞";
    // Simple mapping: 100 = 0dB, 0 = -inf
    const db = 20 * Math.log10(vol / 100);
    return `${db.toFixed(1)} dB`;
  }

  return (
    <div className="bq-panel flex flex-col items-center gap-3 px-4 py-4 flex-1 min-w-[100px] max-w-[160px]">
      {/* Channel Label */}
      <div className="flex flex-col items-center gap-1">
        <span className="text-xl">{icon}</span>
        <span className={`text-xs font-semibold uppercase tracking-wider ${colorClass}`}>
          {label}
        </span>
      </div>

      {/* Volume Slider (vertical) */}
      <div className="flex-1 flex flex-col items-center justify-center min-h-[200px] w-full">
        <input
          type="range"
          min={0}
          max={100}
          value={volume}
          onChange={(e) => onVolumeChange(Number(e.target.value))}
          className="h-full w-2 appearance-none bg-bq-bg-tertiary rounded-full cursor-pointer
                     [writing-mode:vertical-lr] [direction:rtl]
                     [&::-webkit-slider-thumb]:appearance-none
                     [&::-webkit-slider-thumb]:w-4
                     [&::-webkit-slider-thumb]:h-4
                     [&::-webkit-slider-thumb]:rounded-full
                     [&::-webkit-slider-thumb]:bg-bq-accent
                     [&::-webkit-slider-thumb]:shadow-md
                     [&::-webkit-slider-thumb]:hover:bg-bq-accent-hover
                     [&::-webkit-slider-thumb]:transition-colors"
          style={{ opacity: muted ? 0.4 : 1 }}
        />
      </div>

      {/* Volume Display */}
      <div className="text-center">
        <span className="text-xs font-mono text-bq-text-secondary block">
          {volumeToDb(muted ? 0 : volume)}
        </span>
        <span className="text-[10px] font-mono text-bq-text-muted">
          {muted ? "MUTED" : `${volume}%`}
        </span>
      </div>

      {/* Mute Button */}
      <button
        onClick={onMuteToggle}
        className={`w-full py-1.5 rounded text-xs font-medium transition-all duration-150 ${
          muted
            ? "bg-bq-meter-red/20 text-bq-meter-red border border-bq-meter-red/30"
            : "bg-bq-bg-tertiary text-bq-text-muted border border-bq-border hover:border-bq-border-active"
        }`}
      >
        {muted ? "🔇 Muted" : "🔊 Mute"}
      </button>
    </div>
  );
}
