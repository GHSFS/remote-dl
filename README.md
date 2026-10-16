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
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

### Overview

`remote-dl` is a personal download manager for users who want to fetch large files
(videos, archives, datasets, research corpora) **directly into their own cloud
storage** without first buffering them through a local machine.

A small native CLI client (`rdl.exe`) on your desktop coordinates with a
lightweight serverless backend. The actual byte transfer happens entirely in the
cloud, so neither your disk nor your home connection is involved in the data path.

This is a single-user, self-hosted tool. You bring your own cloud storage
account; nothing is shared with other users.

### Features

- **Memory-efficient streaming** — files of any size flow through a streaming
  pipeline; nothing is staged to disk on either client or backend
- **OneDrive native** — chunked, resumable uploads via the Microsoft Graph API
- **Single-user auth** — one-time passwords plus revocable persistent tokens; no
  account system, no third-party identity providers
- **Multiple frontends** — desktop CLI, mobile-friendly web UI, and a private
  Telegram bot all share the same backend
- **Trust boundary isolation** — credentials never leave the layer that owns
  them (operator → edge worker → automation runner → storage provider)
- **Self-rotating tokens** — OAuth refresh tokens are refreshed and persisted
  back into encrypted storage on every successful transfer, so the system
  survives indefinite periods of inactivity

### Architecture

```
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

### Repository layout

```
remote-dl/
├── Cargo.toml                 Package manifest. Defines the rdl binary,
│                              dependencies, and the size-optimised release
│                              profile (opt-level=z, lto, strip, panic=abort)
├── Cargo.lock                 Pinned transitive dependency versions
├── rust-toolchain.toml        Toolchain pin (stable, x86_64-pc-windows-msvc)
├── .cargo/
│   └── config.toml            Build target + rustflags (static MSVC CRT)
├── .gitignore                 Excludes target/, secrets, IDE state, the
│                              private worker.js, and root-level workflow
│                              drafts that aren't meant for the public tree
├── README.md                  This file
├── LICENSE                    MIT
├── CHANGELOG.md               Keep-a-Changelog format
├── docs/
│   └── locales/               Localized documentation (ko, ja, zh-CN, ru, vi, tr, de, es, pt-BR)
├── src/
│   ├── main.rs                CLI entry point. Parses clap args, dispatches
│   │                          to per-subcommand handlers, formats colored
│   │                          terminal output
│   ├── cli.rs                 clap derive definitions: get, list, status,
│   │                          config, auth, watch subcommands with their
│   │                          flags and visible aliases
│   ├── api.rs                 reqwest::blocking client for the edge worker.
│   │                          Wraps /api/dl, /api/runs, /api/runs/<id>,
│   │                          /api/ping with typed request / response shapes
│   ├── config.rs              %APPDATA%\rdl\config.json read/write. Tokens
│   │                          are wrapped with the Windows DPAPI before they
│   │                          touch disk; fallback path for non-Windows
│   │                          targets exists for CI builds
│   └── error.rs               Crate-wide thiserror enum
├── tests/
│   └── integration.rs         assert_cmd-based CLI tests: --version,
│                              subcommand help, config get/set roundtrip,
│                              clean failure mode when no config is present
└── .github/workflows/
    ├── build.yml              CI: cargo build --release for the rdl binary;
    │                          uploads x64 artifact; cuts a Release on tag push
    ├── test.yml               CI: cargo fmt --check, cargo clippy -D warnings,
    │                          cargo test --all-targets
    ├── download.yml           Backend automation. Streams a URL to OneDrive
    │                          via curl | rclone rcat. Inputs are masked.
    │                          Refreshed OAuth token is written back to the
    │                          repo secret so subsequent runs survive
    │                          long-term inactivity
    └── download-hls.yml       Backend automation variant for HLS / streaming
                               sources, using yt-dlp piped into rclone rcat
```

### Compatibility

| Axis | Supported |
|---|---|
| Operating system (client binary) | Windows 10 1809+ and Windows 11 |
| Architecture | x86_64 only |
| Rust toolchain | 1.75+ (`rust-toolchain.toml` pins stable) |
| Linker | MSVC (Visual Studio Build Tools 2022) |
| Cloud storage | Microsoft OneDrive (Personal or Business) via rclone |
| Telegram client | any version that supports webhook bots |

The shipped client binary is statically linked against the MSVC runtime, so
it has no DLL dependencies beyond what Windows itself ships
(`KERNEL32.DLL`, `USER32.DLL`, `ADVAPI32.DLL`, `CRYPT32.DLL`, `WS2_32.DLL`).

### Security considerations

- **Token at rest** — the bearer token in `%APPDATA%\rdl\config.json` is
  DPAPI-wrapped (`CryptProtectData`). Only the originating Windows user
  account can decrypt it.
- **Token in transit** — `reqwest` is built with `https_only=true`; tokens
  are sent in the `Authorization` header over TLS, never in the URL.
- **Auth surface** — only one Telegram user (`TG_OWNER_ID`) is allowed to
  command the bot; webhook calls require a secret in the URL path that is
  shared only with Telegram.
- **Workflow inputs** — every URL and filename passed to the GitHub Actions
  workflows is masked with `::add-mask::` so it does not appear in run logs.
- **OAuth refresh** — the OneDrive refresh token is rotated and written back
  to the `RCLONE_CONF` repo secret on every successful run, so the system
  survives long periods of inactivity without manual intervention.
- **No telemetry** — neither the CLI nor the backend report any usage data
  to third parties.

### Troubleshooting

| Symptom | Likely cause | Resolution |
|---|---|---|
| `worker URL not set` | first-time setup on this machine | `rdl config set worker https://<your-worker>.workers.dev` |
| `not authenticated — run rdl auth login` | no token cached | Issue a permanent token via the Telegram bot, then `rdl auth login --token <token>` |
| `server rejected credentials (401)` | token revoked or expired | Issue a new token from the bot |
| `not found` from `rdl status <id>` | wrong job id, or run was deleted | `rdl list` to find the current job ids |
| Workflow run fails on the OneDrive step | OneDrive token went stale (>90 days inactive) | `rclone config reconnect onedrive:` on a desktop, then update the `RCLONE_CONF` repo secret |
| Telegram bot stops replying | webhook secret mismatch or worker code error | `https://api.telegram.org/bot<TOKEN>/getWebhookInfo` to inspect; check Cloudflare Worker logs |

### Contributing

This is a personal-use tool. PRs that improve the CLI ergonomics, harden
the auth surface, or extend the workflows to additional storage backends
are welcome.

Before opening a PR:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

The CI runs the same three checks; PRs that break any of them will be
flagged.

### Acknowledgements

- [`clap`](https://crates.io/crates/clap) — derive-based CLI argument parsing.
- [`reqwest`](https://crates.io/crates/reqwest) +
  [`rustls`](https://crates.io/crates/rustls) — TLS-only HTTP client.
- [`windows`](https://crates.io/crates/windows) — official Microsoft Win32
  bindings, used here for the DPAPI Crypt{Protect,Unprotect}Data calls.
- [`directories`](https://crates.io/crates/directories) — cross-platform
  config path resolution.
- [`rclone`](https://rclone.org) — does the actual chunked OneDrive upload
  inside the GitHub Actions workflow.

### Installation

#### Pre-built binary (Windows x64)

1. Download `rdl-x64.exe` from the [Releases](https://github.com/GHSFS/remote-dl/releases) page.
2. Place it on your `PATH` (e.g. `C:\Tools\`).
3. Verify: `rdl --version`.

#### Build from source

Requires Rust 1.75+ and the MSVC toolchain.

```bash
git clone https://github.com/GHSFS/remote-dl.git
cd remote-dl
cargo build --release
# Output: target/release/rdl.exe
```

### Quick start

```bash
# 1. Point the client at your backend
rdl config set worker https://your-worker.example.workers.dev

# 2. Authenticate (one-time; obtain a permanent token from the bot first)
rdl auth login --token <permanent-token>

# 3. Queue a download
rdl https://example.com/dataset.tar.zst

# 4. Track progress
rdl list
rdl status <job-id>
```

### CLI reference

```
USAGE:
    rdl <SUBCOMMAND>

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
```

### Configuration

Config lives at `%APPDATA%\rdl\config.json`. Tokens are encrypted with the
Windows DPAPI so they cannot be read outside the originating user account.

| Key       | Description                              |
|-----------|------------------------------------------|
| `worker`  | Base URL of the deployed edge worker     |
| `token`   | Permanent auth token (DPAPI-encrypted)   |
| `folder`  | Default destination folder               |

### Backend setup

The automation backend runs via [`.github/workflows/download.yml`](./.github/workflows/download.yml) (standard HTTP streams) and [`.github/workflows/download-hls.yml`](./.github/workflows/download-hls.yml) (HLS / `yt-dlp` streams):

1. Configure `rclone` locally for your OneDrive account (`rclone config`) and store the resulting `rclone.conf` contents as the `RCLONE_CONF` repository secret.
2. Create a GitHub Personal Access Token with `repo` + `workflow` scopes and store it as `GH_PAT` (used by the workflow's final step to persist rotated OAuth refresh tokens back to `RCLONE_CONF`).
3. Deploy your Cloudflare edge worker pointing `workflow_dispatch` events at this repository, then configure `rdl config set worker <url>`.

### FAQ

**Q. Why not just use `wget` / `curl` / `aria2c` locally?**
A. Because the data path goes through your home network and disk, which is the
exact bottleneck this project removes. `remote-dl` makes a 100 GB transfer cost
zero local disk and zero local bandwidth.

**Q. Can multiple users share an instance?**
A. No, this is intentionally a single-user tool. Multi-tenancy adds significant
complexity to auth, quotas, and abuse handling that is out of scope.

**Q. Are downloads resumable?**
A. Yes — uploads to OneDrive are chunked and auto-resume on the upload side.
The download side is one-shot per job; if it fails, re-queue.

### License

MIT. See [LICENSE](./LICENSE).

### Disclaimer

This is a personal tool intended for archiving content that you own or that
you are authorized to download. The operator is solely responsible for
complying with the terms of service of any source website and with applicable
copyright law. **Do not use this tool to redistribute, mirror, proxy, or share
content with third parties.**
