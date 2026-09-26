import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ChannelMixer } from "./features/channel-mixer/components/ChannelMixer";
import { EqCanvas } from "./features/eq-graph/components/EqCanvas";
import { DeviceSelector } from "./features/device-manager/components/DeviceSelector";
import { StatusBar } from "./shared/components/StatusBar";
import { AutoEqSelector } from "./features/headset-calibration/components/AutoEqSelector";
import { listen } from "@tauri-apps/api/event";

export interface AudioDeviceInfo {
  id: string;
  name: string;
  is_default: boolean;
  sample_rate: number;
  channels: number;
}

const TABS = ["Mixer", "Game", "Chat", "Media", "Aux", "Mic"] as const;
type TabId = typeof TABS[number];

function App() {
  const [engineRunning, setEngineRunning] = useState(false);
  const [outputDevices, setOutputDevices] = useState<AudioDeviceInfo[]>([]);
  const [selectedDevice, setSelectedDevice] = useState<string | null>(null);
  
  const [masterVolume, setMasterVolume] = useState(100);
  const [masterMute, setMasterMute] = useState(false);
  const [activeTab, setActiveTab] = useState<TabId>("Mixer");
  const [isDark, setIsDark] = useState(true);
  const [showPresetBrowser, setShowPresetBrowser] = useState(false);

  useEffect(() => {
    loadDevices();
    
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

  useEffect(() => {
    invoke("plugin:audio|set_master_volume", { volume: masterVolume / 100 }).catch(console.error);
  }, [masterVolume]);

  useEffect(() => {
    invoke("plugin:audio|set_master_mute", { muted: masterMute }).catch(console.error);
  }, [masterMute]);

  const toggleTheme = () => {
    const html = document.documentElement;
    if (html.classList.contains("dark")) {
      html.classList.remove("dark");
      setIsDark(false);
    } else {
      html.classList.add("dark");
      setIsDark(true);
    }
  };

  async function loadDevices() {
    try {
      const devices = await invoke<AudioDeviceInfo[]>("plugin:audio|list_render_devices");
      setOutputDevices(devices);
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
        await invoke("plugin:audio|start_engine", { deviceId: selectedDevice });
        setEngineRunning(true);
      }
    } catch (err) {
      console.error("Engine toggle failed:", err);
    }
  }

  async function handleDeviceChange(deviceId: string) {
    setSelectedDevice(deviceId);
    if (engineRunning) {
      try {
        await invoke("plugin:audio|stop_engine");
        await invoke("plugin:audio|start_engine", { deviceId });
      } catch (err) {
        console.error("Device switch failed:", err);
      }
    }
  }

  return (
    <div className="flex h-screen w-screen bg-bq-bg text-bq-text font-sans">
      <main className="flex-1 flex flex-col overflow-hidden relative">
        {/* Top Navbar */}
        <header className="flex items-center justify-between px-8 pt-6 pb-4 border-b border-bq-border bg-bq-bg">
          <div className="flex items-center gap-8">
            <h1 className="text-xl font-semibold tracking-wide text-bq-text">
              Sonar
            </h1>
            <nav className="flex gap-6">
              {TABS.map(tab => (
                <button
                  key={tab}
                  onClick={() => setActiveTab(tab)}
                  className={`text-sm font-medium pb-2 border-b-2 transition-colors ${
                    activeTab === tab 
                      ? "border-bq-accent text-bq-accent" 
                      : "border-transparent text-bq-text-secondary hover:text-bq-text"
                  }`}
                >
                  {tab}
                </button>
              ))}
            </nav>
          </div>
          <div className="flex items-center gap-4">
            <button
              onClick={toggleTheme}
              className="p-2 rounded-full hover:bg-bq-bg-secondary text-bq-text-secondary transition-colors"
              title="Toggle Light/Dark Mode"
            >
              {isDark ? "☀️" : "🌙"}
            </button>
            <DeviceSelector
              devices={outputDevices}
              selectedId={selectedDevice}
              onDeviceChange={handleDeviceChange}
              onRefresh={loadDevices}
            />
            <button
              onClick={toggleEngine}
              className={`px-4 py-1.5 rounded-md text-sm font-medium transition-all duration-200 ${
                engineRunning
                  ? "bg-bq-meter-green text-white shadow-sm"
                  : "bg-bq-bg-tertiary text-bq-text-secondary border border-bq-border hover:border-bq-border-active"
              }`}
            >
              {engineRunning ? "Live" : "Stopped"}
            </button>
          </div>
        </header>

        {/* Content Area */}
        <div className="flex-1 p-6 overflow-auto relative">
          {activeTab === "Mixer" && (
            <div className="h-full">
              <ChannelMixer engineRunning={engineRunning} />
            </div>
          )}
          
          {activeTab !== "Mixer" && (
            <div className="flex flex-col gap-6 max-w-5xl mx-auto relative h-full">
              {/* Preset Browser Overlay */}
              {showPresetBrowser && (
                <div className="absolute inset-0 bg-bq-bg/95 backdrop-blur-sm z-50 p-6 rounded-lg border border-bq-border flex flex-col">
                  <div className="flex justify-between items-center mb-6">
                    <h2 className="text-xl font-bold flex items-center gap-2">
                      BROWSE <span className="text-bq-accent">🎮</span>
                    </h2>
                    <button onClick={() => setShowPresetBrowser(false)} className="text-bq-text-secondary hover:text-white">✕</button>
                  </div>
                  
                  <div className="flex items-center justify-between mb-6">
                    <div className="flex gap-4 border-b border-bq-border/50 pb-2">
                      <button className="text-bq-accent font-medium px-2 border-b-2 border-bq-accent pb-2 -mb-[10px]">All <span className="text-xs bg-bq-accent/20 px-1.5 rounded ml-1">349</span></button>
                      <button className="text-bq-text-secondary hover:text-white px-2">Biquad Studio <span className="text-xs bg-bq-bg-tertiary px-1.5 rounded ml-1">341</span></button>
                      <button className="text-bq-text-secondary hover:text-white px-2">Favorites <span className="text-xs bg-bq-bg-tertiary px-1.5 rounded ml-1">1</span></button>
                      <button className="text-bq-text-secondary hover:text-white px-2">My presets <span className="text-xs bg-bq-bg-tertiary px-1.5 rounded ml-1">8</span></button>
                      <button className="text-bq-text-secondary hover:text-white px-2 flex items-center gap-1">🔥 New <span className="text-xs bg-bq-bg-tertiary px-1.5 rounded ml-1">8</span></button>
                    </div>
                    <button className="bg-bq-accent hover:bg-bq-accent-hover text-white px-4 py-1.5 rounded-md text-sm font-medium">+ Add New</button>
                  </div>

                  <div className="mb-6 relative w-1/3">
                    <span className="absolute left-3 top-1.5 text-bq-text-secondary">🔍</span>
                    <input type="text" placeholder="Search" className="w-full bg-bq-bg-tertiary border border-bq-accent rounded-md py-1.5 pl-9 pr-4 text-sm text-white focus:outline-none" />
                  </div>

                  <div className="grid grid-cols-3 gap-4 overflow-y-auto">
                    {["Arc Raiders Warm", "Gaming", "Nova 5 Wireless Clarity\\Bassy", "Custom", "Golden Standard", "RPG-Cinema", "CUSTOM Predator", "Musik / Youtube", "#Blud"].map(preset => (
                      <button key={preset} className={`bg-bq-bg-secondary border p-4 rounded-lg flex items-center justify-between group transition-colors ${preset === 'RPG-Cinema' ? 'border-bq-accent bg-bq-accent/10' : 'border-bq-border hover:border-bq-border-active'}`}>
                        <div className="flex items-center gap-3">
                          <div className={`w-10 h-10 rounded-lg flex items-center justify-center ${preset === 'RPG-Cinema' ? 'bg-bq-accent' : 'bg-bq-bg-tertiary group-hover:bg-bq-border'}`}>
                            <span className={preset === 'RPG-Cinema' ? 'text-white' : 'text-bq-text-secondary'}>🎮</span>
                          </div>
                          <span className="font-medium text-sm">{preset}</span>
                          <span className="text-bq-text-muted text-xs">🔗</span>
                        </div>
                        <div className="flex flex-col items-end gap-2">
                          <span className="text-bq-text-muted hover:text-bq-meter-yellow text-sm">☆</span>
                          <span className="text-bq-text-muted hover:text-white">⋯</span>
                        </div>
                      </button>
                    ))}
                  </div>
                </div>
              )}

              <div className="flex items-center justify-between mb-4">
                <div className="flex items-center gap-4">
                  <div className="w-12 h-12 bg-bq-accent rounded-lg flex items-center justify-center shadow-lg shadow-bq-accent/20">
                    <span className="text-white text-2xl">🎮</span>
                  </div>
                  <div>
                    <h2 className="text-xl font-bold flex items-center gap-2">RPG-Cinema <span className="text-bq-text-muted text-sm cursor-pointer">🔗</span></h2>
                    <span className="text-sm text-bq-text-secondary flex items-center gap-1"><span className="w-1.5 h-1.5 rounded-full bg-bq-accent"></span> PRESET</span>
                  </div>
                </div>
                
                <div className="flex items-center gap-4">
                   <button 
                     onClick={() => setShowPresetBrowser(true)}
                     className="w-10 h-10 bg-bq-bg-tertiary rounded-lg border border-bq-border flex items-center justify-center text-bq-text-secondary hover:text-white hover:border-bq-border-active transition-colors"
                   >
                     🔍
                   </button>
                   <div className="flex gap-2 p-1 bg-bq-bg-secondary rounded-lg border border-bq-border">
                     <button className="w-16 h-12 bg-bq-bg-tertiary rounded flex flex-col items-center justify-center gap-1 border border-bq-border">
                       <span className="text-bq-accent text-lg">🎮</span>
                     </button>
                     <button className="w-16 h-12 bg-bq-bg rounded flex flex-col items-center justify-center gap-1 border border-bq-border/50 text-bq-text-secondary">
                       <span className="text-lg">🔊</span>
                     </button>
                     <button className="w-16 h-12 bg-bq-bg rounded flex flex-col items-center justify-center gap-1 border border-bq-border/50 text-bq-text-secondary">
                       <span className="text-lg">🎵</span>
                     </button>
                   </div>
                </div>
              </div>

              <section className="bq-panel p-6">
                <div className="flex items-center justify-between mb-4">
                  <h3 className="font-semibold flex items-center gap-2">
                    <div className="w-10 h-6 bg-bq-accent rounded-full relative">
                      <div className="absolute right-1 top-1 w-4 h-4 bg-white rounded-full"></div>
                    </div>
                    SPATIAL AUDIO
                  </h3>
                  <div className="flex gap-2">
                    <button className="bg-bq-accent text-white px-3 py-1 rounded text-sm font-medium">HEADPHONE MODE</button>
                    <button className="bg-bq-bg-tertiary text-bq-text-secondary border border-bq-border px-3 py-1 rounded text-sm font-medium">SPEAKER MODE</button>
                  </div>
                </div>
                
                <div className="flex gap-8 items-center bg-bq-bg p-6 rounded-lg border border-bq-border">
                  <div className="w-48 h-48 rounded-full border-2 border-bq-border flex items-center justify-center relative">
                    <div className="w-8 h-4 bg-bq-text rounded-full"></div>
                    {/* Dots for spatial visualization */}
                    {[0,45,90,135,180,225,270,315].map(deg => (
                      <div key={deg} className="absolute w-4 h-4 bg-bq-accent rounded-full shadow-lg shadow-bq-accent/50" 
                           style={{ transform: `rotate(${deg}deg) translateY(-85px)` }}>
                      </div>
                    ))}
                  </div>
                  <div className="flex-1 flex flex-col gap-4">
                    <p className="text-sm text-bq-text-secondary leading-relaxed">
                      Tune slider towards Performance to improve the senses of directionality and localization, which are ideal for competitive FPS games. Or tune it towards Immersion to improve the environmental effects for a more realistic surround experience.
                    </p>
                    <div>
                      <div className="flex justify-between text-xs font-medium mb-1">
                        <span>Performance</span>
                        <span>Immersion</span>
                      </div>
                      <input type="range" className="w-full h-2 bg-bq-bg-tertiary rounded-lg appearance-none cursor-pointer" />
                    </div>
                    <div>
                      <div className="flex justify-between text-xs font-medium mb-1">
                        <span>Distance</span>
                        <span>70</span>
                      </div>
                      <input type="range" className="w-full h-2 bg-bq-bg-tertiary rounded-lg appearance-none cursor-pointer" defaultValue={70} />
                    </div>
                  </div>
                </div>
              </section>

              <section className="bq-panel p-6 flex flex-col gap-4">
                 <div className="flex items-center justify-between">
                  <h3 className="font-semibold flex items-center gap-2">
                    <div className="w-10 h-6 bg-bq-accent rounded-full relative">
                      <div className="absolute right-1 top-1 w-4 h-4 bg-white rounded-full"></div>
                    </div>
                    EQUALIZER
                  </h3>
                </div>
                <div className="h-64">
                   <EqCanvas />
                </div>
                <AutoEqSelector />
              </section>
            </div>
          )}
        </div>
        
        {/* Subdued Status Bar */}
        <StatusBar engineRunning={engineRunning} sampleRate={48000} />
      </main>
    </div>
  );
}

export default App;
