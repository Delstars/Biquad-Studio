import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface ProcessInfo {
  pid: number;
  name: string;
  memory: number;
}

export function AppRouter() {
  const [apps, setApps] = useState<ProcessInfo[]>([]);
  const [assignments, setAssignments] = useState<Record<string, string>>({});

  useEffect(() => {
    loadApps();
    const interval = setInterval(loadApps, 5000); // Refresh every 5s
    return () => clearInterval(interval);
  }, []);

  const loadApps = async () => {
    try {
      const running = await invoke<ProcessInfo[]>("plugin:audio|list_running_apps");
      setApps(running);
    } catch (err) {
      console.error("Failed to load apps", err);
    }
  };

  const assignApp = async (appName: string, channel: string) => {
    try {
      // In a real implementation, this would call the Rust WASAPI capture backend
      // await invoke("plugin:audio|assign_app_to_channel", { appName, channel });
      setAssignments((prev) => ({ ...prev, [appName]: channel }));
      console.log(`Assigned ${appName} to ${channel}`);
    } catch (err) {
      console.error(err);
    }
  };

  const getChannelColor = (channel: string) => {
    switch (channel) {
      case "game": return "text-purple-400 bg-purple-400/10 border-purple-400/30";
      case "chat": return "text-blue-400 bg-blue-400/10 border-blue-400/30";
      case "media": return "text-green-400 bg-green-400/10 border-green-400/30";
      default: return "text-bq-text-secondary bg-bq-bg-tertiary border-bq-border";
    }
  };

  return (
    <div className="bg-bq-bg-secondary border border-bq-border rounded-xl p-4 flex flex-col gap-3 max-h-64 overflow-hidden">
      <div>
        <h3 className="text-white font-medium mb-1">Process Routing</h3>
        <p className="text-bq-text-secondary text-sm">
          Assign running applications to virtual audio channels.
        </p>
      </div>

      <div className="flex-1 overflow-y-auto pr-2 custom-scrollbar flex flex-col gap-2">
        {apps.map((app) => (
          <div key={app.name} className="flex items-center justify-between p-2 rounded-md bg-bq-bg-tertiary border border-bq-border/50">
            <div className="flex flex-col">
              <span className="text-sm text-white font-medium truncate w-32">{app.name}</span>
              <span className="text-xs text-bq-text-secondary">{(app.memory / 1024 / 1024).toFixed(0)} MB</span>
            </div>
            
            <div className="flex gap-1">
              {["game", "chat", "media"].map((channel) => {
                const isAssigned = assignments[app.name] === channel;
                return (
                  <button
                    key={channel}
                    onClick={() => assignApp(app.name, isAssigned ? "none" : channel)}
                    className={`px-2 py-1 text-xs rounded border transition-colors ${
                      isAssigned ? getChannelColor(channel) : "text-bq-text-secondary border-transparent hover:bg-bq-bg-secondary"
                    }`}
                  >
                    {channel.toUpperCase()}
                  </button>
                );
              })}
            </div>
          </div>
        ))}
        {apps.length === 0 && (
          <div className="text-center text-bq-text-secondary text-sm py-4">Scanning for apps...</div>
        )}
      </div>
    </div>
  );
}
