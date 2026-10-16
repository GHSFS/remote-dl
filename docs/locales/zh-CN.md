<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <strong>🇨🇳 中文</strong> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# remote-dl

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

### 概述

`remote-dl` 是一个面向希望将大文件(视频、归档、数据集、研究语料)**直接获取
到自己的云存储**而不经过本地机器缓冲的用户的个人下载管理器。

桌面上的小型原生 CLI 客户端(`rdl.exe`)与轻量级的无服务器后端通信,实际的
字节传输完全在云端进行。因此你的磁盘和家庭网络都不在数据路径中。

这是一个单用户、自托管的工具。你需要自带云存储账号;不会与其他用户共享任何
资源。

### 特性

- **内存高效的流式传输** — 任意大小的文件都通过流式管道处理,客户端和后端
  都不会暂存到磁盘
- **OneDrive 原生支持** — 通过 Microsoft Graph API 进行分块、可恢复的上传
- **单用户认证** — 一次性密码 + 可撤销的持久令牌;没有账户系统,不依赖第三方
  身份提供商
- **多种前端** — 桌面 CLI、移动友好的 Web UI 和私有 Telegram 机器人共享同一
  后端
- **信任边界隔离** — 凭据不会离开拥有它们的层(操作者 → 边缘 worker →
  自动化 runner → 存储提供商)
- **自动轮换令牌** — 每次成功传输后,OAuth refresh token 都会刷新并持久化
  回加密存储,系统可以在长期空闲后继续工作

### 项目结构

```
remote-dl/
├── Cargo.toml             包清单,大小优化的 release 配置
├── rust-toolchain.toml    工具链锁定 (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml     构建目标 + 静态 MSVC CRT
├── README.md / LICENSE / CHANGELOG.md
├── src/
│   ├── main.rs            CLI 入口,clap 调度器,彩色输出
│   ├── cli.rs             clap 定义: get / list / status / config / auth / watch
│   ├── api.rs             reqwest::blocking 客户端(封装 worker API)
│   ├── config.rs          config.json 读写 + DPAPI 令牌保护
│   └── error.rs           crate 范围错误类型
├── tests/integration.rs   基于 assert_cmd 的 CLI 测试
└── .github/workflows/
    ├── build.yml          rdl.exe release 构建 CI
    ├── test.yml           fmt / clippy / test CI
    ├── download.yml       后端自动化: URL → OneDrive 流式传输
    └── download-hls.yml   HLS / 流媒体变体 (yt-dlp + rclone rcat)
```

完整的安装、快速入门、CLI 参考和 FAQ 请参见 [English](#english) 部分。

### 许可证

MIT。详见 [LICENSE](./LICENSE)。
