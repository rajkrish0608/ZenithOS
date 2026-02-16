# Zenith: Quick Start Guide

**Zenith** is your personal AI Focus Assistant. It runs quietly in the background, watches what you are doing, and automatically optimizes your Mac to help you stay in the zone.

## 🚀 How to Run It

### Option 1: For Developers (Source Code)
1.  Open Terminal.
2.  Navigate to the project folder.
3.  Run:
    ```bash
    cargo run -p zenith_daemon
    ```

### Option 2: For Users (App Mode)
*(Once built)*
1.  Double-click the `zenith_daemon` executable.
2.  Open your browser to: [http://localhost:9999](http://localhost:9999)

---

## ⚡ What Can Zenith Do? (Simple Terms)

1.  **Auto-Focus Mode**:
    *   If you open **VS Code** or **Terminal**, Zenith knows you are working.
    *   It automatically silences distractions and gives your code editor more CPU power so it runs faster.

2.  **Distraction Blocker**:
    *   If you are in "Focus Mode" and try to open a game or social media, Zenith can slow it down or block it to keep you on track.

3.  **Live Dashboard**:
    *   See exactly how focused you are with a real-time **Focus Score** (0-100).
    *   Manually switch between **Standard**, **Focus**, and **Chill** modes.

4.  **Privacy First**:
    *   Everything happens on your Mac. No data is sent to the cloud.

---

## 📦 How to "Go Live" (Release to Users)

To share this with friends or testers:

1.  **Build the App**:
    Run this command to create a fast, optimized version:
    ```bash
    cargo build --release -p zenith_daemon
    ```

2.  **Locate the File**:
    The shareable file is at:
    `target/release/zenith_daemon`

3.  **Share**:
    Send that file (and the `static` folder) to your users. They just need to run it!
