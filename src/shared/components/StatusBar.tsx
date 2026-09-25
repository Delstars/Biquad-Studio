interface StatusBarProps {
  engineRunning: boolean;
  sampleRate: number;
}

export function StatusBar({ engineRunning, sampleRate }: StatusBarProps) {
  return (
    <footer className="flex items-center justify-between px-4 py-1.5 border-t border-bq-border bg-bq-bg-secondary/30 text-xs font-mono text-bq-text-muted">
      <div className="flex items-center gap-4">
        <span className="flex items-center gap-1.5">
          <span
            className={`w-1.5 h-1.5 rounded-full ${
              engineRunning ? "bg-bq-meter-green animate-pulse" : "bg-bq-text-muted"
            }`}
          />
          {engineRunning ? "Processing" : "Idle"}
        </span>
        <span>{sampleRate / 1000} kHz</span>
        <span>32-bit float</span>
      </div>
      <div className="flex items-center gap-4">
        <span>WASAPI Shared</span>
        <span>Biquad Studio v0.1.0</span>
      </div>
    </footer>
  );
}
