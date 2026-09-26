import { ReactNode } from "react";

interface ChannelStripProps {
  label: string;
  icon: string;
  colorClass: string;
  volume: number;
  muted: boolean;
  onVolumeChange: (value: number) => void;
  onMuteToggle: () => void;
  engineRunning: boolean;
  children?: ReactNode;
}

export function ChannelStrip({
  label,
  icon,
  colorClass,
  volume,
  muted,
  onVolumeChange,
  onMuteToggle,
  children
}: ChannelStripProps) {
  return (
    <div className="bq-panel flex flex-col items-center gap-3 px-4 py-4 flex-1 min-w-[120px] max-w-[200px]">
      <div className="flex items-center gap-2 mb-2 w-full justify-center">
        <span className={`text-sm font-bold tracking-wide uppercase ${colorClass}`}>
          {icon} {label}
        </span>
        <button className="text-bq-text-muted hover:text-white ml-auto">⚙</button>
      </div>

      <div className="w-full flex flex-col gap-1 text-[10px] uppercase font-bold text-bq-text-muted mt-2">
        <div className="flex items-center justify-between">
          <span>PRESETS</span>
        </div>
        <button className="bg-bq-bg-tertiary px-2 py-1.5 rounded w-full text-left flex items-center gap-2 text-white truncate hover:bg-bq-border-active transition-colors">
          <span className={`w-3 h-3 rounded-sm ${colorClass.replace('text-', 'bg-')}`}></span>
          {label === 'GAME' ? 'RPG-Cinema' : label === 'CHAT' ? 'Chat' : 'Flat'}
        </button>
      </div>

      <div className="w-full flex flex-col gap-1 text-[10px] uppercase font-bold text-bq-text-muted mt-2">
        <div className="flex items-center justify-between">
          <span>DEVICES</span>
        </div>
        <button className="bg-bq-bg-tertiary px-2 py-1.5 rounded w-full text-left flex justify-between items-center text-white truncate hover:bg-bq-border-active transition-colors">
          <span className="truncate flex items-center gap-1">
            <span className="text-bq-meter-green">🔗</span> Headphones
          </span>
          <span className="text-bq-text-muted">100%</span>
        </button>
      </div>

      <div className="flex-1 flex flex-col items-center justify-center min-h-[250px] w-full mt-6">
        <input
          type="range"
          min={0}
          max={100}
          value={volume}
          onChange={(e) => onVolumeChange(Number(e.target.value))}
          className="h-full w-2 appearance-none bg-bq-bg-tertiary rounded-full cursor-pointer
                     [writing-mode:vertical-lr] [direction:rtl]
                     [&::-webkit-slider-thumb]:appearance-none
                     [&::-webkit-slider-thumb]:w-5
                     [&::-webkit-slider-thumb]:h-3
                     [&::-webkit-slider-thumb]:rounded-sm
                     [&::-webkit-slider-thumb]:bg-white
                     [&::-webkit-slider-thumb]:shadow-md
                     [&::-webkit-slider-thumb]:hover:bg-gray-200
                     [&::-webkit-slider-thumb]:transition-colors"
          style={{ opacity: muted ? 0.4 : 1 }}
        />
      </div>

      <div className="text-center mt-2 w-full">
        <button
          onClick={onMuteToggle}
          className={`w-full py-2 rounded-md text-xs transition-all duration-150 flex justify-center items-center ${
            muted
              ? "bg-bq-meter-red/10 text-bq-meter-red hover:bg-bq-meter-red/20"
              : "text-bq-text-secondary hover:text-white hover:bg-bq-bg-tertiary"
          }`}
        >
          {muted ? "🔇" : "🔊"}
        </button>
      </div>

      {children && <div className="w-full">{children}</div>}
    </div>
  );
}
