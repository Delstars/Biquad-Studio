import type { AudioDeviceInfo } from "../../../App";

interface DeviceSelectorProps {
  devices: AudioDeviceInfo[];
  selectedId: string | null;
  onDeviceChange: (deviceId: string) => void;
  onRefresh: () => void;
}

export function DeviceSelector({
  devices,
  selectedId,
  onDeviceChange,
  onRefresh,
}: DeviceSelectorProps) {
  return (
    <div className="flex items-center gap-2">
      <label className="text-xs text-bq-text-muted uppercase tracking-wider">
        Output
      </label>
      <select
        value={selectedId ?? ""}
        onChange={(e) => onDeviceChange(e.target.value)}
        className="bg-bq-bg-tertiary border border-bq-border rounded-md px-3 py-1.5 text-sm text-bq-text
                   focus:outline-none focus:border-bq-accent/50 focus:ring-1 focus:ring-bq-accent/30
                   appearance-none cursor-pointer min-w-[200px]"
      >
        {devices.length === 0 && (
          <option value="" disabled>
            No devices found
          </option>
        )}
        {devices.map((device) => (
          <option key={device.id} value={device.id}>
            {device.name}
            {device.is_default ? " (Default)" : ""}
          </option>
        ))}
      </select>
      <button
        onClick={onRefresh}
        className="p-1.5 rounded-md text-bq-text-muted hover:text-bq-text-secondary
                   hover:bg-bq-bg-tertiary transition-colors"
        title="Refresh devices"
      >
        🔄
      </button>
    </div>
  );
}
