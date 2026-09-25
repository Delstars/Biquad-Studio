interface SidebarProps {
  // Will be extended with active route state in Phase 2
}

const NAV_ITEMS = [
  { id: "mixer", label: "Mixer", icon: "🎚️", active: true },
  { id: "equalizer", label: "Equalizer", icon: "📊", active: false },
  { id: "calibration", label: "Calibration", icon: "🎧", active: false },
  { id: "spatial", label: "Spatial", icon: "🌐", active: false },
  { id: "settings", label: "Settings", icon: "⚙️", active: false },
] as const;

export function Sidebar(_props: SidebarProps) {
  return (
    <aside className="w-16 bg-bq-bg-secondary border-r border-bq-border flex flex-col items-center py-4 gap-2">
      {/* Logo */}
      <div className="w-10 h-10 rounded-lg bg-bq-accent/20 flex items-center justify-center mb-4">
        <span className="text-bq-accent font-bold text-lg">B</span>
      </div>

      {/* Navigation */}
      <nav className="flex flex-col gap-1 flex-1">
        {NAV_ITEMS.map((item) => (
          <button
            key={item.id}
            className={`w-12 h-12 rounded-lg flex flex-col items-center justify-center gap-0.5 transition-colors duration-150 ${
              item.active
                ? "bg-bq-accent/15 text-bq-accent"
                : "text-bq-text-muted hover:text-bq-text-secondary hover:bg-bq-bg-tertiary"
            }`}
            title={item.label}
          >
            <span className="text-base">{item.icon}</span>
            <span className="text-[9px] font-medium leading-none">{item.label}</span>
          </button>
        ))}
      </nav>

      {/* Version indicator */}
      <div className="text-[10px] text-bq-text-muted font-mono">v0.1</div>
    </aside>
  );
}
