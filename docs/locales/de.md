<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <strong>🇩🇪 Deutsch</strong> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# remote-dl

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

### Überblick

`remote-dl` ist ein persönlicher Download-Manager für Nutzer, die große
Dateien (Videos, Archive, Datensätze, Forschungskorpora) **direkt in ihren
eigenen Cloud-Speicher** abrufen möchten, ohne sie zuerst über einen lokalen
Rechner zu puffern.

Ein kleiner nativer CLI-Client (`rdl.exe`) auf deinem Desktop kommuniziert
mit einem leichtgewichtigen serverlosen Backend. Die eigentliche
Byte-Übertragung erfolgt vollständig in der Cloud, sodass weder deine
Festplatte noch deine Heimverbindung im Datenpfad liegen.

Dies ist ein Single-User-Self-Hosted-Tool. Du bringst deinen eigenen
Cloud-Speicher-Account mit; es werden keine Ressourcen mit anderen Nutzern
geteilt.

### Funktionen

- **Speichereffizientes Streaming** — Dateien jeder Größe fließen durch eine
  Streaming-Pipeline; weder im Client noch im Backend wird etwas auf die
  Festplatte ausgelagert
- **Native OneDrive-Unterstützung** — Chunked, wiederaufnehmbare Uploads über
  die Microsoft Graph API
- **Single-User-Authentifizierung** — Einmalpasswörter und widerrufbare
  permanente Tokens; kein Kontosystem, keine Abhängigkeit von Drittanbieter-IdPs
- **Mehrere Frontends** — Desktop-CLI, mobilfreundliche Web-UI und privater
  Telegram-Bot teilen sich dasselbe Backend
- **Trust-Boundary-Isolation** — Anmeldedaten verlassen nie die Ebene, der
  sie gehören (Betreiber → Edge Worker → Automation Runner → Storage-Provider)
- **Selbst rotierende Tokens** — OAuth-Refresh-Tokens werden bei jeder
  erfolgreichen Übertragung erneuert und in den verschlüsselten Speicher
  zurückgeschrieben

### Projektstruktur

```
remote-dl/
├── Cargo.toml             Paketmanifest, größenoptimiertes Release-Profil
├── rust-toolchain.toml    Toolchain-Pin (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml     Build-Ziel + statische MSVC-CRT
├── README.md / LICENSE / CHANGELOG.md
├── src/
│   ├── main.rs            CLI-Einstiegspunkt, clap-Dispatcher, farbige Ausgabe
│   ├── cli.rs             clap-Definitionen: get / list / status / config / auth / watch
│   ├── api.rs             reqwest::blocking-Client (Wrapper für Worker-API)
│   ├── config.rs          config.json lesen/schreiben + DPAPI-Tokenschutz
│   └── error.rs           Crate-weiter Fehlertyp
├── tests/integration.rs   CLI-Tests basierend auf assert_cmd
└── .github/workflows/
    ├── build.yml          rdl.exe Release-Build-CI
    ├── test.yml           fmt / clippy / test CI
    ├── download.yml       Backend-Automatisierung: URL → OneDrive-Streaming
    └── download-hls.yml   HLS / Streaming-Variante (yt-dlp + rclone rcat)
```

Ausführliche Installations-, Schnellstart-, CLI-Referenz- und FAQ-Anleitungen
findest du im Abschnitt [English](#english).

### Lizenz

MIT. Siehe [LICENSE](./LICENSE).
