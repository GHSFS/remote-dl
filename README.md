<div align="center">

<p align="center">
  <strong>🇺🇸 English</strong> · <a href="./docs/locales/ko.md">🇰🇷 한국어</a> · <a href="./docs/locales/zh-CN.md">🇨🇳 中文</a> · <a href="./docs/locales/ja.md">🇯🇵 日本語</a> · <a href="./docs/locales/es.md">🇪🇸 Español</a><br />
  <a href="./docs/locales/ru.md">🇷🇺 Русский</a> · <a href="./docs/locales/vi.md">🇻🇳 Tiếng Việt</a> · <a href="./docs/locales/tr.md">🇹🇷 Türkçe</a> · <a href="./docs/locales/de.md">🇩🇪 Deutsch</a> · <a href="./docs/locales/pt-BR.md">🇧🇷 Português</a>
</p>

# remote-dl

> Stream-to-cloud download manager — pipe URLs directly to your personal cloud storage without staging on local disk.

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#-installation--deployment)

</div>

---

## 🛡️ Overview & Key Capabilities

<details open>
<summary><b>🛡️ Overview & Key Capabilities</b></summary>
<br>

`remote-dl` is a personal download manager for operators who want to fetch large files (videos, archives, datasets, research corpora) **directly into their own cloud storage** without first buffering them through a local machine.

A small native CLI client (`rdl.exe`) on your desktop coordinates with a lightweight serverless backend. The actual byte transfer happens entirely in the cloud, so neither your disk nor your home connection is involved in the data path.

This is a single-user, self-hosted tool. You bring your own cloud storage account; nothing is shared with other users.

</details>

---

## ⚡ Core Features

<details open>
<summary><b>⚡ Core Features</b></summary>
<br>

- **Memory-efficient streaming** — files of any size flow through a streaming pipeline; nothing is staged to disk on either client or backend.
- **OneDrive native** — chunked, resumable uploads via the Microsoft Graph API.
- **Single-user auth** — one-time passwords plus revocable persistent tokens; no account system, no third-party identity providers.
- **Multiple frontends** — desktop CLI, mobile-friendly web UI, and a private Telegram bot all share the same backend.
- **Trust boundary isolation** — credentials never leave the layer that owns them (operator → edge worker → automation runner → storage provider).
- **Self-rotating tokens** — OAuth refresh tokens are refreshed and persisted back into encrypted storage on every successful transfer, so the system survives indefinite periods of inactivity.

</details>

---

## 🏛️ Architecture & System Topology

<details open>
<summary><b>🏛️ Architecture & System Topology</b></summary>
<br>

```text
  ┌──────────────────────┐
  │  rdl.exe CLI         │
  │  Web UI              │ ──HTTPS──┐
  │  Telegram bot        │          │
  └──────────────────────┘          ▼
                              ┌──────────────────────┐
                              │  Edge worker         │
                              │  (auth + dispatch)   │
                              └──────────┬───────────┘
                                         │ workflow_dispatch
                                         ▼
                              ┌──────────────────────┐
                              │  Automation runner   │
                              │  (streaming proxy)   │
                              └──────────┬───────────┘
                                         │ chunked upload
                                         ▼
                              ┌──────────────────────┐
                              │  OneDrive            │
                              └──────────────────────┘
```

</details>

---

## 📂 Repository Layout

<details open>
<summary><b>📂 Repository Layout</b></summary>
<br>

```text
remote-dl/
├── Cargo.toml                 Package manifest. Defines the rdl binary, dependencies, and size-optimised release profile
├── Cargo.lock                 Pinned transitive dependency versions
├── rust-toolchain.toml        Toolchain pin (stable, x86_64-pc-windows-msvc)
├── .cargo/
│   └── config.toml            Build target + rustflags (static MSVC CRT)
├── .gitignore                 Excludes target/, secrets, IDE state, and private drafts
├── README.md                  This file
├── LICENSE                    MIT
├── CHANGELOG.md               Keep-a-Changelog format
├── docs/
│   └── locales/               Localized documentation (ko, zh-CN, ja, es, ru, vi, tr, de, pt-BR)
├── src/
│   ├── main.rs                CLI entry point. Parses clap args, dispatches handlers, colored output
│   ├── cli.rs                 clap derive definitions: get, list, status, config, auth, watch
│   ├── api.rs                 reqwest::blocking client for edge worker API
│   ├── config.rs              %APPDATA%\rdl\config.json read/write with Windows DPAPI encryption
│   └── error.rs               Crate-wide thiserror enum
├── tests/
│   └── integration.rs         assert_cmd CLI tests (--version, config roundtrip, error modes)
└── .github/workflows/
    ├── build.yml              CI: cargo build --release for rdl binary; uploads x64 artifact
    ├── test.yml               CI: cargo fmt, cargo clippy, cargo test --all-targets
    ├── download.yml           Backend automation: URL to OneDrive stream via curl | rclone rcat
    └── download-hls.yml       HLS / streaming automation variant (yt-dlp | rclone rcat)
```

</details>

---

## 💻 Compatibility & Runtime Targets

<details open>
<summary><b>💻 Compatibility & Runtime Targets</b></summary>
<br>

| Axis | Supported |
|---|---|
| Operating system (client binary) | Windows 10 1809+ and Windows 11 |
| Architecture | x86_64 only |
| Rust toolchain | 1.75+ (`rust-toolchain.toml` pins stable) |
| Linker | MSVC (Visual Studio Build Tools 2022) |
| Cloud storage | Microsoft OneDrive (Personal or Business) via rclone |
| Telegram client | any version that supports webhook bots |

The shipped client binary is statically linked against the MSVC runtime, so it has no DLL dependencies beyond standard Windows system libraries (`KERNEL32.DLL`, `USER32.DLL`, `ADVAPI32.DLL`, `CRYPT32.DLL`, `WS2_32.DLL`).

</details>

---

## 🔒 Security Considerations

<details open>
<summary><b>🔒 Security Considerations</b></summary>
<br>

- **Token at rest** — the bearer token in `%APPDATA%\rdl\config.json` is DPAPI-wrapped (`CryptProtectData`). Only the originating Windows user account can decrypt it.
- **Token in transit** — `reqwest` is built with `https_only=true`; tokens are sent in the `Authorization` header over TLS, never in the URL.
- **Auth surface** — only one Telegram user (`TG_OWNER_ID`) is allowed to command the bot; webhook calls require a secret in the URL path shared only with Telegram.
- **Workflow inputs** — every URL and filename passed to GitHub Actions workflows is masked with `::add-mask::` so it never appears in logs.
- **OAuth refresh** — the OneDrive refresh token is rotated and written back to the `RCLONE_CONF` repo secret on every successful run, surviving indefinite inactivity.
- **No telemetry** — neither the CLI nor the backend report any telemetry or usage metrics to third parties.

</details>

---

## 🛠️ Troubleshooting Matrix

<details open>
<summary><b>🛠️ Troubleshooting Matrix</b></summary>
<br>

| Symptom | Likely cause | Resolution |
|---|---|---|
| `worker URL not set` | first-time setup on this machine | `rdl config set worker https://<your-worker>.workers.dev` |
| `not authenticated — run rdl auth login` | no token cached | Issue a permanent token via Telegram bot, then `rdl auth login --token <token>` |
| `server rejected credentials (401)` | token revoked or expired | Issue a new token from the bot |
| `not found` from `rdl status <id>` | wrong job id, or run was deleted | `rdl list` to find the current job ids |
| Workflow run fails on OneDrive step | OneDrive token went stale (>90 days inactive) | `rclone config reconnect onedrive:` on desktop, then update `RCLONE_CONF` repo secret |
| Telegram bot stops replying | webhook secret mismatch or worker code error | `https://api.telegram.org/bot<TOKEN>/getWebhookInfo` to inspect; check Worker logs |

</details>

---

## 📦 Installation & Deployment

<details open>
<summary><b>📦 Installation & Deployment</b></summary>
<br>

### Pre-built binary (Windows x64)

1. Download `rdl-x64.exe` from the [Releases](https://github.com/GHSFS/remote-dl/releases) page.
2. Place it on your `PATH` (e.g. `C:\Tools\`).
3. Verify: `rdl --version`.

### Build from source

Requires Rust 1.75+ and the MSVC toolchain.

```bash
git clone https://github.com/GHSFS/remote-dl.git
cd remote-dl
cargo build --release
# Output: target/release/rdl.exe
```

</details>

---

## 🚀 Quick Start & CLI Reference

<details open>
<summary><b>🚀 Quick Start & CLI Reference</b></summary>
<br>

```bash
# 1. Point the client at your backend
rdl config set worker https://your-worker.example.workers.dev

# 2. Authenticate (one-time; obtain permanent token from bot first)
rdl auth login --token <permanent-token>

# 3. Queue a download
rdl https://example.com/dataset.tar.zst

# 4. Track progress
rdl list
rdl status <job-id>
```

### CLI Command Specifications

```text
USAGE:
    rdl [OPTIONS] <SUBCOMMAND>

SUBCOMMANDS:
    get, g <url>                Queue a URL for download (--name, --folder, --json)
    list, ls                    Show recent downloads (--limit, --json)
    status, st [job-id]         Show job status (--json)
    inspect <url>               Pre-flight inspect remote headers, size, range support (--json)
    batch <file>                Batch enqueue URLs from a text file (--folder, --json)
    ping                        Test worker connectivity and authentication (--json)
    config <get|set|path|show>  Read/write/diagnose client configuration
    auth <login|logout>         Manage DPAPI permanent authentication token
    watch                       Run in clipboard-monitor mode (rdl-tray companion)
    help                        Print help

OPTIONS:
    -j, --json                  Output machine-readable JSON
    -h, --help                  Print help information
    -V, --version               Print version information
```

</details>

---

## ⚙️ Configuration & Backend Setup

<details open>
<summary><b>⚙️ Configuration & Backend Setup</b></summary>
<br>

### Client Configuration

Config lives at `%APPDATA%\rdl\config.json`. Tokens are encrypted with Windows DPAPI so they cannot be read outside the originating user account.

| Key | Description |
|---|---|
| `worker` | Base URL of the deployed edge worker |
| `token` | Permanent auth token (DPAPI-encrypted) |
| `folder` | Default destination folder |

### Backend Setup

The automation backend runs via [`.github/workflows/download.yml`](./.github/workflows/download.yml) and [`.github/workflows/download-hls.yml`](./.github/workflows/download-hls.yml):

1. Configure `rclone` locally for OneDrive (`rclone config`) and store the resulting `rclone.conf` contents as the `RCLONE_CONF` repository secret.
2. Create a GitHub Personal Access Token with `repo` + `workflow` scopes and store it as `GH_PAT` (used to persist rotated OAuth refresh tokens back to `RCLONE_CONF`).
3. Deploy your Cloudflare edge worker pointing `workflow_dispatch` events at this repository, then configure `rdl config set worker <url>`.

</details>

---

## ❓ FAQ & Common Inquiries

<details open>
<summary><b>❓ FAQ & Common Inquiries</b></summary>
<br>

**Q. Why not just use `wget` / `curl` / `aria2c` locally?**
A. Because the data path goes through your home network and disk, which is the exact bottleneck this project removes. `remote-dl` makes a 100 GB transfer cost zero local disk and zero local bandwidth.

**Q. Can multiple users share an instance?**
A. No, this is intentionally a single-user tool. Multi-tenancy adds significant complexity to auth, quotas, and abuse handling that is out of scope.

**Q. Are downloads resumable?**
A. Yes — uploads to OneDrive are chunked and auto-resume on the upload side. The download side is one-shot per job; if it fails, re-queue.

</details>

---

## 🤝 Contributing & Acknowledgements

<details open>
<summary><b>🤝 Contributing & Acknowledgements</b></summary>
<br>

Before opening a PR:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

### Acknowledgements

- [`clap`](https://crates.io/crates/clap) — derive-based CLI argument parsing.
- [`reqwest`](https://crates.io/crates/reqwest) + [`rustls`](https://crates.io/crates/rustls) — TLS-only HTTP client.
- [`windows`](https://crates.io/crates/windows) — official Microsoft Win32 bindings (DPAPI).
- [`directories`](https://crates.io/crates/directories) — cross-platform config path resolution.
- [`rclone`](https://rclone.org) — chunked OneDrive upload engine inside GitHub Actions.

</details>

---

## 📄 License & Disclaimer

<details open>
<summary><b>📄 License & Disclaimer</b></summary>
<br>

Licensed under the **MIT License** ([LICENSE](./LICENSE)).

### Disclaimer

This is a personal tool intended for archiving content that you own or that you are authorized to download. The operator is solely responsible for complying with the terms of service of any source website and with applicable copyright law. **Do not use this tool to redistribute, mirror, proxy, or share content with third parties.**

</details>
