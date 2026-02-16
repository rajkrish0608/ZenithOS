# Zenith: Product vs Research

This document breaks down the two distinct parts of the Zenith project: **The App** (Product) and **The OS** (Research).

---

## 1. Zenith Product (The App)
**Status**: Release Candidate (v1.0)
**Location**: `dist/Zenith_Product` (Runs on macOS/Linux)

### ⚡ What it does
It runs quietly in the background on your current computer, watching your activity context (e.g., Coding vs Gaming) and automatically adjusting system settings to help you focus.

### 🌟 Key Features
*   **Auto-Focus**: If you open **VS Code** or **Terminal**, it automatically silences notifications and boosts CPU priority for your work tools.
*   **Distraction Blocker**: If you try to open a game or social media during work hours, it slows it down or warns you to stay on track.
*   **Live Dashboard**: Shows a real-time **Focus Score** (0-100) at `http://localhost:9999`.
*   **Pro Mode**: A simulated "Pro" tier that unlocks advanced features (for demonstration purposes).

### 🎯 Who is it for?
*   **Knowledge Workers**: Developers, Writers, and Designers who want to be more productive on their existing Mac.
*   **Self-Improvers**: Anyone looking to quantify and improve their daily focus.

---

## 2. Zenith Research (The OS Kernel)
**Status**: Research Prototype
**Location**: `dist/Zenith_Research_Kernel` (Runs in QEMU Simulator)

### ⚡ What it does
It is a tiny, custom-built Operating System that boots from scratch on bare metal (ARM64). It does **not** rely on macOS, Windows, or Linux. It is a clean-slate reimagining of how an OS should work.

### 🌟 Key Features
*   **Instant Boot**: Boots in **<1 second**, demonstrating extreme efficiency.
*   **Security Demo**: Contains a "Red Team" test where it blocks a hacker attack (buffer overflow) that would crash traditional systems.
*   **AI Scheduler**: Demonstrates a new way of managing CPU tasks using **User Intent** instead of just "Time Slicing".
*   **Semantic Shell**: Allows finding files by meaning ("my tax project") rather than exact filenames.

### 🎯 Who is it for?
*   **Systems Engineers**: Developers interested in low-level OS architecture, Rust kernels, and embedded systems.
*   **Researchers**: Those exploring the future of safe, AI-native operating systems.

---

## 🚀 Summary: Which do I use?

| Feature | Zenith Product (App) | Zenith Research (Kernel) |
| :--- | :--- | :--- |
| **Goal** | Improve Focus Today | Build OS of Tomorrow |
| **Where** | Your current Mac | Simulator (QEMU) |
| **Risk** | Safe (User Space) | Experimental (Kernel Space) |
| **Install** | `cargo run -p zenith_daemon` | `make run-qemu` |
