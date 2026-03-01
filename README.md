# Packet Builder

A desktop application for visualizing network packet byte layout. Students fill in Ethernet, IP, UDP, custom TCP header, and payload fields and see a color-coded hex dump of the resulting raw bytes. Hovering over hex bytes or header fields highlights the corresponding bytes and brings up the associated header diagram.

Built with [Dioxus](https://dioxuslabs.com/) (Rust) for the Spring 2026 Networks course.

## Prerequisites (Ubuntu)

Install the required system libraries:

```bash
sudo apt update
sudo apt install \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  lld
```

Install Rust (if not already installed):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Running

```bash
git clone https://github.com/cs-uh-3012-networks/packet-builder.git
cd packet-builder
cargo run --release
```

If the app launches but shows a black window, set this environment variable and try again:

```bash
export WEBKIT_DISABLE_COMPOSITING_MODE=1
cargo run --release
```

## AI Disclosure

This repository was vibe coded with [Claude Code](https://claude.ai/code).
