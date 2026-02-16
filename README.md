# Zenith OS (2026 Research-Native)

**High-Performance | Memory-Safe | Zero-Trust | AI-Native**

Zenith is a dual-layer project comprising two distinct components:
1.  **Zenith Product (The App)**: A background productivity engine for macOS/Linux that optimizes your focus *today*.
2.  **Zenith Research (The Kernel)**: A next-gen Framekernel OS built from scratch in Rust for the *future*.

---

## 1. Zenith Product (The App)
**Status**: Release Candidate (v1.0)
**Runs On**: Your current Mac / Linux machine.

### ⚡ What it does
It runs quietly in the background, minimizing distractions and optimizing CPU resources based on your active context (e.g., Coding vs. Gaming).

### 🌟 Key Features
*   **Auto-Focus**: Detects when you open developer tools (VS Code, Terminal) and automatically silences notifications.
*   **Distraction Guard**: Warns or slows down social media apps during work hours.
*   **Live Dashboard**: View your real-time **Focus Score** at `http://localhost:9999`.
*   **Pro Mode**: Simulated "Pro" tier demonstrating feature gating.

### 🚀 How to Run (Quick Start)
```bash
# 1. Start the Daemon
cargo run -p zenith_daemon

# 2. Open dashboard
open http://localhost:9999
```

---

## 2. Zenith Research (The OS Kernel)
**Status**: Research Prototype
**Runs On**: QEMU Simulator / Bare Metal ARM64.

### ⚡ What it does
A clean-slate Operating System built from scratch in Rust. It explores new concepts in kernel design, unburdened by legacy code.

### 🌟 Key Features
*   **Instant Boot**: Boots in **<1 second**.
*   **Security Demo**: "Red Team" tests prove immunity to buffer overflows.
*   **AI Scheduler**: Uses "User Intent" instead of "Time Slices" to manage CPU.
*   **Semantic Shell**: Find files by meaning ("tax documents") via vector search.

### 🚀 How to Run (Simulation)
```bash
# Requires QEMU
make run-qemu
```

---

## 📊 Comparison Table

| Feature | Zenith Product (App) | Zenith Research (Kernel) |
| :--- | :--- | :--- |
| **Goal** | Improve Focus Today | Build OS of Tomorrow |
| **Where** | macOS / Linux | Simulator (QEMU) |
| **Risk** | Safe (User Space) | Experimental (Kernel Space) |
| **Install** | `cargo run -p zenith_daemon` | `make run-qemu` |

---

## 🛠️ Technology Stack
*   **Language**: Rust (Edition 2021)
*   **Build System**: Cargo Workspace, Makefile
*   **Security**: SandCell (Isolation), Ecdysis (Capabilities), HelixDB (Audit)
*   **Simulation**: QEMU with `hvf` acceleration
*   **Verification**: DeepSURF (Fuzzing), Red Team (Attack Simulation)

---
*Zenith OS is a research project exploring the future of safe, agentic operating systems.*
