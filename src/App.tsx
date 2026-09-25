import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ChannelMixer } from "./features/channel-mixer/components/ChannelMixer";
import { EqCanvas } from "./features/eq-graph/components/EqCanvas";
import { DeviceSelector } from "./features/device-manager/components/DeviceSelector";
import { StatusBar } from "./shared/components/StatusBar";
import { Sidebar } from "./shared/components/Sidebar";
import { CloudSync } from "./features/cloud-sync/components/CloudSync";
import { AutoEqSelector } from "./features/headset-calibration/components/AutoEqSelector";
import { AppRouter } from "./features/app-router/components/AppRouter";

/** Audio device info returned from the Rust backend */
export interface AudioDeviceInfo {
  id: string;
  name: string;
  is_default: boolean;
  sample_rate: number;
  channels: number;
}

/** Engine status returned from the Rust backend */
export interface EngineStatus {
  running: boolean;
  sample_rate: number;
  buffer_frames: number;
  output_device: string | null;
}

import { listen } from "@tauri-apps/api/event";

function App() {
  const [engineRunning, setEngineRunning] = useState(false);
  const [outputDevices, setOutputDevices] = useState<AudioDeviceInfo[]>([]);
  const [selectedDevice, setSelectedDevice] = useState<string | null>(null);
  
  const [masterVolume, setMasterVolume] = useState(100);
  const [masterMute, setMasterMute] = useState(false);

  useEffect(() => {
    // Enumerate audio devices on mount
    loadDevices();
    
    // Listen for global hotkeys
    const unlistenMute = listen("hotkey-mute-toggle", () => {
      setMasterMute(m => !m);
    });
    const unlistenVolUp = listen("hotkey-volume-up", () => {
      setMasterVolume(v => Math.min(100, v + 5));
    });
    const unlistenVolDown = listen("hotkey-volume-down", () => {
      setMasterVolume(v => Math.max(0, v - 5));
    });

    return () => {
      unlistenMute.then(f => f());
      unlistenVolUp.then(f => f());
      unlistenVolDown.then(f => f());
    };
  }, []);

  // Sync master volume to backend
  useEffect(() => {
    invoke("plugin:audio|set_master_volume", { volume: masterVolume / 100 }).catch(console.error);
  }, [masterVolume]);

  // Sync master mute to backend
  useEffect(() => {
    invoke("plugin:audio|set_master_mute", { muted: masterMute }).catch(console.error);
  }, [masterMute]);

  async function loadDevices() {
    try {
      const devices = await invoke<AudioDeviceInfo[]>("plugin:audio|list_render_devices");
      setOutputDevices(devices);
      // Auto-select the default device
      const defaultDevice = devices.find((d) => d.is_default);
      if (defaultDevice && !selectedDevice) {
        setSelectedDevice(defaultDevice.id);
      }
    } catch (err) {
      console.error("Failed to enumerate devices:", err);
    }
  }

  async function toggleEngine() {
    try {
      if (engineRunning) {
        await invoke("plugin:audio|stop_engine");
        setEngineRunning(false);
      } else {
        await invoke("plugin:audio|start_engine", {
          deviceId: selectedDevice,
        });
        setEngineRunning(true);
      }
    } catch (err) {
      console.error("Engine toggle failed:", err);
    }
  }

  async function handleDeviceChange(deviceId: string) {
    setSelectedDevice(deviceId);
    if (engineRunning) {
      // Restart engine with new device
      try {
        await invoke("plugin:audio|stop_engine");
        await invoke("plugin:audio|start_engine", { deviceId });
      } catch (err) {
        console.error("Device switch failed:", err);
      }
    }
  }

  return (
    <div className="flex h-screen w-screen bg-bq-bg">
      {/* Sidebar Navigation */}
      <Sidebar />

      {/* Main Content Area */}
      <main className="flex-1 flex flex-col overflow-hidden">
        {/* Top Bar with Device Selector and Engine Toggle */}
        <header className="flex items-center justify-between px-6 py-3 border-b border-bq-border bg-bq-bg-secondary/50">
          <div className="flex items-center gap-4">
            <h1 className="text-lg font-semibold text-bq-text">
              <span className="text-bq-accent">Biquad</span> Studio
            </h1>
            <div className="h-5 w-px bg-bq-border" />
            <DeviceSelector
              devices={outputDevices}
              selectedId={selectedDevice}
              onDeviceChange={handleDeviceChange}
              onRefresh={loadDevices}
            />
          </div>
          <div className="flex items-center gap-6">
            {/* Master Volume Controls */}
            <div className="flex items-center gap-2">
              <button
                onClick={() => setMasterMute(!masterMute)}
                className={`w-8 h-8 flex items-center justify-center rounded-md ${
                  masterMute ? "bg-red-500/20 text-red-500" : "bg-bq-bg-tertiary text-bq-text-secondary hover:text-white"
                }`}
                title="Master Mute (Ctrl+Alt+M)"
              >
                {masterMute ? "🔇" : "🔊"}
              </button>
              <input
                type="range"
                min="0"
                max="100"
                value={masterVolume}
                onChange={(e) => setMasterVolume(Number(e.target.value))}
                className="w-24 h-1.5 bg-bq-bg-tertiary rounded-lg appearance-none cursor-pointer"
                title="Master Volume (Ctrl+Alt+Up/Down)"
              />
            </div>

            <button
              onClick={toggleEngine}
              className={`px-4 py-1.5 rounded-md text-sm font-medium transition-all duration-200 ${
                engineRunning
                  ? "bg-bq-meter-green/20 text-bq-meter-green border border-bq-meter-green/30 hover:bg-bq-meter-green/30"
                  : "bg-bq-bg-tertiary text-bq-text-secondary border border-bq-border hover:border-bq-border-active"
              }`}
            >
              {engineRunning ? "● Engine Running" : "○ Engine Stopped"}
            </button>
          </div>
        </header>

        {/* Main Interface */}
        <div className="flex-1 p-6 overflow-auto flex flex-col gap-6">
          {/* EQ Editor Section */}
          <section className="flex flex-col gap-3">
            <h2 className="text-sm font-medium text-bq-text-secondary uppercase tracking-wider">Parametric EQ</h2>
            <EqCanvas />
          </section>

          {/* AutoEQ Calibration */}
          <section className="flex flex-col gap-3">
            <h2 className="text-sm font-medium text-bq-text-secondary uppercase tracking-wider">Calibration</h2>
            <AutoEqSelector />
          </section>

          {/* App Routing */}
          <section className="flex flex-col gap-3">
            <h2 className="text-sm font-medium text-bq-text-secondary uppercase tracking-wider">Process Routing</h2>
            <AppRouter />
          </section>

          {/* Channel Mixer Section */}
          <section className="flex-1 flex flex-col gap-3">
            <h2 className="text-sm font-medium text-bq-text-secondary uppercase tracking-wider">Routing Matrix</h2>
            <ChannelMixer engineRunning={engineRunning} />
          </section>

          {/* Cloud Sync Section */}
          <section className="flex flex-col gap-3 pb-8">
            <CloudSync />
          </section>
        </div>

        {/* Status Bar */}
        <StatusBar engineRunning={engineRunning} sampleRate={48000} />
      </main>
    </div>
  );
}

export default App;
