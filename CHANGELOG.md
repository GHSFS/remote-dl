# Changelog

All notable changes to `rdl` (the desktop client) are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.2.0] - 2026-10-13

### Added
- **Pre-flight URL Inspection (`rdl inspect <url>`)**: Fast remote HTTP HEAD/Range check to extract `Content-Length`, `Content-Type`, `Accept-Ranges`, resumable streaming capability, and suggested filename.
- **Batch Enqueueing (`rdl batch <file>`)**: Sequential multi-URL queueing from a text file with comments and blank-line support.
- **Worker Connectivity Ping (`rdl ping`)**: Instant latency and authentication validation against the deployed Edge Worker.
- **Unified JSON Mode (`--json`)**: Machine-readable JSON output across `get`, `list`, `status`, `inspect`, `batch`, and `config show`.
- **Config Summary Command (`rdl config show`)**: Visual overview of active configuration with masked DPAPI token status.
- **Integration Test Suite**: 10 tests passing covering inspect, batch, configuration, and version verification.

## [0.1.0] - 2026-10-12

### Added
- Initial CLI with `get`, `list`, `status`, `config`, `auth` subcommands.
- DPAPI-protected token storage at `%APPDATA%\rdl\config.json`.
- Reusable HTTP client (`src/api.rs`) for the edge worker.
- GitHub Actions CI: format check, clippy, integration tests.
