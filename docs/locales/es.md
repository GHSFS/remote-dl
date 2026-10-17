<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <strong>🇪🇸 Español</strong><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# remote-dl

> Administrador de descargas en flujo directo a la nube — canalice URLs directamente a su almacenamiento personal sin intermediar en el disco local ni agotar su ancho de banda.

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#-instalación-y-despliegue)

</div>

---

## 🛡️ Descripción general y capacidades clave

<details open>
<summary><b>🛡️ Descripción general y capacidades clave</b></summary>
<br>

`remote-dl` es un gestor de descargas personal diseñado para operadores que necesitan transferir archivos pesados (vídeos de alta resolución, archivos comprimidos, conjuntos de datos o corpus de investigación) **directamente a su propio almacenamiento en la nube**, sin almacenarlos primero en el disco de su máquina local.

Un cliente nativo de línea de comandos (`rdl.exe`) ultraligero en su escritorio coordina las operaciones con un backend sin servidor (serverless). La transferencia de bytes real ocurre íntegramente dentro de la infraestructura en la nube, por lo que ni el disco local ni la conexión de banda ancha de su hogar intervienen en la ruta de los datos.

Esta herramienta está diseñada para ser **autohospedada por un único usuario**. Usted aporta su propia cuenta de almacenamiento en la nube; no se comparte ningún dato ni recurso con terceros.

</details>

---

## ⚡ Características principales

<details open>
<summary><b>⚡ Características principales</b></summary>
<br>

- **Transmisión de flujo con memoria ultraligera** — archivos de cualquier tamaño fluyen a través de una tubería de streaming continuo; no se realiza almacenamiento intermedio en disco ni en el cliente ni en el backend.
- **Integración nativa con OneDrive** — transferencias por fragmentos (chunks) compatibles con reanudación automática mediante la API de Microsoft Graph.
- **Autenticación sólida para un único usuario** — contraseñas de un solo uso (OTP) junto con tokens persistentes revocables; sin sistemas complejos de cuentas ni proveedores de identidad externos.
- **Múltiples interfaces unificadas** — interfaz CLI de escritorio, interfaz web ligera para móviles y un bot privado de Telegram comparten el mismo backend.
- **Aislamiento riguroso del límite de confianza** — las credenciales nunca abandonan la capa a la que pertenecen (operador → Edge Worker → ejecutor de automatización → proveedor de almacenamiento).
- **Autorrotación continua de credenciales** — los tokens de actualización OAuth se renuevan y se vuelven a guardar cifrados tras cada transferencia exitosa, garantizando funcionamiento continuo tras largos periodos de inactividad.

</details>

---

## 🏛️ Arquitectura y topología del sistema

<details open>
<summary><b>🏛️ Arquitectura y topología del sistema</b></summary>
<br>

```text
  ┌──────────────────────┐
  │  Cliente CLI rdl.exe │
  │  Interfaz Web        │ ──HTTPS──┐
  │  Bot de Telegram     │          │
  └──────────────────────┘          ▼
                               ┌──────────────────────┐
                               │  Worker de borde     │
                               │  (Auth + Despacho)   │
                               └──────────┬───────────┘
                                          │ workflow_dispatch
                                          ▼
                               ┌──────────────────────┐
                               │  Ejecutor en la nube │
                               │  (Proxy en flujo)    │
                               └──────────┬───────────┘
                                          │ Subida por fragmentos
                                          ▼
                               ┌──────────────────────┐
                               │  OneDrive personal   │
                               └──────────────────────┘
```

</details>

---

## 📂 Estructura del repositorio

<details open>
<summary><b>📂 Estructura del repositorio</b></summary>
<br>

```text
remote-dl/
├── Cargo.toml                 Manifiesto del paquete: define el binario rdl, dependencias y perfil de lanzamiento optimizado en tamaño
├── Cargo.lock                 Versiones exactas y fijadas de todas las dependencias transitivas
├── rust-toolchain.toml        Fijación de versión del compilador (stable, x86_64-pc-windows-msvc)
├── .cargo/
│   └── config.toml            Objetivo de compilación y parámetros rustflags (enlace estático a MSVC CRT)
├── .gitignore                 Excluye target/, claves secretas, estado del IDE y borradores
├── README.md                  Documento principal en inglés
├── LICENSE                    Licencia de código abierto MIT
├── CHANGELOG.md               Registro de cambios en formato Keep-a-Changelog
├── docs/
│   └── locales/               Documentación traducida y localizada (ko, zh-CN, ja, es, ru, vi, tr, de, pt-BR)
├── src/
│   ├── main.rs                Punto de entrada de la CLI: procesa argumentos con clap, despacha comandos y da formato en color
│   ├── cli.rs                 Definiciones de comandos clap: get, list, status, config, auth, watch
│   ├── api.rs                 Cliente REST API basado en reqwest::blocking para el Worker de borde
│   ├── config.rs              Lectura y escritura en %APPDATA%\rdl\config.json con cifrado hardware Windows DPAPI
│   └── error.rs               Enumeración y gestión centralizada de errores con thiserror
├── tests/
│   └── integration.rs         Pruebas de integración de la CLI con assert_cmd (--version, configuración, manejo de errores)
└── .github/workflows/
    ├── build.yml              CI: compilación cargo build --release y generación de binarios para x64
    ├── test.yml               CI: comprobación exhaustiva con cargo fmt, cargo clippy y cargo test --all-targets
    ├── download.yml           Flujo automatizado en la nube: tubería de URL a OneDrive (curl | rclone rcat)
    └── download-hls.yml       Variante de automatización para flujos de vídeo HLS (yt-dlp | rclone rcat)
```

</details>

---

## 💻 Compatibilidad y plataformas soportadas

<details open>
<summary><b>💻 Compatibilidad y plataformas soportadas</b></summary>
<br>

| Aspecto | Nivel de soporte |
|---|---|
| Sistema operativo (binario cliente) | Windows 10 1809+ y Windows 11 |
| Arquitectura del procesador | Únicamente x86_64 |
| Cadena de herramientas Rust | 1.75+ (bloqueado en estable por `rust-toolchain.toml`) |
| Enlazador (Linker) | MSVC (Visual Studio Build Tools 2022) |
| Almacenamiento en la nube | Microsoft OneDrive (Personal o Corporativo) vía rclone |
| Cliente de Telegram | Cualquier cliente que admita bots mediante Webhooks |

El binario cliente distribuido está enlazado estáticamente contra el tiempo de ejecución de MSVC, por lo que carece de dependencias de DLL externas más allá de las bibliotecas estándar del sistema operativo Windows (`KERNEL32.DLL`, `USER32.DLL`, `ADVAPI32.DLL`, `CRYPT32.DLL`, `WS2_32.DLL`).

</details>

---

## 🔒 Consideraciones de seguridad y principios de diseño

<details open>
<summary><b>🔒 Consideraciones de seguridad y principios de diseño</b></summary>
<br>

- **Cifrado de tokens en reposo (Token at rest)** — El token de portador guardado en `%APPDATA%\rdl\config.json` está cifrado mediante Windows DPAPI (`CryptProtectData`). Únicamente la cuenta de usuario de Windows que generó el secreto puede descifrarlo.
- **Protección del token en tránsito (Token in transit)** — El cliente `reqwest` se compila con `https_only=true`; los tokens viajan siempre protegidos por TLS en la cabecera `Authorization`, nunca expuestos en los parámetros URL.
- **Superficie de autenticación restringida** — Solo un usuario autorizado de Telegram (`TG_OWNER_ID`) tiene permiso para enviar órdenes al bot; la recepción de Webhooks requiere una clave secreta en la ruta URL compartida exclusivamente con Telegram.
- **Ocultamiento de datos sensibles en registros** — Cada URL y nombre de archivo transferido a los flujos de GitHub Actions se procesa con `::add-mask::` para evitar su aparición accidental en los registros públicos de compilación.
- **Autocuración y rotación de tokens OAuth** — En cada ejecución satisfactoria, el token de actualización de OneDrive se rota y se almacena nuevamente en el secreto del repositorio `RCLONE_CONF`, garantizando que la integración no caduque tras meses de inactividad.
- **Cero telemetría** — Ni la aplicación CLI ni los servicios backend envían métricas de uso ni registros analíticos a servidores externos de terceros.

</details>

---

## 🛠️ Matriz de resolución de problemas

<details open>
<summary><b>🛠️ Matriz de resolución de problemas</b></summary>
<br>

| Síntoma | Causa probable | Procedimiento de resolución |
|---|---|---|
| `worker URL not set` | Primera ejecución en este equipo sin inicialización previa | Ejecute `rdl config set worker https://<su-worker>.workers.dev` |
| `not authenticated — run rdl auth login` | No hay ningún token en la caché local | Genere un token permanente a través del bot de Telegram y ejecute `rdl auth login --token <token>` |
| `server rejected credentials (401)` | El token fue revocado manualmente o ha caducado | Emita un nuevo token desde el bot de Telegram e inicie sesión de nuevo |
| `rdl status <id>` responde con `not found` | Identificador de tarea erróneo o registro eliminado | Ejecute `rdl list` para comprobar los IDs de tareas activos |
| El flujo falla durante el paso de OneDrive | Token de OneDrive caducado por inactividad (>90 días) | Ejecute `rclone config reconnect onedrive:` en su PC y actualice el secreto de repositorio `RCLONE_CONF` |
| El bot de Telegram no responde a los comandos | Discrepancia en el secreto del Webhook o error en el Worker | Visite `https://api.telegram.org/bot<TOKEN>/getWebhookInfo` para verificar el estado y revise los logs del Worker |

</details>

---

## 📦 Instalación y despliegue

<details open>
<summary><b>📦 Instalación y despliegue</b></summary>
<br>

### Binario precompilado (Windows x64)

1. Descargue el archivo ejecutable `rdl-x64.exe` desde la [página de lanzamientos oficiales](https://github.com/GHSFS/remote-dl/releases).
2. Ubíquelo en una carpeta incluida en su variable de entorno `PATH` (por ejemplo, `C:\Tools\`).
3. Compruebe la correcta instalación ejecutando en su terminal: `rdl --version`.

### Compilación desde el código fuente

Requiere tener instalado Rust 1.75+ y el conjunto de herramientas de compilación MSVC.

```bash
git clone https://github.com/GHSFS/remote-dl.git
cd remote-dl
cargo build --release
# Binario resultante: target/release/rdl.exe
```

</details>

---

## 🚀 Inicio rápido y referencia de comandos CLI

<details open>
<summary><b>🚀 Inicio rápido y referencia de comandos CLI</b></summary>
<br>

```bash
# 1. Configure el cliente para conectarse a su backend
rdl config set worker https://your-worker.example.workers.dev

# 2. Autentíquese (únicamente la primera vez; obtenga el token desde el bot)
rdl auth login --token <permanent-token>

# 3. Ponga en cola una descarga remota
rdl https://example.com/dataset.tar.zst

# 4. Supervise el estado y progreso
rdl list
rdl status <job-id>
```

### Especificación exhaustiva de comandos CLI

```text
USAGE:
    rdl [OPTIONS] <SUBCOMMAND>

SUBCOMMANDS:
    get, g <url>                Pone en cola una URL para su descarga remota (--name, --folder, --json)
    list, ls                    Muestra las descargas recientes y su progreso (--limit, --json)
    status, st [job-id]         Muestra el estado detallado en tiempo real de una tarea (--json)
    inspect <url>               Inspección previa: cabeceras, tamaño del archivo y soporte de rangos (--json)
    batch <file>                Encola de forma masiva URLs listadas en un archivo de texto (--folder, --json)
    ping                        Comprueba la conectividad con el Edge Worker y la validez de credenciales (--json)
    config <get|set|path|show>  Lee, define, localiza o diagnostica la configuración del cliente
    auth <login|logout>         Gestiona el token permanente protegido por cifrado Windows DPAPI
    watch                       Ejecuta el cliente en modo monitorización del portapapeles (para rdl-tray)
    help                        Muestra este mensaje de ayuda

OPTIONS:
    -j, --json                  Produce la salida estructurada en formato JSON para lectura automatizada
    -h, --help                  Muestra la ayuda
    -V, --version               Muestra la información de versión
```

</details>

---

## ⚙️ Configuración y puesta en marcha del backend

<details open>
<summary><b>⚙️ Configuración y puesta en marcha del backend</b></summary>
<br>

### Configuración local del cliente

El archivo de configuración reside en `%APPDATA%\rdl\config.json`. Los tokens sensibles se almacenan cifrados con Windows DPAPI, impidiendo que otros usuarios o procesos los puedan leer.

| Clave | Descripción |
|---|---|
| `worker` | URL base del Worker de borde desplegado en Cloudflare |
| `token` | Token permanente de autenticación (cifrado por hardware con DPAPI) |
| `folder` | Nombre de la carpeta de destino por defecto en OneDrive |

### Despliegue del backend de automatización

El pipeline de automatización en la nube se ejecuta mediante flujos de trabajo de GitHub Actions (consulte [`download.yml`](./.github/workflows/download.yml) y [`download-hls.yml`](./.github/workflows/download-hls.yml)):

1. Configure `rclone` localmente con su cuenta de OneDrive (`rclone config`) y copie el contenido íntegro del archivo generado `rclone.conf` en el secreto del repositorio llamado `RCLONE_CONF`.
2. Genere un GitHub Personal Access Token (PAT) con los permisos `repo` y `workflow` y regístrelo como el secreto `GH_PAT` del repositorio (necesario para persistir la renovación de los tokens de OneDrive).
3. Despliegue el Worker de Cloudflare configurado para disparar eventos `workflow_dispatch` en su repositorio y ejecute en su terminal local: `rdl config set worker <URL-del-worker>`.

</details>

---

## ❓ Preguntas frecuentes (FAQ)

<details open>
<summary><b>❓ Preguntas frecuentes (FAQ)</b></summary>
<br>

**P. ¿Por qué no usar simplemente `wget`, `curl` o `aria2c` en local?**
R. Porque esas herramientas obligan a que todo el volumen de datos (a menudo decenas o cientos de gigabytes) pase por su red doméstica y sature su disco local. `remote-dl` permite descargar un archivo de 100 GB consumiendo exactamente cero bytes de su disco y cero bytes de su ancho de banda local.

**P. ¿Pueden múltiples usuarios compartir una misma instancia?**
R. No. Esta herramienta ha sido diseñada exclusivamente como un sistema monousuario. Soportar múltiples usuarios agregaría una complejidad innecesaria en la gestión de cuotas, aislamiento de credenciales y prevención de abusos, lo cual va en contra de la filosofía de simplicidad y ligereza del proyecto.

**P. ¿Se admiten descargas interrumpibles y reanudables?**
R. Sí. La fase de subida a OneDrive se realiza mediante fragmentos continuos con soporte automático de reanudación. La descarga del enlace de origen se ejecuta como una tubería en una única sesión; si el origen se corta inesperadamente, simplemente vuelva a encolar la tarea.

</details>

---

## 🤝 Contribuciones y agradecimientos

<details open>
<summary><b>🤝 Contribuciones y agradecimientos</b></summary>
<br>

Antes de abrir una solicitud de incorporación de cambios (Pull Request), asegúrese de pasar estas tres verificaciones de calidad obligatorias:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

### Agradecimientos a bibliotecas principales

- [`clap`](https://crates.io/crates/clap) — análisis ultrarrápido y robusto de argumentos de línea de comandos mediante macros derive
- [`reqwest`](https://crates.io/crates/reqwest) + [`rustls`](https://crates.io/crates/rustls) — cliente HTTP puro moderno con seguridad TLS estricta
- [`windows`](https://crates.io/crates/windows) — enlaces oficiales de Microsoft para la API de Win32 (DPAPI)
- [`directories`](https://crates.io/crates/directories) — resolución estándar de directorios de configuración según la plataforma
- [`rclone`](https://rclone.org) — motor avanzado de transferencia y subida por fragmentos a OneDrive en GitHub Actions

</details>

---

## 📄 Licencia y descargo de responsabilidad

<details open>
<summary><b>📄 Licencia y descargo de responsabilidad</b></summary>
<br>

Este proyecto se distribuye bajo los términos de la **Licencia MIT** (consulte [LICENSE](../../LICENSE)).

### Descargo de responsabilidad (Disclaimer)

Esta herramienta está pensada exclusivamente para fines personales de archivado y preservación de contenidos de los cuales usted sea el propietario legítimo o cuente con los permisos de descarga pertinentes. El operador asume la responsabilidad exclusiva de acatar los términos de servicio de los sitios web de origen y la legislación vigente en materia de propiedad intelectual. **Queda expresamente prohibido el uso de esta herramienta para redistribución no autorizada, servicios de proxy público, réplicas no oficiales o difusión a terceros.**

</details>
