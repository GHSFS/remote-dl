<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <strong>🇧🇷 Português</strong>
</p>

# remote-dl

> Gerenciador de downloads via streaming direto para a nuvem — envie URLs diretamente para seu armazenamento pessoal sem passar pelo disco local nem consumir sua banda de internet.

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#-instalação-e-implantação)

</div>

---

## 🛡️ Visão Geral e Capacidades Essenciais

<details open>
<summary><b>🛡️ Visão Geral e Capacidades Essenciais</b></summary>
<br>

O `remote-dl` é um gerenciador de downloads para uso pessoal, desenvolvido para operadores que necessitam transferir arquivos volumosos (vídeos em alta resolução, arquivos compactados, grandes datasets ou conjuntos de dados para pesquisa) **diretamente para seu próprio armazenamento na nuvem**, sem a necessidade de baixar e intermediar esses dados pelo disco da sua máquina local.

Um cliente nativo de linha de comando (`rdl.exe`) ultrarrápido em sua área de trabalho interage com um backend sem servidor (serverless). Todo o fluxo de transferência de dados ocorre estritamente dentro da nuvem; portanto, nem o espaço em disco do seu computador nem o consumo da sua conexão residencial são afetados no processo.

Trata-se de uma **ferramenta de auto-hospedagem para um único usuário**. Você utiliza sua própria conta de armazenamento em nuvem pessoal; nenhuma informação é compartilhada com terceiros.

</details>

---

## ⚡ Recursos Principais

<details open>
<summary><b>⚡ Recursos Principais</b></summary>
<br>

- **Streaming contínuo com alta eficiência de memória** — arquivos de qualquer tamanho trafegam por uma esteira em fluxo direto; nenhum dado é temporariamente armazenado em disco, nem no cliente nem no servidor.
- **Suporte nativo ao OneDrive** — envio por blocos (chunks) com suporte completo a retomada automática via API do Microsoft Graph.
- **Autenticação segura e direta para usuário único** — senhas descartáveis (OTP) combinadas com tokens persistentes revogáveis; sem a necessidade de contas complexas ou provedores externos de identidade.
- **Múltiplas interfaces unificadas** — CLI nativo para desktop, interface web adaptada para dispositivos móveis e um bot privado do Telegram conectados ao mesmo backend.
- **Isolamento rigoroso de fronteiras de confiança** — credenciais jamais deixam a camada responsável por elas (operador → Edge Worker → executor de automação → provedor de armazenamento).
- **Autorrotação contínua de credenciais** — tokens de atualização OAuth são renovados e salvos de forma criptografada a cada transferência bem-sucedida, permitindo que o sistema funcione sem interrupções mesmo após meses de inatividade.

</details>

---

## 🏛️ Arquitetura e Topologia do Sistema

<details open>
<summary><b>🏛️ Arquitetura e Topologia do Sistema</b></summary>
<br>

```text
  ┌──────────────────────┐
  │  Cliente CLI rdl.exe │
  │  Interface Web       │ ──HTTPS──┐
  │  Bot do Telegram     │          │
  └──────────────────────┘          ▼
                               ┌──────────────────────┐
                               │  Worker de Borda     │
                               │  (Autenticação + Dispatch)
                               └──────────┬───────────┘
                                          │ workflow_dispatch
                                          ▼
                               ┌──────────────────────┐
                               │  Executor na Nuvem   │
                               │  (Proxy em fluxo)    │
                               └──────────┬───────────┘
                                          │ Envio em blocos
                                          ▼
                               ┌──────────────────────┐
                               │  OneDrive Pessoal    │
                               └──────────────────────┘
```

</details>

---

## 📂 Estrutura do Repositório

<details open>
<summary><b>📂 Estrutura do Repositório</b></summary>
<br>

```text
remote-dl/
├── Cargo.toml                 Manifesto do pacote: define o binário rdl, dependências e perfil de compilação otimizado para tamanho
├── Cargo.lock                 Versões fixadas e exatas de todas as dependências transitivas
├── rust-toolchain.toml        Definição da versão do compilador (stable, x86_64-pc-windows-msvc)
├── .cargo/
│   └── config.toml            Destino de compilação e flags rustflags (linkagem estática ao CRT do MSVC)
├── .gitignore                 Ignora pastas target/, segredos, configurações da IDE e rascunhos
├── README.md                  Documento principal em inglês
├── LICENSE                    Licença de código aberto MIT
├── CHANGELOG.md               Histórico de mudanças no padrão Keep-a-Changelog
├── docs/
│   └── locales/               Documentações traduzidas (ko, zh-CN, ja, es, ru, vi, tr, de, pt-BR)
├── src/
│   ├── main.rs                Ponto de entrada do CLI: análise de argumentos com clap, despacho de comandos e cores
│   ├── cli.rs                 Definições estruturais de comandos com clap: get, list, status, config, auth, watch
│   ├── api.rs                 Cliente REST API em reqwest::blocking para comunicação com o Edge Worker
│   ├── config.rs              Leitura e escrita de %APPDATA%\rdl\config.json com criptografia Windows DPAPI
│   └── error.rs               Enumeração unificada de tratamento de erros com a biblioteca thiserror
├── tests/
│   └── integration.rs         Testes de integração do CLI com assert_cmd (--version, ciclo de configurações, tratamento de erros)
└── .github/workflows/
    ├── build.yml              CI: compilação cargo build --release gerando o binário rdl para x64
    ├── test.yml               CI: validação rigorosa com cargo fmt, cargo clippy e cargo test --all-targets
    ├── download.yml           Fluxo em nuvem: esteira de URL para o OneDrive (curl | rclone rcat)
    └── download-hls.yml       Variante automatizada para transmissões de vídeo HLS (yt-dlp | rclone rcat)
```

</details>

---

## 💻 Compatibilidade e Ambientes Suportados

<details open>
<summary><b>💻 Compatibilidade e Ambientes Suportados</b></summary>
<br>

| Aspecto | Nível de Compatibilidade |
|---|---|
| Sistema operacional (binário cliente) | Windows 10 1809+ e Windows 11 |
| Arquitetura do processador | Exclusivamente x86_64 |
| Cadeia de ferramentas Rust | 1.75+ (fixado em estável via `rust-toolchain.toml`) |
| Vinculador (Linker) | MSVC (Visual Studio Build Tools 2022) |
| Armazenamento na nuvem | Microsoft OneDrive (Pessoal ou Corporativo) via rclone |
| Cliente do Telegram | Qualquer aplicativo oficial ou de terceiros com suporte a Webhooks |

O binário distribuído do cliente é linkado estaticamente com o runtime do MSVC, eliminando dependências de DLLs adicionais fora as bibliotecas essenciais do sistema Windows (`KERNEL32.DLL`, `USER32.DLL`, `ADVAPI32.DLL`, `CRYPT32.DLL`, `WS2_32.DLL`).

</details>

---

## 🔒 Considerações de Segurança e Invariantes

<details open>
<summary><b>🔒 Considerações de Segurança e Invariantes</b></summary>
<br>

- **Criptografia em repouso (Token at rest)** — O token bearer armazenado em `%APPDATA%\rdl\config.json` é protegido por criptografia de hardware do Windows DPAPI (`CryptProtectData`). Apenas a conta de usuário do Windows que gerou o arquivo tem permissão para decifrá-lo.
- **Segurança em trânsito (Token in transit)** — O cliente `reqwest` é compilado com `https_only=true`; os tokens viajam sempre protegidos por TLS no cabeçalho `Authorization`, nunca expostos na URL.
- **Área de ataque restrita** — Apenas o usuário registrado do Telegram (`TG_OWNER_ID`) pode disparar comandos no bot; as chamadas de Webhook exigem uma chave secreta no caminho da URL conhecida apenas pelo Telegram.
- **Omissão de dados sensíveis em logs** — Todas as URLs e nomes de arquivos repassados aos fluxos do GitHub Actions recebem a diretiva `::add-mask::` para impedir vazamentos acidentais nos registros públicos de compilação.
- **Autorrenovação de credenciais OAuth** — Após cada execução concluída, o token de atualização do OneDrive é rotacionado e reescrito no segredo do repositório `RCLONE_CONF`, garantindo pleno funcionamento mesmo após meses sem uso.
- **Sem telemetria** — Nem o cliente CLI nem os serviços de backend transmitem métricas de uso ou relatórios de diagnóstico para servidores de terceiros.

</details>

---

## 🛠️ Matriz de Diagnóstico e Resolução de Problemas

<details open>
<summary><b>🛠️ Matriz de Diagnóstico e Resolução de Problemas</b></summary>
<br>

| Sintoma | Causa Mais Provável | Ação de Resolução |
|---|---|---|
| `worker URL not set` | Primeira execução nesta máquina sem inicialização | Execute `rdl config set worker https://<seu-worker>.workers.dev` |
| `not authenticated — run rdl auth login` | Nenhum token encontrado no cache local | Emita um token permanente pelo bot do Telegram e execute `rdl auth login --token <token>` |
| `server rejected credentials (401)` | Token revogado manualmente ou expirado | Solicite uma nova credencial no bot do Telegram e realize o login novamente |
| `rdl status <id>` retorna `not found` | ID de tarefa incorreto ou o registro foi purgado | Execute `rdl list` para verificar a lista de tarefas ativas e seus IDs corretos |
| O fluxo falha no envio para o OneDrive | O token do OneDrive expirou por inatividade (>90 dias) | No computador local, execute `rclone config reconnect onedrive:` e atualize o segredo `RCLONE_CONF` |
| O bot do Telegram parou de responder | Incompatibilidade no segredo de Webhook ou falha no Worker | Acesse `https://api.telegram.org/bot<TOKEN>/getWebhookInfo` para verificar o estado e inspecione os logs do Worker |

</details>

---

## 📦 Instalação e Implantação

<details open>
<summary><b>📦 Instalação e Implantação</b></summary>
<br>

### Binário pré-compilado (Windows x64)

1. Baixe o executável `rdl-x64.exe` da [página oficial de lançamentos](https://github.com/GHSFS/remote-dl/releases).
2. Coloque-o em um diretório presente no seu `PATH` de sistema (como `C:\Tools\`).
3. Verifique o funcionamento no terminal: `rdl --version`.

### Compilação a partir do código-fonte

Requer Rust 1.75 ou superior e o conjunto de ferramentas MSVC do Visual Studio.

```bash
git clone https://github.com/GHSFS/remote-dl.git
cd remote-dl
cargo build --release
# Binário resultante em: target/release/rdl.exe
```

</details>

---

## 🚀 Guia Rápido e Referência de Comandos da CLI

<details open>
<summary><b>🚀 Guia Rápido e Referência de Comandos da CLI</b></summary>
<br>

```bash
# 1. Aponte o cliente para o seu backend na nuvem
rdl config set worker https://your-worker.example.workers.dev

# 2. Realize a autenticação inicial (apenas uma vez; obtenha o token no bot)
rdl auth login --token <permanent-token>

# 3. Envie uma URL para a fila de downloads na nuvem
rdl https://example.com/dataset.tar.zst

# 4. Acompanhe o progresso da transferência
rdl list
rdl status <job-id>
```

### Especificação Completa dos Comandos da CLI

```text
USAGE:
    rdl [OPTIONS] <SUBCOMMAND>

SUBCOMMANDS:
    get, g <url>                Adiciona uma URL à fila de downloads na nuvem (--name, --folder, --json)
    list, ls                    Exibe a lista e o status dos downloads recentes (--limit, --json)
    status, st [job-id]         Exibe detalhes em tempo real de uma tarefa específica (--json)
    inspect <url>               Inspeção prévia: cabeçalhos HTTP, tamanho e suporte a download particionado (--json)
    batch <file>                Enfileira URLs em lote a partir de um arquivo de texto (--folder, --json)
    ping                        Testa a conectividade com o Edge Worker e a validade da autenticação (--json)
    config <get|set|path|show>  Lê, define, exibe caminhos e faz diagnósticos do arquivo de configurações
    auth <login|logout>         Gerencia o token de autenticação protegido por criptografia Windows DPAPI
    watch                       Executa em modo de monitoramento contínuo da área de transferência (para rdl-tray)
    help                        Exibe a mensagem de ajuda

OPTIONS:
    -j, --json                  Exibe os resultados em formato estruturado JSON para leitura automatizada
    -h, --help                  Exibe a mensagem de ajuda
    -V, --version               Exibe as informações da versão
```

</details>

---

## ⚙️ Configurações e Implantação do Backend

<details open>
<summary><b>⚙️ Configurações e Implantação do Backend</b></summary>
<br>

### Configuração do Cliente Local

O arquivo de configurações é armazenado em `%APPDATA%\rdl\config.json`. Os dados confidenciais são protegidos com o Windows DPAPI para evitar acesso indevido por outros usuários ou processos maliciosos.

| Chave | Descrição |
|---|---|
| `worker` | URL base do Worker da Cloudflare implantado |
| `token` | Token de autenticação permanente (criptografado com DPAPI) |
| `folder` | Nome da pasta padrão de destino dentro do OneDrive |

### Configuração da Automação no Backend

A esteira de processamento em nuvem utiliza as automações do GitHub Actions (veja [`download.yml`](./.github/workflows/download.yml) e [`download-hls.yml`](./.github/workflows/download-hls.yml)):

1. Configure o `rclone` localmente para o OneDrive (`rclone config`) e transfira o conteúdo completo do `rclone.conf` para o segredo de repositório chamado `RCLONE_CONF`.
2. Gere um GitHub Personal Access Token (PAT) com escopos `repo` e `workflow` e salve-o como o segredo `GH_PAT` no repositório (usado na renovação automática dos tokens).
3. Publique seu Worker na Cloudflare habilitado para disparar eventos `workflow_dispatch` e execute no terminal: `rdl config set worker <URL-do-worker>`.

</details>

---

## ❓ Perguntas Frequentes (FAQ)

<details open>
<summary><b>❓ Perguntas Frequentes (FAQ)</b></summary>
<br>

**P. Qual a vantagem sobre usar ferramentas locais como `wget`, `curl` ou `aria2c`?**
R. As ferramentas locais forçam todo o tráfego de dados (geralmente dezenas ou centenas de gigabytes) a passar pelo seu provedor de internet e desgastar o disco do seu computador. O `remote-dl` permite transferir um arquivo de 100 GB consumindo exatamente 0 bytes do seu disco e 0 bytes da sua conexão de internet residencial.

**P. Vários usuários podem utilizar a mesma instância?**
R. Não. A ferramenta foi concebida exclusivamente para uso monousuário. O suporte a múltiplos usuários acrescentaria grande complexidade de gerenciamento de cotas, isolamento de acessos e mitigação de abusos, indo contra a proposta de leveza do projeto.

**P. É possível retomar downloads interrompidos?**
R. Sim. A etapa de envio para o OneDrive opera por blocos com suporte nativo a retomada automática em caso de instabilidade. A leitura do link de origem opera em fluxo contínuo; se houver corte brusco da fonte, basta colocar o download na fila novamente.

</details>

---

## 🤝 Contribuições e Agradecimentos

<details open>
<summary><b>🤝 Contribuições e Agradecimentos</b></summary>
<br>

Antes de enviar uma solicitação de Pull Request (PR), certifique-se de que os três testes a seguir foram aprovados localmente:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

### Agradecimentos às Bibliotecas

- [`clap`](https://crates.io/crates/clap) — interpretação ágil e fortemente tipada de argumentos de linha de comando
- [`reqwest`](https://crates.io/crates/reqwest) + [`rustls`](https://crates.io/crates/rustls) — cliente HTTP nativo focado em segurança TLS estrita
- [`windows`](https://crates.io/crates/windows) — bindings oficiais da Microsoft para a API Win32 (DPAPI)
- [`directories`](https://crates.io/crates/directories) — resolução precisa de diretórios de configuração em conformidade com o sistema operacional
- [`rclone`](https://rclone.org) — robusto mecanismo de upload particionado para o OneDrive em ambientes de automação em nuvem

</details>

---

## 📄 Licença e Isenção de Responsabilidade

<details open>
<summary><b>📄 Licença e Isenção de Responsabilidade</b></summary>
<br>

Distribuído sob os termos da **Licença MIT** (consulte [LICENSE](../../LICENSE)).

### Isenção de Responsabilidade (Disclaimer)

Esta ferramenta é destinada ao arquivamento e guarda de dados para os quais o operador detenha a posse legítima ou autorização legal explícita de download. O usuário assume total responsabilidade por cumprir os termos de serviço dos servidores de origem e a legislação aplicável sobre direitos autorais. **É estritamente vedado o uso deste software para redistribuição não autorizada, serviços públicos de proxy, espelhamento comercial ou compartilhamento indiscriminado de dados com terceiros.**

</details>
