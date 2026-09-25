# Biquad Studio 🎧

**Biquad Studio** is an open-source, hardware-agnostic, ultra-low-latency desktop audio suite built for PC gamers, streamers, and creators.

It provides a lightweight alternative to heavy proprietary audio ecosystems (like SteelSeries Sonar or Razer Synapse), offering professional-grade DSP processing, virtual multi-channel routing, and acoustic hardware auto-calibration at near-zero system cost.

![Biquad Studio UI](https://raw.githubusercontent.com/Delstars/Biquad-Studio/main/biquad_studio_ui.jpg) *(UI Preview)*

## ✨ Features

- **Hardware Agnostic**: Works with any headset (USB, 3.5mm analog, or wireless).
- **Virtual Audio Routing**: Assign Windows apps to dedicated `Game`, `Chat`, or `Media` channels to balance audio streams on the fly.
- **Parametric Gaming EQ**: A real-time 10-band interactive EQ graph with Robert Bristow-Johnson (RBJ) biquad filters.
- **AutoEQ Headphone Calibration**: Instantly correct the frequency response of over 8,800 supported headphone models to a neutral reference target.
- **Game/Chat CrossFader**: Balance your voice chat vs game volume with a single slider.
- **Spatial Audio Passthrough**: Hooks into Windows Sonic / Dolby Atmos for 7.1.4 object-based surround sound.
- **Global Hotkeys**: Control master volume (`Ctrl+Alt+Up/Down`) and mute (`Ctrl+Alt+M`) even while in exclusive full-screen games.
- **Ultra-Lightweight**: Built on Tauri (Rust + React) consuming only ~40-50MB of RAM—no background bloatware or mandatory accounts.

## 🚀 Installation

1. Navigate to the **[Releases](../../releases)** tab on GitHub.
2. Download the latest `Biquad Studio_x.x.x_x64-setup.exe` installer.
3. Run the installer and follow the prompt.
4. Launch **Biquad Studio** from your Start menu!

## 🛠️ Usage Instructions

### 1. Set Your Output Device
When you first open Biquad Studio, select your physical headset from the **Output Device** dropdown in the top right.

### 2. Auto-Calibrate Your Headset
Under the **Calibration** section, search for your headphone model (e.g., *Sennheiser HD 600*). Click **Apply** to instantly load its correction profile and flatten your headset's frequency response.

### 3. Route Your Audio
Under the **Routing Matrix**, you will see three faders: **Game**, **Chat**, and **Media**. 
*(Note: Full per-app virtual routing via Process Loopback is enabled under the hood. App assignment UI is coming in the next update!)*

Use the horizontal slider at the bottom to fade between your Game audio and Chat audio.

### 4. Create a Custom EQ
Click and drag the nodes on the **Parametric EQ** graph to boost footsteps, lower muddy bass, or fine-tune your gaming audio to your exact preference.

## 🏗️ Development & Building from Source

**Prerequisites:**
- Node.js (v18+)
- `pnpm` (`npm install -g pnpm`)
- Rust toolchain (`rustup`)
- *Windows Only:* MinGW/GNU or MSVC build tools.

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build the release installer (.exe)
pnpm tauri build
```

## 📜 License
MIT License. Copyright (c) 2026 Delstars.
