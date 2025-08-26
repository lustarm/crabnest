# CrabNest TODO 🦀

## Core Features
- [ ] **C2 Server**
  - [ ] HTTP(S) listener (with `warp` or `axum`)
  - [ ] TLS encryption (via `rustls`)
  - [ ] Basic authentication / API keys
  - [ ] Command dispatching (list connected agents, send commands)

- [ ] **Agent (Implant)**
  - [ ] Beaconing with jitter (configurable intervals)
  - [ ] Fetch & execute commands
  - [ ] Collect basic system info (OS, hostname, username)
  - [ ] Command execution (run shell commands, return output)

## Communication
- [ ] **Transport Layers**
  - [ ] HTTP/HTTPS POST for C2 traffic
  - [ ] Encrypted JSON (via `serde` + `aes-gcm` or `chacha20poly1305`)
  - [ ] Support fallback to DNS tunneling (stretch goal)

## Operator Console
- [ ] **CLI Tool**
  - [ ] List agents
  - [ ] Send commands
  - [ ] Retrieve responses
  - [ ] Logging system

## Advanced Features
- [ ] **Persistence Modules** (lab/demo use only)
  - [ ] File dropper
  - [ ] Registry modification (Windows)
- [ ] **Exfiltration**
  - [ ] Upload/download files
- [ ] **Payload Handling**
  - [ ] Encoder/decoder (Base64, XOR, AES)
  - [ ] Stager vs full agent

## Nice-to-Haves
- [ ] Web UI dashboard (React or Yew)
- [ ] Multi-agent support with tags
- [ ] OPSEC features (random user agents, sleep schedules, canaries)

---

