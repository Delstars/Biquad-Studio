# Biquad-Studio
A hardware-agnostic, low-latency gaming audio engine featuring dynamic channel routing, auto-calibration, and custom parametric EQ.

## 🔥 Key Features

* **Dynamic Audio Routing:** Registers distinct virtual endpoints (`Game`, `Chat`, `Media`, `Mic`) natively within Windows.
* **Hardware-Independent Game/Chat Balance:** Blend your communication and game tracks seamlessly using an in-app slider, tray controls, or system-wide global hotkeys.
* **Dual-Stage DSP Engine:**
  * **Stage 1 (Acoustic Calibration):** Automatically parses open-source acoustic databases to generate an inverse biquad filter curve, mathematically flattening your specific headset's hardware flaws.
  * **Stage 2 (Gaming Equalizer):** A fully customizable parametric EQ layer optimized for situational audio tuning (e.g., footstep prioritization, dialogue clarity).
* **Dolby Atmos Compatibility:** Feeds multi-channel audio directly into Microsoft's native Spatial Sound API, leveraging your existing system-wide Dolby Atmos for Headphones license cleanly without performance penalties.

---

## 🛠️ Technical Stack (Proposed)

* **UI Dashboard:** Tauri (Rust + React / TypeScript) for a memory-efficient, lightweight footprint.
* **Audio Core & Mixing Loop:** C++ or Rust (utilizing the JUCE framework or low-level WASAPI bindings).
* **Driver Architecture:** Custom Windows Audio Processing Object (APO) or kernel-mode virtual routing layer.
* **Global Input Hooks:** Low-level OS input interception (`rdev` / `iohook`) for game-focused shortcut triggers.

* LEGAL DISCLAIMER
----------------
Biquad Studio is an independent, community-driven open-source project created from scratch. It is not affiliated, associated, authorized, endorsed by, or in any way officially connected with SteelSeries, Razer, Corsair, Logitech, or any of their subsidiaries or affiliates. 

All product and company names, logos, or registered trademarks (including "SteelSeries GG", "Sonar", "Razer Synapse", and "Dolby Atmos") are trademarks™ or registered® trademarks of their respective holders. Use of them does not imply any affiliation with or endorsement by them, and they are used strictly under nominative fair use guidelines for architectural comparison purposes.
