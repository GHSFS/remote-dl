<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <strong>🇨🇳 中文</strong> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# remote-dl

> 流式直传云端下载管理器 —— 直接将网络 URL 管道式直传至个人云存储，全程不占用本地磁盘与带宽。

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#-安装与部署)

</div>

---

## 🛡️ 项目概述与核心能力

<details open>
<summary><b>🛡️ 项目概述与核心能力</b></summary>
<br>

`remote-dl` 是一款专为操作员设计的个人下载管理器，旨在将大型文件（超清视频、大型归档包、公开数据集、学术研究语料）**直接抓取并管道式存入您专属的云存储空间**，彻底摆脱必须先在本地机器缓存下载的传统瓶颈。

桌面端原生极简 CLI 客户端（`rdl.exe`）与轻量级无服务器（Serverless）后端紧密协同。实际的数据传输流完全在云端管道中闭环完成，因此在整个传输生命周期中，既不占用您的本地磁盘空间，也不消耗您的家庭网络上传与下载带宽。

本项目属于**单用户专属自托管工具**。您只需绑定您本人的云存储账号即可使用；绝不与其他第三方用户共享资源或泄露任何数据。

</details>

---

## ⚡ 核心特性

<details open>
<summary><b>⚡ 核心特性</b></summary>
<br>

- **高内存效率流式直传** —— 无论文件体量大小，均通过云端流式管道（Streaming Pipeline）直接对接；客户端与后端均无需在磁盘上暂存任何切片数据。
- **原生深度适配 OneDrive** —— 基于 Microsoft Graph API 原生提供分块（Chunked）且支持断点续传的上传能力。
- **单用户极简鉴权** —— 一次性动态口令（OTP）结合可吊销的永久令牌；无需繁琐复杂的账户管理系统，不依赖任何第三方身份提供商。
- **多端统一接入** —— 桌面端原生 CLI、移动端适配 Web UI 以及专属私有 Telegram Bot 均无缝共享同一个后端服务。
- **严格信任边界隔离** —— 敏感凭据永远留在对应权限层（操作员 → 边缘 Worker → 自动化运行器 → 云存储提供商），杜绝跨层泄露风险。
- **令牌自动轮换保活** —— 每次数据流传输成功后，OAuth 刷新令牌都会自动轮换并持久化加密回存，确保系统在经历超长闲置期后依然随时可用。

</details>

---

## 🏛️ 系统架构与拓扑图

<details open>
<summary><b>🏛️ 系统架构与拓扑图</b></summary>
<br>

```text
  ┌──────────────────────┐
  │  rdl.exe CLI 客户端  │
  │  Web 管理界面        │ ──HTTPS──┐
  │  Telegram 机器人     │          │
  └──────────────────────┘          ▼
                               ┌──────────────────────┐
                               │  边缘 Worker 节点    │
                               │  (鉴权校验 + 调度分发)│
                               └──────────┬───────────┘
                                          │ workflow_dispatch
                                          ▼
                               ┌──────────────────────┐
                               │  云端自动化运行器    │
                               │  (流式管道中继代理)  │
                               └──────────┬───────────┘
                                          │ 分块上传
                                          ▼
                               ┌──────────────────────┐
                               │  OneDrive 云存储     │
                               └──────────────────────┘
```

</details>

---

## 📂 仓库目录结构

<details open>
<summary><b>📂 仓库目录结构</b></summary>
<br>

```text
remote-dl/
├── Cargo.toml                 项目清单文件：定义 rdl 二进制文件、依赖项以及体积极致优化的 release 构建配置
├── Cargo.lock                 精确锁定的传递依赖版本库
├── rust-toolchain.toml        编译器工具链锁定 (stable, x86_64-pc-windows-msvc)
├── .cargo/
│   └── config.toml            构建目标与 rustflags 优化参数 (静态链接 MSVC CRT)
├── .gitignore                 排除 target/、私钥凭据、IDE 配置及草稿
├── README.md                  英文主文档
├── LICENSE                    MIT 开源许可证
├── CHANGELOG.md               遵循 Keep-a-Changelog 规范的版本变更日志
├── docs/
│   └── locales/               多语言本地化文档集 (ko, zh-CN, ja, es, ru, vi, tr, de, pt-BR)
├── src/
│   ├── main.rs                CLI 应用程序入口：解析 clap 参数、分发指令、高亮色彩输出
│   ├── cli.rs                 clap derive 命令行定义：get, list, status, config, auth, watch
│   ├── api.rs                 基于 reqwest::blocking 的边缘 Worker REST API 客户端
│   ├── config.rs              %APPDATA%\rdl\config.json 读写及 Windows DPAPI 硬件级加密
│   └── error.rs               基于 thiserror 的项目统一错误枚举与处理机制
├── tests/
│   └── integration.rs         基于 assert_cmd 的集成测试 (--version, 配置文件往返, 错误状态)
└── .github/workflows/
    ├── build.yml              CI: 执行 cargo build --release 生成 rdl 二进制并归档 x64 构件
    ├── test.yml               CI: 严格执行 cargo fmt, cargo clippy 与 cargo test --all-targets
    ├── download.yml           后端云自动化流水线：URL 直传 OneDrive 流 (curl | rclone rcat)
    └── download-hls.yml       HLS / 流媒体流式直传自动化变体 (yt-dlp | rclone rcat)
```

</details>

---

## 💻 兼容性与运行环境支持

<details open>
<summary><b>💻 兼容性与运行环境支持</b></summary>
<br>

| 维度 | 支持范围 |
|---|---|
| 操作系统（客户端二进制） | Windows 10 1809+ 及 Windows 11 |
| CPU 架构 | 仅支持 x86_64 |
| Rust 编译工具链 | 1.75+（`rust-toolchain.toml` 固定稳定版） |
| 链接器 | MSVC（Visual Studio Build Tools 2022） |
| 云存储提供商 | 个人版或商业版 Microsoft OneDrive（通过 rclone 对接） |
| Telegram 客户端 | 支持 Webhook 机器人的所有官方及第三方版本 |

发布的客户端预编译二进制采用静态链接 MSVC 运行库，除了 Windows 标准系统动态链接库（`KERNEL32.DLL`, `USER32.DLL`, `ADVAPI32.DLL`, `CRYPT32.DLL`, `WS2_32.DLL`）之外，没有任何外部第三方 DLL 依赖。

</details>

---

## 🔒 安全考量与设计不变量

<details open>
<summary><b>🔒 安全考量与设计不变量</b></summary>
<br>

- **静态配置保密（Token at rest）** —— 存放在 `%APPDATA%\rdl\config.json` 中的访问令牌均经过 Windows DPAPI（`CryptProtectData`）强加密包裹。只有生成该配置的 Windows 本机用户账户才有权解密。
- **传输链路加密（Token in transit）** —— `reqwest` 客户端强制开启 `https_only=true`；认证令牌始终通过 TLS 加密通道在 `Authorization` 请求头中发送，绝不拼接入 URL。
- **鉴权暴露面收敛** —— 仅允许绑定的单一 Telegram 用户（`TG_OWNER_ID`）下发指令；Webhook 调用必须匹配仅与 Telegram 共享的 URL 路径私密密钥。
- **敏感参数日志脱敏** —— 所有传递给 GitHub Actions 工作流的下载 URL 和文件名，均自动施加 `::add-mask::` 遮罩，防止敏感链接泄露在公开构建日志中。
- **OAuth 令牌自愈轮换** —— 每次流水线成功执行，OneDrive 的刷新令牌均会自动完成轮换并写回仓库的 `RCLONE_CONF` 密钥库，即便长达数月不使用也不会导致凭据失效。
- **零遥测与隐私保护** —— 无论是 CLI 客户端还是后端服务，均绝不上报任何遥测数据、运行日志或用户统计指标至任何第三方服务器。

</details>

---

## 🛠️ 故障排查排查表

<details open>
<summary><b>🛠️ 故障排查排查表</b></summary>
<br>

| 故障现象 | 潜在根本原因 | 推荐解决方案 |
|---|---|---|
| `worker URL not set` | 在当前机器上首次运行且未初始化 | 执行 `rdl config set worker https://<你的Worker>.workers.dev` |
| `not authenticated — run rdl auth login` | 本地未缓存有效认证令牌 | 通过 Telegram 机器人生成永久令牌后执行 `rdl auth login --token <令牌>` |
| `server rejected credentials (401)` | 认证令牌已被主动吊销或失效 | 在 Telegram 机器人中重新签发新令牌并重新登入 |
| `rdl status <id>` 返回 `not found` | 任务 ID 输入有误或该记录已被云端清理 | 执行 `rdl list` 查看当前活跃的任务列表与实际 ID |
| 工作流在 OneDrive 上传步骤失败 | OneDrive 访问凭据超过 90 天未活动已过期 | 在桌面端执行 `rclone config reconnect onedrive:` 重新授权并更新仓库 `RCLONE_CONF` Secret |
| Telegram 机器人无任何消息响应 | Webhook 密钥不匹配或 Worker 脚本抛错 | 访问 `https://api.telegram.org/bot<TOKEN>/getWebhookInfo` 排查状态；检查 Worker 实时日志 |

</details>

---

## 📦 安装与部署

<details open>
<summary><b>📦 安装与部署</b></summary>
<br>

### 预编译二进制安装 (Windows x64)

1. 从官方 [Releases 页面](https://github.com/GHSFS/remote-dl/releases) 下载最新版的 `rdl-x64.exe`。
2. 将可执行文件放置在系统环境变量 `PATH` 包含的目录下（例如 `C:\Tools\`）。
3. 打开终端验证：执行 `rdl --version`。

### 从源码编译

需要安装 Rust 1.75+ 以及 Visual Studio MSVC 构建工具。

```bash
git clone https://github.com/GHSFS/remote-dl.git
cd remote-dl
cargo build --release
# 生成文件位置: target/release/rdl.exe
```

</details>

---

## 🚀 快速上手与 CLI 命令规范

<details open>
<summary><b>🚀 快速上手与 CLI 命令规范</b></summary>
<br>

```bash
# 1. 配置客户端连接至您的无服务器后端
rdl config set worker https://your-worker.example.workers.dev

# 2. 身份认证（首次仅需一次；先从私有机器人获取永久令牌）
rdl auth login --token <permanent-token>

# 3. 提交下载任务至云端
rdl https://example.com/dataset.tar.zst

# 4. 查看传输进度与状态
rdl list
rdl status <job-id>
```

### CLI 详细指令规范

```text
USAGE:
    rdl [OPTIONS] <SUBCOMMAND>

SUBCOMMANDS:
    get, g <url>                将指定 URL 加入云端下载队列 (--name, --folder, --json)
    list, ls                    列出最近的下载任务及进度 (--limit, --json)
    status, st [job-id]         查看指定任务的实时运行状态 (--json)
    inspect <url>               预检远程目标：响应头、文件体量、Range 分块支持 (--json)
    batch <file>                从文本文件中批量导入并分发下载 URL (--folder, --json)
    ping                        测试与边缘 Worker 节点的连通性与身份有效性 (--json)
    config <get|set|path|show>  读取、设置、查看路径及诊断客户端配置文件
    auth <login|logout>         管理经 Windows DPAPI 硬件加密的永久认证令牌
    watch                       以剪贴板监视模式常驻运行（rdl-tray 托盘配套模式）
    help                        打印帮助信息

OPTIONS:
    -j, --json                  以机器可读的 JSON 结构化格式输出
    -h, --help                  打印帮助信息
    -V, --version               打印版本信息
```

</details>

---

## ⚙️ 详细配置与后端部署说明

<details open>
<summary><b>⚙️ 详细配置与后端部署说明</b></summary>
<br>

### 客户端配置说明

配置文件默认保存在系统安全路径 `%APPDATA%\rdl\config.json`。敏感凭据在写入前均由 Windows DPAPI 深度加密，杜绝其他进程或非法账户跨权读取。

| 配置项 | 含义与说明 |
|---|---|
| `worker` | 已部署的 Cloudflare 边缘 Worker 基础根地址 |
| `token` | 永久身份验证令牌（受 DPAPI 硬件加密保护） |
| `folder` | OneDrive 中的默认存储目标文件夹名称 |

### 后端自动化流水线配置

云端处理流水线依托 GitHub Actions 自动化体系运行（见 [`download.yml`](./.github/workflows/download.yml) 与 [`download-hls.yml`](./.github/workflows/download-hls.yml)）：

1. 在本地通过 `rclone config` 完成针对 OneDrive 的授权配置，并将最终生成的 `rclone.conf` 完整内容存入 GitHub 仓库名为 `RCLONE_CONF` 的 Secret 中。
2. 生成一个拥有 `repo` 和 `workflow` 范围权限的 GitHub Personal Access Token (PAT)，并将其存入仓库 `GH_PAT` Secret（用于在每次转存后自动持久化更新轮换的 OAuth 凭据）。
3. 部署 Cloudflare 边缘 Worker 代码（负责接收请求并触发仓库的 `workflow_dispatch` 事件），然后本地执行 `rdl config set worker <Worker地址>` 完成连通。

</details>

---

## ❓ 常见问题与解答 (FAQ)

<details open>
<summary><b>❓ 常见问题与解答 (FAQ)</b></summary>
<br>

**问：为什么不直接在本地电脑上使用 `wget`、`curl` 或 `aria2c`？**
答：本地下载意味着数十甚至数百 GB 的巨量数据流必须经过您的家庭网络带宽和电脑物理硬盘，这正是导致网络卡顿和磁盘损耗的直接瓶颈。`remote-dl` 让一个 100 GB 的文件转存过程彻底消耗 0 字节本地磁盘和 0 字节本地网络带宽。

**问：是否支持多个用户共享同一个后端实例？**
答：不支持。本系统设计理念为严格的单用户私有工具。多租户架构会引入复杂的配额限制、身份隔离与防滥用机制，这与极简、高效、个人私有的核心定位不符。

**问：下载任务支持断点续传吗？**
答：支持。向 OneDrive 上传的链路完全采用分块流式上传，断点续传由底层引擎自动保障。远程源 URL 的下载在单次任务中按完整管道处理，若发生源站超时或中断，直接重新排队即可。

</details>

---

## 🤝 贡献指南与致谢

<details open>
<summary><b>🤝 贡献指南与致谢</b></summary>
<br>

在提交 Pull Request (PR) 之前，必须在本地完整通过以下三大质量门禁检查：

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

### 致谢依赖库

- [`clap`](https://crates.io/crates/clap) — 基于 derive 宏的高性能 CLI 参数解析库
- [`reqwest`](https://crates.io/crates/reqwest) + [`rustls`](https://crates.io/crates/rustls) — 专注于现代化 TLS 的 HTTP 客户端
- [`windows`](https://crates.io/crates/windows) — Microsoft 官方 Win32 深度绑定库 (DPAPI)
- [`directories`](https://crates.io/crates/directories) — 符合平台规范的标准配置路径查找库
- [`rclone`](https://rclone.org) — GitHub Actions 云端环境中高效稳定的分块上传引擎

</details>

---

## 📄 开源许可证与法律免责声明

<details open>
<summary><b>📄 开源许可证与法律免责声明</b></summary>
<br>

本项目遵循 **MIT 开源许可证** 发布（详见 [LICENSE](../../LICENSE)）。

### 免责声明 (Disclaimer)

本工具仅供技术研究以及用户合法归档其拥有所有权或已获得明确授权下载的个人数据之用。操作员须自行全权遵守目标源网站的服务条款（TOS）及所在司法辖区的相关著作权法律法规。**严禁使用本工具进行任何未授权重新分发、公开镜像、商业代理或第三方共享行为。**

</details>
