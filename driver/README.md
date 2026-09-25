# Biquad Studio Virtual Audio Driver (Phase 5)

This directory contains the scaffolding for the C++ **WaveRT Virtual Audio Miniport Driver**.

## Why a Kernel Driver?
In Phases 1-4, Biquad Studio leverages **Process Loopback** (capturing audio on a per-app basis from user-mode). While effective, the holy grail of virtual audio mixing (what Voicemeeter, SteelSeries Sonar, and Razer Synapse use) is a virtual kernel driver. 

A kernel driver allows us to spawn virtual endpoints in the Windows Sound control panel (e.g., `Biquad-Game`, `Biquad-Chat`, `Biquad-Media`). Users can select these as default playback devices natively in Windows or within specific games.

## Architecture
- **Language:** C++ (WDM kernel drivers cannot be written in pure Rust currently).
- **Framework:** PortCls (Audio Port Class) and WaveRT.
- **Base:** Microsoft `sysvad` sample.

## How to Build

> [!CAUTION]
> **Kernel Driver Development requires extreme care.** Bugs here will cause a Blue Screen of Death (BSOD) (`IRQL_NOT_LESS_OR_EQUAL`). Do not install this on your primary machine without testing in a VM first.

1. **Install Prerequisites**:
   - Visual Studio 2022 (with "Desktop development with C++" workload).
   - Windows SDK (Matching your OS version).
   - Windows Driver Kit (WDK) integrated with Visual Studio.
   
2. **Build**:
   - Open the `.sln` file in Visual Studio (To be generated).
   - Set the configuration to `Release / x64`.
   - Build Solution. This will produce `biquad_vad.sys`, `biquad-vad.inf`, and `biquad_vad.cat`.

3. **Deployment**:
   - Enable "Test Signing Mode" on your Windows machine (`bcdedit /set testsigning on` and reboot) because this driver is not signed with an EV Code Signing certificate.
   - Right-click `biquad-vad.inf` -> Install.
   - OR use `devcon.exe install biquad-vad.inf ROOT\BiquadVAD`.
