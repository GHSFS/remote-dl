<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <strong>🇧🇷 Português</strong>
</p>

# remote-dl

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

### Visão geral

`remote-dl` é um gerenciador pessoal de downloads para usuários que querem
obter arquivos grandes (vídeos, arquivos compactados, datasets, corpora de
pesquisa) **diretamente para seu próprio armazenamento em nuvem** sem
primeiro armazená-los em uma máquina local.

Um pequeno cliente CLI nativo (`rdl.exe`) no seu desktop se comunica com um
backend serverless leve. A transferência real dos bytes acontece inteiramente
na nuvem, então nem seu disco nem sua conexão doméstica entram no caminho
dos dados.

Esta é uma ferramenta self-hosted de usuário único. Você traz sua própria
conta de armazenamento em nuvem; não há recursos compartilhados com outros
usuários.

### Recursos

- **Streaming eficiente em memória** — arquivos de qualquer tamanho passam
  por um pipeline de streaming; nada é armazenado em disco no cliente ou no
  backend
- **Suporte nativo a OneDrive** — uploads em chunks com retomada automática
  via API Microsoft Graph
- **Autenticação de usuário único** — senhas de uso único e tokens
  permanentes revogáveis; sem sistema de contas, sem dependência de
  provedores de identidade de terceiros
- **Múltiplos frontends** — CLI desktop, interface web amigável para mobile e
  bot privado do Telegram compartilham o mesmo backend
- **Isolamento de fronteiras de confiança** — credenciais nunca deixam a
  camada à qual pertencem (operador → edge worker → executor de automação →
  provedor de armazenamento)
- **Tokens auto-rotativos** — refresh tokens do OAuth são renovados e
  persistidos de volta no armazenamento criptografado a cada transferência
  bem-sucedida

### Estrutura do projeto

```
remote-dl/
├── Cargo.toml             Manifesto do pacote, perfil release otimizado em tamanho
├── rust-toolchain.toml    Fixação do toolchain (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml     Alvo de build + CRT estática do MSVC
├── README.md / LICENSE / CHANGELOG.md
├── src/
│   ├── main.rs            Entrada da CLI, dispatcher do clap, saída colorida
│   ├── cli.rs             Definições do clap: get / list / status / config / auth / watch
│   ├── api.rs             Cliente reqwest::blocking (envolve a API do worker)
│   ├── config.rs          Leitura/escrita de config.json + proteção DPAPI do token
│   └── error.rs           Tipo de erro do crate
├── tests/integration.rs   Testes CLI baseados em assert_cmd
└── .github/workflows/
    ├── build.yml          CI de build release do rdl.exe
    ├── test.yml           CI fmt / clippy / test
    ├── download.yml       Automação de backend: URL → OneDrive em streaming
    └── download-hls.yml   Variante HLS / streaming (yt-dlp + rclone rcat)
```

Para instruções detalladas de instalação, início rápido, referência de CLI
e FAQ, consulte a seção [English](#english).

### Licença

MIT. Veja [LICENSE](./LICENSE).
