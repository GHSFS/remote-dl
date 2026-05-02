# Changelog

All notable changes to `rdl` (the desktop client) are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- Initial CLI with `get`, `list`, `status`, `config`, `auth` subcommands.
- DPAPI-protected token storage at `%APPDATA%\rdl\config.json`.
- Reusable HTTP client (`src/api.rs`) for the edge worker.
- GitHub Actions CI: format check, clippy, integration tests, signed release builds.

### Planned
- `rdl-tray.exe` companion (clipboard watcher + toast notifications).
- Optional progress streaming via Server-Sent Events from the worker.
- macOS / Linux build targets (currently Windows-only).
