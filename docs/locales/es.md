<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <strong>🇪🇸 Español</strong><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# remote-dl

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

### Descripción general

`remote-dl` es un gestor de descargas personal para usuarios que quieren
obtener archivos grandes (vídeos, archivos comprimidos, conjuntos de datos,
corpus de investigación) **directamente en su propio almacenamiento en la
nube** sin almacenarlos primero en una máquina local.

Un pequeño cliente nativo CLI (`rdl.exe`) en tu escritorio se comunica con un
backend ligero sin servidor. La transferencia real de bytes ocurre
íntegramente en la nube, por lo que ni tu disco ni tu conexión doméstica
están en la ruta de datos.

Esta es una herramienta autoalojada de un solo usuario. Tú aportas tu propia
cuenta de almacenamiento en la nube; no se comparte ningún recurso con otros
usuarios.

### Características

- **Streaming eficiente en memoria** — los archivos de cualquier tamaño
  pasan por una tubería de streaming; nada se almacena en disco ni en el
  cliente ni en el backend
- **Soporte nativo de OneDrive** — subidas por fragmentos y reanudables
  mediante la API Microsoft Graph
- **Autenticación de un solo usuario** — contraseñas de un solo uso y tokens
  permanentes revocables; sin sistema de cuentas, sin dependencia de
  proveedores de identidad de terceros
- **Múltiples frontends** — CLI de escritorio, interfaz web amigable para
  móvil y bot privado de Telegram comparten el mismo backend
- **Aislamiento de límites de confianza** — las credenciales nunca salen de
  la capa a la que pertenecen (operador → edge worker → ejecutor de
  automatización → proveedor de almacenamiento)
- **Tokens auto-rotativos** — los tokens de actualización de OAuth se
  refrescan y persisten de vuelta al almacenamiento cifrado tras cada
  transferencia exitosa

### Estructura del proyecto

```
remote-dl/
├── Cargo.toml             Manifiesto del paquete, perfil release optimizado en tamaño
├── rust-toolchain.toml    Fijación del toolchain (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml     Destino de build + CRT estática de MSVC
├── README.md / LICENSE / CHANGELOG.md
├── src/
│   ├── main.rs            Entrada de CLI, dispatcher de clap, salida coloreada
│   ├── cli.rs             Definiciones de clap: get / list / status / config / auth / watch
│   ├── api.rs             Cliente reqwest::blocking (envuelve la API del worker)
│   ├── config.rs          Lectura/escritura de config.json + protección DPAPI del token
│   └── error.rs           Tipo de error del crate
├── tests/integration.rs   Pruebas CLI basadas en assert_cmd
└── .github/workflows/
    ├── build.yml          CI de build release de rdl.exe
    ├── test.yml           CI fmt / clippy / test
    ├── download.yml       Automatización de backend: URL → OneDrive en streaming
    └── download-hls.yml   Variante HLS / streaming (yt-dlp + rclone rcat)
```

Para instrucciones detalladas de instalación, inicio rápido, referencia de
CLI y FAQ, consulta la sección [English](#english).

### Licencia

MIT. Consulta [LICENSE](./LICENSE).
