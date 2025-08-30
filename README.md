# 🦀 CrabNest

CrabNest is a **proof-of-concept Command & Control (C2) framework** written in Rust.
It is designed for **educational, research, and red team lab use only**.

> ⚠️ Disclaimer: This tool is intended **only for use in controlled environments** (labs, CTFs, personal research).
> Do not deploy against systems you do not own or have explicit permission to test.

---

## ✨ Features
- Lightweight **Rust-based C2 server**
- Encrypted communications (`rustls`, AES/ChaCha)
- Implant/agent that:
  - Beacons to C2
  - Executes remote commands
  - Returns results securely
- Operator CLI for tasking and monitoring

---

## 🚧 Roadmap
See [TODO.md](./TODO.md) for a breakdown of planned features:
- Core: C2 server, agents, operator console
- Comms: HTTP(S), encryption, beacon jitter
- Advanced: persistence modules, exfiltration, file handling
- Optional: Web UI dashboard, OPSEC features

---

## 🛠️ Build
Requirements:
- Rust (latest stable)
- Cargo

```bash
git clone https://github.com/lustarm/crabnest.git
cd crabnest
cargo build --release

