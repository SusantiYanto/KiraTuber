<div align="center">

<img src="docs/banner.svg" alt="KiraTuber — Anime PNGTuber Broadcast Studio" width="100%">

<br>

[![Latest Release](https://img.shields.io/github/v/release/SusantiYanto/KiraTuber?style=for-the-badge&logo=github&label=Release&color=00e5ff&labelColor=0b1222)](https://github.com/SusantiYanto/KiraTuber/releases/latest)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%2F11%20%7C%20Linux-ff2e7e?style=for-the-badge&logo=windows&labelColor=0b1222)](https://github.com/SusantiYanto/KiraTuber/releases/latest)
[![Built With](https://img.shields.io/badge/Engine-Tauri%20v2%20%2B%20Rust-00f5a0?style=for-the-badge&logo=rust&labelColor=0b1222)](https://v2.tauri.app/)
[![Installer Size](https://img.shields.io/badge/Setup%20Size-1.12%20MB-ffe600?style=for-the-badge&labelColor=0b1222)](https://github.com/SusantiYanto/KiraTuber/releases/latest)
[![License](https://img.shields.io/badge/License-MIT-a855f7?style=for-the-badge&labelColor=0b1222)](LICENSE)

<p>
  <strong>An ultra-lightweight 2D PNGTuber Studio styled like an Anime Broadcast HUD for OBS Studio, Discord, YouTube, &amp; Twitch.</strong><br>
  No heavy webcam tracking required — just plug in your microphone, upload PNG/WebP artwork, and go live at a silky-smooth 60 FPS.
</p>

<p>
  <a href="https://github.com/SusantiYanto/KiraTuber/releases/latest"><strong>⬇ Download KiraTuber</strong></a> ·
  <a href="#-key-features"><strong>✦ Key Features</strong></a> ·
  <a href="#-obs-studio--discord-guide"><strong>🎥 OBS Guide</strong></a> ·
  <a href="#-default-keyboard-shortcuts"><strong>⌨️ Hotkeys</strong></a> ·
  <a href="#-building-from-source"><strong>🛠️ Build from Source</strong></a>
</p>

</div>

---

## ✦ Workflow &amp; Studio Themes at a Glance

<p align="center">
  <img src="docs/preview-workflow.svg" alt="KiraTuber 3-Pose System & 4 Studio Themes" width="100%">
</p>

---

## 🚀 Key Features

| Category | Highlights |
| :--- | :--- |
| **⚡ Ultra-Lightweight (Rust + Tauri v2)** | Tiny **~1.1 MB** installer (**~3 MB** portable), GPU-composited animations (`translate3d` &amp; `scaleX`) with zero bloated libraries — minimal CPU &amp; RAM usage while running demanding games. |
| **👁️ 3-Pose System (`Idle` · `Blink` · `Talking`)** | Features **natural Auto-Blink** (adjustable interval &amp; duration + non-robotic random timing &amp; occasional double-blinks) alongside gentle idle breathing and smooth left-right sway. |
| **🎮 OBS Anti-Freeze While Gaming** | Specially optimized so character animations **never freeze in OBS Window Capture**, even when the KiraTuber window is occluded behind fullscreen games. |
| **🎙️ Precision Voice Detection + 2s Calibration** | Real-time VU Meter, adjustable *Noise Gate*, microphone *Noise Reduction*, and one-click **Auto-Calibrate Noise** — just stay quiet for 2 seconds. |
| **✨ Emote Deck &amp; 9 Special Movements** | 6 instant expression slots with custom emoji icons + 9 procedural stage motions (*Jump*, *Sway*, *Spin Twice*, *Pop*, *Shake*, *Bow*, *Float Up*, *Flash*, *Dance*), all bound to customizable hotkeys. |
| **🖼️ Built-In Crop &amp; Rotate Photo Editor** | Crop aspect ratios (`1:1`, `4:3`, `3:2`, `16:9`, portrait, or freeform) and fine-tune rotation angles directly inside the app before auto-saving locally to your device. |
| **🎨 4 Anime Broadcast Themes + Bilingual UI** | Ships with 4 studio themes (*Anime Dark*, *Schale Light*, *Starlight Idol*, *Cyber ZZZ*) and full **English** &amp; **Bahasa Indonesia** language support. |

---

## 📦 Downloads

Grab the ready-to-use packages from the [**GitHub Releases**](https://github.com/SusantiYanto/KiraTuber/releases/latest) page:

### 🪟 Windows (Windows 10 &amp; Windows 11)
| Package | File | Description |
| :--- | :--- | :--- |
| **Primary Installer (Recommended)** | `KiraTuber_0.1.0_windows_x64-setup.exe` | Lightweight &amp; convenient 64-bit NSIS setup (`1.12 MB`) |
| **Windows MSI (64-bit)** | `KiraTuber_0.1.0_windows_x64.msi` | Standard 64-bit MSI installer package (`1.59 MB`) |
| **Portable (64-bit)** | `KiraTuber_0.1.0_windows_x64_portable.exe` / `.zip` | Runs right out of the box with no installation |
| **Windows 32-bit (`x86`)** | `KiraTuber_0.1.0_windows_x86-setup.exe` / `.msi` / `.zip` | Full support for 32-bit Windows PCs |

### 🐧 Linux (`x86_64` / `amd64`)
| Distro / Format | File | Install / Run Command |
| :--- | :--- | :--- |
| **Ubuntu / Debian / Mint / Pop!_OS** | `KiraTuber_0.1.0_linux_amd64.deb` | `sudo apt install ./KiraTuber_0.1.0_linux_amd64.deb` |
| **Fedora / RHEL / openSUSE** | `KiraTuber_0.1.0_linux_x86_64.rpm` | `sudo dnf install ./KiraTuber_0.1.0_linux_x86_64.rpm` |
| **Universal Linux (AppImage)** | `KiraTuber_0.1.0_linux_amd64.AppImage` | `chmod +x KiraTuber_*.AppImage && ./KiraTuber_*.AppImage` |
| **Portable Tarball** | `KiraTuber_0.1.0_linux_x64_portable.tar.gz` | `tar -xzf KiraTuber_*_portable.tar.gz && ./KiraTuber-*/kiratuber` |

---

## 🎥 OBS Studio &amp; Discord Guide

> **💡 Language Tip:** To switch the app interface to English, open the **⚙️ Stage &amp; App** (*Layar &amp; App*) tab → **Theme &amp; language** (*Tema &amp; bahasa*), and select **English**.

### Capturing Your Avatar Without a Background in OBS Studio
1. Open **KiraTuber**, select your audio input under the **🎙️ Mic &amp; Motion** tab, and click **Start mic**.
2. Open the **⚙️ Stage &amp; App** tab, then set **Background color** to **Pure green (for OBS)** or **Transparent (no background)**.
3. Press **`H`** (or click the **`⇥`** icon in the top-right corner) to hide the studio control panel — leaving a 100% clean stage with no edge shadows.
4. In **OBS Studio**:
   - Add **Sources → Window Capture** → select **`[kiratuber.exe]: KiraTuber`**.
   - If using a green background: right-click the source → **Filters → `+` Chroma Key** (*Key Color Type: Green*).
   - If using a transparent background: enable *Allow Transparency* on capture sources that support alpha transparency.

### Using Your Avatar as a Camera in Discord / Zoom / Meet
1. Set up your avatar scene in **OBS Studio** following the steps above.
2. Click **Start Virtual Camera** in OBS Studio.
3. In **Discord → User Settings → Voice &amp; Video → Camera**, select **OBS Virtual Camera**.

---

## ⌨️ Default Keyboard Shortcuts

Every hotkey below can be freely customized at any time — simply click the *keycap* button next to its setting and press a new key:

| Default Key | Action |
| :---: | :--- |
| `H` | Hide / show the studio panel (clean stage mode for OBS) |
| `Q` `W` `E` `R` | Quick-switch active avatar (Character Slots 1 – 4) |
| `Z` `X` `C` `V` `B` `N` | Toggle Emotes / Expressions 1 – 6 |
| `1` – `9` | Trigger Special Movements 1 – 9 (*Jump*, *Sway*, *Spin Twice*, *Dance*, etc.) |
| `Ctrl + O` | **Return to normal** (instantly stop active expressions &amp; special movements) |

---

## 🛠️ Building from Source

### Project Structure
```text
KiraTuber/
├── src/
│   └── index.html          # Complete UI, WebAudio engine, & 60 FPS animations (zero external deps)
├── src-tauri/
│   ├── src/main.rs         # Tauri v2 Rust backend (Win32 DWM window control & mic permissions)
│   ├── capabilities/       # Tauri v2 window security capability definitions
│   └── tauri.conf.json     # Transparent window, app icons, & bundle configuration
├── docs/                   # Documentation illustrations & preview assets
└── DESIGN.md               # Anime Broadcast HUD design system documentation
```

### Prerequisites &amp; Build Commands
1. Install [Node.js](https://nodejs.org/) (v20+) and [Rust](https://rustup.rs/).
   - **Windows:** Make sure *Microsoft C++ Build Tools* &amp; *WebView2* are installed.
   - **Linux (Debian/Ubuntu):**
     ```bash
     sudo apt update
     sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev patchelf
     ```
2. Run in development mode or build production installers:
   ```bash
   npm install
   npm run dev     # Start development mode
   npm run build   # Build production release packages
   ```

---

<details>
<summary><strong>🇬🇧 English Summary & Quick Start (Click to Expand)</strong></summary>

<br>

**KiraTuber** is an ultra-lightweight, bloatware-free 2D **Anime PNGTuber Broadcast Studio** built with **Rust** and **Tauri v2**. It is designed for VTubers, gamers, and streamers who want a voice-reactive avatar without the CPU/GPU load of camera-based face tracking.

### Key Features
- **Extra Lightweight:** The installer is only ~1.1 MB, with 60 FPS GPU-composited animation and single-transaction asset storage in IndexedDB.
- **3-Pose Avatar System:** Supports **Idle**, **Auto-Blink** (with natural random intervals to avoid stiffness, plus an occasional double-blink), and **Talking** images, complete with breathing motion and subtle sway.
- **OBS Anti-Freeze During Fullscreen Games:** Optimized so animation keeps running smoothly in OBS Window Capture even when the app window is covered by a fullscreen game.
- **Expression Deck & Special Motions:** 6 expression slots with custom emoji icons + 9 procedural stage motions with freely configurable hotkeys.
- **Full Indonesian Language Support:** Change the interface language at any time via the **⚙️ Stage & App → Theme & language** menu.

</details>

---

<div align="center">
  <sub>Built with ✦ using <strong>Rust &amp; Tauri v2</strong> · Released under the <a href="LICENSE">MIT License</a></sub>
</div>
