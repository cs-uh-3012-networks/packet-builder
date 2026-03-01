# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Dioxus 0.7 desktop-only Rust application for a networking course (Spring 2026). Target environment is Ubuntu Linux. The main feature is a **Packet Byte Visualizer** — students fill in Ethernet, IP, UDP, custom TCP header, and payload fields and see a color-coded hex dump of the resulting raw bytes. Non-user-specified bytes (all Ethernet, most IP fields) display as "XX".

## Build & Serve Commands

Requires the Dioxus CLI (`dx`). Install with: `curl -sSL http://dioxus.dev/install.sh | sh`

- **Serve:** `dx serve`
- **Build:** `cargo build`
- **Clippy:** `cargo clippy`

Desktop-only (no web/mobile/server features). Tailwind CSS is handled automatically by Dioxus 0.7 — no separate build step needed.

## Architecture

**Entry point:** `src/main.rs` — defines the `Route` enum (with `#[derive(Routable)]`) and the `App` component that mounts `Router::<Route> {}`.

**Two-layer structure:**
- `src/views/` — Route-level components (`Home`, `Navbar` layout with `Outlet`)
- `src/components/` — Reusable UI components (`PacketForm`, `HexView`)
- `src/packet.rs` — Data structs (`IpFields`, `UdpFields`, `TcpFields`, `PacketState`) and byte serialization

**Routes:** `/` → `Home` (renders `PacketForm` + `HexView` in a two-panel layout). Wrapped in `Navbar` layout.

**Component tree:**
```
Home (owns Signal<PacketState>)
├── PacketForm (takes Signal<PacketState>, writes fields directly)
│     sections: Ethernet | IP | UDP | Custom TCP | Payload
└── HexView (takes Vec<AnnotatedByte>, purely presentational)
      renders 16 bytes/row, color-coded by layer, XX for placeholders
```

**Byte order:** IP/UDP headers use big-endian (network order). Custom TCP header uses little-endian (host order, matching the C struct on x86).

**Placeholder bytes:** Ethernet header bytes and non-editable IP fields (version, IHL, DSCP, total length, identification, flags, TTL, protocol, checksum) display as "XX" in the hex dump. Only user-specified fields (src/dst IP, ports, TCP fields, payload) show actual hex values.

**Key constants:** `MSS_SIZE=1500`, `ETH_HDR_SIZE=14`, `IP_HDR_SIZE=20`, `UDP_HDR_SIZE=8`, `TCP_HDR_SIZE=16`, `DATA_SIZE=1456`.

**Platform:** Desktop only (Dioxus desktop via system webview). No web/mobile/server features.

## Dioxus 0.7 Key Patterns

Refer to `AGENTS.md` for comprehensive Dioxus 0.7 API documentation. Critical differences from older versions:
- `cx`, `Scope`, and `use_state` are **removed** — use `use_signal` instead
- Signals: `use_signal` for local state, `.read()` / `.write()` for access
- Assets: `asset!("/assets/...")` macro with paths relative to project root
- Stylesheets: injected per-component via `document::Stylesheet` or `document::Link`

## Clippy Configuration

`clippy.toml` flags these types as unsafe to hold across `.await` points:
- `generational_box::GenerationalRef` / `GenerationalRefMut`
- `dioxus_signals::WriteLock`
