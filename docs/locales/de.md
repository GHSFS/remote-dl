<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <strong>🇩🇪 Deutsch</strong> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# remote-dl

> Direkt-Streaming-Download-Manager für die Cloud — leitet URLs direkt in Ihren persönlichen Cloud-Speicher weiter, ohne Daten auf der lokalen Festplatte zwischenzuspeichern oder Ihre Bandbreite zu belasten.

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#-installation-und-bereitstellung)

</div>

---

## 🛡️ Projektübersicht und Kernkompetenzen

<details open>
<summary><b>🛡️ Projektübersicht und Kernkompetenzen</b></summary>
<br>

`remote-dl` ist ein persönlicher Download-Manager für Operatoren, die große Dateien (hochauflösende Videos, umfangreiche Archive, Datensätze, Forschungskorpora) **direkt in ihren eigenen Cloud-Speicher übertragen** möchten, ohne diese zuvor über einen lokalen Computer puffern zu müssen.

Ein schlanker, nativer CLI-Client (`rdl.exe`) auf Ihrem Desktop koordiniert die Vorgänge mit einem leichtgewichtigen serverlosen Backend. Der tatsächliche Datentransfer erfolgt vollständig innerhalb der Cloud-Infrastruktur, sodass weder Ihre lokale Festplatte noch Ihr Heimnetzwerk im Datenübertragungsweg beansprucht werden.

Dieses Werkzeug ist für den **selbstgehosteten Einzelbenutzerbetrieb** konzipiert. Sie binden Ihr eigenes Cloud-Speicherkonto ein; es werden keinerlei Daten oder Ressourcen mit Dritten geteilt.

</details>

---

## ⚡ Kernfunktionen

<details open>
<summary><b>⚡ Kernfunktionen</b></summary>
<br>

- **Speichereffizientes Direkt-Streaming** — Dateien beliebiger Größe fließen durch eine kontinuierliche Streaming-Pipeline; weder auf dem Client noch im Backend findet ein Zwischenspeichern auf der Festplatte statt.
- **Native OneDrive-Anbindung** — Segmentierter Upload (Chunks) mit automatischer Wiederaufnahme über die Microsoft Graph API.
- **Einfache und robuste Einzelbenutzer-Authentifizierung** — Einmalpasswörter (OTP) kombiniert mit widerrufbaren Dauertokens; kein komplexes Kontosystem, keine Abhängigkeit von externen Identitätsanbietern.
- **Mehrere einheitliche Schnittstellen** — Desktop-CLI, mobiloptimierte Weboberfläche und ein privater Telegram-Bot greifen nahtlos auf dasselbe Backend zu.
- **Strikte Isolierung der Vertrauensgrenzen (Trust Boundaries)** — Zugangsdaten verbleiben ausnahmslos in der zuständigen Schicht (Operator → Edge-Worker → Automatisierungs-Runner → Speicheranbieter).
- **Automatische Token-Rotation** — OAuth-Aktualisierungstokens werden nach jedem erfolgreichen Transfer erneuert und verschlüsselt persistiert, sodass das System auch nach monatelanger Inaktivität zuverlässig betriebsbereit bleibt.

</details>

---

## 🏛️ Systemarchitektur und Topologie

<details open>
<summary><b>🏛️ Systemarchitektur und Topologie</b></summary>
<br>

```text
  ┌──────────────────────┐
  │  rdl.exe CLI-Client  │
  │  Web-Oberfläche      │ ──HTTPS──┐
  │  Telegram-Bot        │          │
  └──────────────────────┘          ▼
                               ┌──────────────────────┐
                               │  Edge-Worker         │
                               │  (Auth + Steuerung)  │
                               └──────────┬───────────┘
                                          │ workflow_dispatch
                                          ▼
                               ┌──────────────────────┐
                               │  Cloud-Runner        │
                               │  (Streaming-Relay)   │
                               └──────────┬───────────┘
                                          │ Chunks-Upload
                                          ▼
                               ┌──────────────────────┐
                               │  Persönliches        │
                               │  OneDrive            │
                               └──────────────────────┘
```

</details>

---

## 📂 Verzeichnisstruktur des Repositories

<details open>
<summary><b>📂 Verzeichnisstruktur des Repositories</b></summary>
<br>

```text
remote-dl/
├── Cargo.toml                 Paketmanifest: Definiert die rdl-Binärdatei, Abhängigkeiten und größenoptimiertes Release-Profil
├── Cargo.lock                 Exakt festgeschriebene Versionen aller transitiven Abhängigkeiten
├── rust-toolchain.toml        Festlegung der Compiler-Version (stable, x86_64-pc-windows-msvc)
├── .cargo/
│   └── config.toml            Build-Ziel und rustflags-Optimierungsparameter (statische MSVC-CRT-Verlinkung)
├── .gitignore                 Schließt target/, Geheimnisse, IDE-Konfigurationen und Entwürfe aus
├── README.md                  Englische Hauptdokumentation
├── LICENSE                    MIT-Lizenz
├── CHANGELOG.md               Änderungsprotokoll im Keep-a-Changelog-Format
├── docs/
│   └── locales/               Lokalisierte Dokumentation (ko, zh-CN, ja, es, ru, vi, tr, de, pt-BR)
├── src/
│   ├── main.rs                Haupteinstiegspunkt: Verarbeitet clap-Argumente, delegiert Befehle, farbige Ausgabe
│   ├── cli.rs                 clap-Befehlsstruktur: get, list, status, config, auth, watch
│   ├── api.rs                 REST-API-Client auf Basis von reqwest::blocking für den Edge-Worker
│   ├── config.rs              Lesen/Schreiben von %APPDATA%\rdl\config.json mit Windows-DPAPI-Hardwareverschlüsselung
│   └── error.rs               Zentrale Fehlerbehandlung auf Basis des thiserror-Crates
├── tests/
│   └── integration.rs         CLI-Integrationstests mit assert_cmd (--version, Konfigurationspersistenz, Fehlerzustände)
└── .github/workflows/
    ├── build.yml              CI: Führt cargo build --release aus und erzeugt das rdl-Release-Artefakt für x64
    ├── test.yml               CI: Führt cargo fmt, cargo clippy und cargo test --all-targets aus
    ├── download.yml           Cloud-Automatisierungs-Workflow: Direkte Pipeline von URL zu OneDrive (curl | rclone rcat)
    └── download-hls.yml       Automatisierungsvariante für HLS-Videostreams (yt-dlp | rclone rcat)
```

</details>

---

## 💻 Kompatibilität und Systemanforderungen

<details open>
<summary><b>💻 Kompatibilität und Systemanforderungen</b></summary>
<br>

| Merkmal | Unterstützter Bereich |
|---|---|
| Betriebssystem (Client-Binärdatei) | Windows 10 Version 1809+ sowie Windows 11 |
| Prozessorarchitektur | Ausschließlich x86_64 |
| Rust-Toolchain | 1.75+ (über `rust-toolchain.toml` auf stabile Version festgelegt) |
| Linker | MSVC (Visual Studio Build Tools 2022) |
| Cloud-Speicher | Microsoft OneDrive (Persönlich oder Geschäftlich) über rclone |
| Telegram-Client | Jeder Client mit Unterstützung für Webhook-Bots |

Die ausgelieferte Client-Binärdatei ist statisch mit der MSVC-Laufzeitumgebung verknüpft und besitzt daher abgesehen von den standardmäßigen Windows-System-DLLs (`KERNEL32.DLL`, `USER32.DLL`, `ADVAPI32.DLL`, `CRYPT32.DLL`, `WS2_32.DLL`) keinerlei externe Abhängigkeiten.

</details>

---

## 🔒 Sicherheitsaspekte und Design-Invarianten

<details open>
<summary><b>🔒 Sicherheitsaspekte und Design-Invarianten</b></summary>
<br>

- **Verschlüsselung im Ruhezustand (Token at rest)** — Das in `%APPDATA%\rdl\config.json` gespeicherte Authentifizierungstoken wird über die Windows DPAPI (`CryptProtectData`) verschlüsselt. Ausschließlich das Benutzerkonto, das den Schlüssel angelegt hat, kann ihn wieder entschlüsseln.
- **Schutz während der Übertragung (Token in transit)** — Der `reqwest`-Client erzwingt `https_only=true`; Authentifizierungstokens werden grundsätzlich über TLS im `Authorization`-Header übertragen und verbleiben niemals in der URL.
- **Minimierte Angriffsoberfläche** — Ausschließlich die hinterlegte Telegram-Benutzer-ID (`TG_OWNER_ID`) ist berechtigt, Steuerbefehle zu senden; Webhook-Aufrufe erfordern ein Geheimnis im URL-Pfad, das ausschließlich mit Telegram geteilt wird.
- **Maskierung sensibler Daten in Protokollen** — Alle an GitHub Actions übergebenen URLs und Dateinamen werden mit `::add-mask::` maskiert, um eine Offenlegung in öffentlichen Protokollen zu verhindern.
- **Selbstheilende OAuth-Token-Rotation** — Bei jedem erfolgreichen Durchlauf wird das OneDrive-Aktualisierungstoken erneuert und in das Repository-Geheimnis `RCLONE_CONF` zurückgeschrieben. Dadurch verfällt der Zugriff selbst bei monatelanger Nichtbenutzung nicht.
- **Keine Telemetrie** — Weder der CLI-Client noch das Backend senden Nutzungsstatistiken oder Diagnosedaten an Server Dritter.

</details>

---

## 🛠️ Matrix zur Fehlerbehebung

<details open>
<summary><b>🛠️ Matrix zur Fehlerbehebung</b></summary>
<br>

| Symptom | Wahrscheinliche Ursache | Empfohlene Lösung |
|---|---|---|
| `worker URL not set` | Erstmalige Ausführung auf diesem System ohne Initialisierung | Führen Sie `rdl config set worker https://<ihr-worker>.workers.dev` aus |
| `not authenticated — run rdl auth login` | Kein Token im lokalen Cache vorhanden | Erzeugen Sie ein Dauertoken im Telegram-Bot und führen Sie `rdl auth login --token <token>` aus |
| `server rejected credentials (401)` | Token wurde widerrufen oder ist abgelaufen | Fordern Sie über den Telegram-Bot ein neues Token an und melden Sie sich erneut an |
| `rdl status <id>` meldet `not found` | Ungültige Job-ID oder die Ausführungsprotokolle wurden gelöscht | Führen Sie `rdl list` aus, um die Liste der aktiven Job-IDs einzusehen |
| Workflow scheitert beim OneDrive-Schritt | OneDrive-Token ist wegen Inaktivität (>90 Tage) abgelaufen | Führen Sie lokal `rclone config reconnect onedrive:` aus und aktualisieren Sie das `RCLONE_CONF`-Geheimnis |
| Telegram-Bot reagiert nicht mehr | Webhook-Geheimnis stimmt nicht überein oder Worker-Fehler | Prüfen Sie den Status über `https://api.telegram.org/bot<TOKEN>/getWebhookInfo` und prüfen Sie die Worker-Logs |

</details>

---

## 📦 Installation und Bereitstellung

<details open>
<summary><b>📦 Installation und Bereitstellung</b></summary>
<br>

### Vorkompilierte Binärdatei (Windows x64)

1. Laden Sie die Binärdatei `rdl-x64.exe` von der [offiziellen Release-Seite](https://github.com/GHSFS/remote-dl/releases) herunter.
2. Legen Sie die Datei in einem Verzeichnis ab, das in Ihrer Systemumgebungsvariable `PATH` enthalten ist (z. B. `C:\Tools\`).
3. Überprüfen Sie die Installation im Terminal mit: `rdl --version`.

### Aus dem Quellcode kompilieren

Erfordert Rust 1.75+ sowie die Microsoft C++-Buildtools (MSVC).

```bash
git clone https://github.com/GHSFS/remote-dl.git
cd remote-dl
cargo build --release
# Ausgabedatei: target/release/rdl.exe
```

</details>

---

## 🚀 Schnelleinstieg und CLI-Befehlsreferenz

<details open>
<summary><b>🚀 Schnelleinstieg und CLI-Befehlsreferenz</b></summary>
<br>

```bash
# 1. Client auf Ihr Backend konfigurieren
rdl config set worker https://your-worker.example.workers.dev

# 2. Authentifizierung durchführen (nur einmal erforderlich; Token zuvor über den Bot erstellen)
rdl auth login --token <permanent-token>

# 3. Download in die Cloud-Warteschlange einreihen
rdl https://example.com/dataset.tar.zst

# 4. Übertragungsstatus überwachen
rdl list
rdl status <job-id>
```

### Vollständige Spezifikation der CLI-Befehle

```text
USAGE:
    rdl [OPTIONS] <SUBCOMMAND>

SUBCOMMANDS:
    get, g <url>                Reiht eine URL in die Download-Warteschlange ein (--name, --folder, --json)
    list, ls                    Zeigt kürzlich übertragene Downloads und deren Fortschritt an (--limit, --json)
    status, st [job-id]         Gibt den Echtzeit-Status eines bestimmten Auftrags aus (--json)
    inspect <url>               Vorabprüfung: HTTP-Header, Dateigröße und Range-Unterstützung untersuchen (--json)
    batch <file>                Mehrere URLs aus einer Textdatei im Stapel einreihen (--folder, --json)
    ping                        Überprüft die Verbindung zum Edge-Worker und die Gültigkeit des Tokens (--json)
    config <get|set|path|show>  Konfigurationsdatei lesen, schreiben, Pfad ermitteln oder diagnostizieren
    auth <login|logout>         Verwaltung des über Windows DPAPI geschützten Authentifizierungstokens
    watch                       Startet im Zwischenablage-Überwachungsmodus (Begleitmodus für rdl-tray)
    help                        Hilfetext anzeigen

OPTIONS:
    -j, --json                  Ausgabe im maschinenlesbaren JSON-Format
    -h, --help                  Hilfe anzeigen
    -V, --version               Versionsinformationen ausgeben
```

</details>

---

## ⚙️ Detaillierte Konfiguration und Backend-Setup

<details open>
<summary><b>⚙️ Detaillierte Konfiguration und Backend-Setup</b></summary>
<br>

### Client-Konfiguration

Die Konfigurationsdatei wird standardmäßig unter `%APPDATA%\rdl\config.json` gespeichert. Vertrauliche Daten werden über die Windows DPAPI verschlüsselt, um unberechtigten Zugriff durch andere Benutzerkonten oder Fremdprozesse zu unterbinden.

| Schlüssel | Beschreibung |
|---|---|
| `worker` | Basis-URL des bereitgestellten Cloudflare Edge-Workers |
| `token` | Dauerhaftes Authentifizierungstoken (DPAPI-verschlüsselt) |
| `folder` | Standardmäßiger Zielordnername in OneDrive |

### Einrichtung des Automatisierungs-Backends

Das Cloud-Backend basiert auf GitHub Actions-Workflows (siehe [`download.yml`](./.github/workflows/download.yml) und [`download-hls.yml`](./.github/workflows/download-hls.yml)):

1. Konfigurieren Sie `rclone` lokal für OneDrive (`rclone config`) und hinterlegen Sie den vollständigen Inhalt der generierten Datei `rclone.conf` als Repository-Geheimnis mit dem Namen `RCLONE_CONF`.
2. Erstellen Sie ein GitHub Personal Access Token (PAT) mit den Berechtigungen `repo` und `workflow` und speichern Sie es als Geheimnis `GH_PAT` im Repository (wird zur automatischen Speicherung erneuerter Tokens verwendet).
3. Veröffentlichen Sie Ihren Cloudflare Edge-Worker, der `workflow_dispatch`-Ereignisse im Repository auslöst, und verbinden Sie den lokalen Client mit `rdl config set worker <Worker-URL>`.

</details>

---

## ❓ Häufig gestellte Fragen (FAQ)

<details open>
<summary><b>❓ Häufig gestellte Fragen (FAQ)</b></summary>
<br>

**F: Warum nicht einfach `wget`, `curl` oder `aria2c` lokal auf dem Computer verwenden?**
A: Bei lokalen Werkzeugen müssen Dateien mit oft Dutzenden oder Hunderten Gigabyte durch Ihre private Internetverbindung und über Ihre lokale Festplatte geleitet werden. `remote-dl` ermöglicht die Übertragung einer 100-GB-Datei mit exakt 0 Byte lokalem Festplattenverbrauch und 0 Byte heimischem Datenverkehr.

**F: Können mehrere Benutzer dieselbe Instanz gemeinsam nutzen?**
A: Nein, das Werkzeug ist bewusst als reines Einzelbenutzer-System ausgelegt. Mehrbenutzerunterstützung würde erhebliche Komplexität bei Benutzerisolierung, Quotenverwaltung und Missbrauchsprävention verursachen, was dem Ziel maximaler Einfachheit widerspricht.

**F: Werden unterbrochene Downloads automatisch fortgesetzt?**
A: Ja. Der Upload-Vorgang zu OneDrive erfolgt in Streaming-Chunks und unterstützt die automatische Wiederaufnahme vollständig. Der Download von der Quell-URL wird innerhalb eines Jobs als Direktübertragung ausgeführt; schlägt die Verbindung fehl, reihen Sie die URL einfach erneut ein.

</details>

---

## 🤝 Mitwirkung und Danksagung

<details open>
<summary><b>🤝 Mitwirkung und Danksagung</b></summary>
<br>

Vor dem Einreichen eines Pull Requests (PR) müssen folgende drei Qualitätsprüfungen erfolgreich bestanden werden:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

### Danksagung an zentrale Programmbibliotheken

- [`clap`](https://crates.io/crates/clap) — Robuste und extrem schnelle CLI-Argumentanalyse über Derive-Makros
- [`reqwest`](https://crates.io/crates/reqwest) + [`rustls`](https://crates.io/crates/rustls) — Moderner HTTP-Client mit konsequenter TLS-Absicherung
- [`windows`](https://crates.io/crates/windows) — Offizielle Microsoft Win32-Bindungen für DPAPI
- [`directories`](https://crates.io/crates/directories) — Plattformkonforme Auflösung von Konfigurationspfaden
- [`rclone`](https://rclone.org) — Bewährte Streaming-Upload-Engine für segmentierte OneDrive-Uploads in der Cloud

</details>

---

## 📄 Lizenz und Haftungsausschluss

<details open>
<summary><b>📄 Lizenz und Haftungsausschluss</b></summary>
<br>

Dieses Projekt ist unter den Bedingungen der **MIT-Lizenz** lizenziert (siehe [LICENSE](../../LICENSE)).

### Haftungsausschluss (Disclaimer)

Dieses Werkzeug dient ausschließlich dem Zweck der privaten Archivierung von Inhalten, für die Sie die Eigentumsrechte besitzen oder zu deren Download Sie ausdrücklich berechtigt sind. Der Anwender ist allein dafür verantwortlich, die Nutzungsbedingungen der jeweiligen Quell-Websites sowie das geltende Urheberrecht einzuhalten. **Jede unbefugte Weiterverbreitung, öffentliche Spiegelung, Proxy-Nutzung oder Weitergabe an Dritte unter Verwendung dieses Werkzeugs ist strikt untersagt.**

</details>
