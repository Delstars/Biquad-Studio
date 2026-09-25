import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ChannelStrip } from "./ChannelStrip";
import { CrossFader } from "./CrossFader";

interface ChannelMixerProps {
  engineRunning: boolean;
}

const CHANNELS = [
  { id: "game", label: "Game", icon: "🎮", color: "text-indigo-400" },
  { id: "chat", label: "Chat", icon: "🎙️", color: "text-emerald-400" },
  { id: "media", label: "Media", icon: "🎵", color: "text-amber-400" },
  { id: "mic", label: "Mic", icon: "🎤", color: "text-rose-400" },
] as const;

export function ChannelMixer({ engineRunning }: ChannelMixerProps) {
  const [volumes, setVolumes] = useState<Record<string, number>>({
    game: 80,
    chat: 80,
    media: 80,
    mic: 80,
  });
  const [mutes, setMutes] = useState<Record<string, boolean>>({
    game: false,
    chat: false,
    media: false,
    mic: false,
  });
  const [crossfade, setCrossfade] = useState(50); // 0-100, 50 = center

  async function handleVolumeChange(channelId: string, value: number) {
    setVolumes((prev) => ({ ...prev, [channelId]: value }));
    if (engineRunning) {
      try {
        await invoke("plugin:audio|set_channel_volume", {
          channel: channelId,
          volume: value / 100,
        });
      } catch (err) {
        console.error(`Failed to set ${channelId} volume:`, err);
      }
    }
  }

  async function handleMuteToggle(channelId: string) {
    const newMuted = !mutes[channelId];
    setMutes((prev) => ({ ...prev, [channelId]: newMuted }));
    if (engineRunning) {
      try {
        await invoke("plugin:audio|set_channel_mute", {
          channel: channelId,
          muted: newMuted,
        });
      } catch (err) {
        console.error(`Failed to toggle ${channelId} mute:`, err);
      }
    }
  }

  async function handleCrossfadeChange(value: number) {
    setCrossfade(value);
    if (engineRunning) {
      try {
        await invoke("plugin:audio|set_crossfade", {
          position: value / 100,
        });
      } catch (err) {
        console.error("Failed to set crossfade:", err);
      }
    }
  }

  return (
    <div className="flex flex-col gap-6 h-full">
      {/* Channel Header */}
      <div className="flex items-center justify-between">
        <h2 className="text-sm font-semibold uppercase tracking-wider text-bq-text-secondary">
          Channel Mixer
        </h2>
        <span className="text-xs text-bq-text-muted">
          {engineRunning ? "Live" : "Preview"}
        </span>
      </div>

      {/* Channel Strips */}
      <div className="flex gap-4 flex-1 min-h-0">
        {CHANNELS.map((channel) => (
          <ChannelStrip
            key={channel.id}
            label={channel.label}
            icon={channel.icon}
            colorClass={channel.color}
            volume={volumes[channel.id]}
            muted={mutes[channel.id]}
            onVolumeChange={(v) => handleVolumeChange(channel.id, v)}
            onMuteToggle={() => handleMuteToggle(channel.id)}
            engineRunning={engineRunning}
          />
        ))}
      </div>

      {/* Game/Chat CrossFader */}
      <CrossFader
        value={crossfade}
        onChange={handleCrossfadeChange}
      />
    </div>
  );
}
