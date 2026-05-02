# remote-dl

> Stream-to-cloud download manager — pipe URLs directly to your personal cloud storage without staging on local disk.

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

[English](#english) · [한국어](#한국어)

---

## English

### Overview

`remote-dl` is a personal download manager for users who want to fetch large files
(videos, archives, datasets, research corpora) **directly into their own cloud
storage** without first buffering them through a local machine.

A small native CLI client (`rdl.exe`) on your desktop coordinates with a
lightweight serverless backend. The actual byte transfer happens entirely in the
cloud, so neither your disk nor your home connection is involved in the data path.

This is a single-user, self-hosted tool. You bring your own cloud storage
account; nothing is shared with other users.

### Features

- **Memory-efficient streaming** — files of any size flow through a streaming
  pipeline; nothing is staged to disk on either client or backend
- **OneDrive native** — chunked, resumable uploads via the Microsoft Graph API
- **Single-user auth** — one-time passwords plus revocable persistent tokens; no
  account system, no third-party identity providers
- **Multiple frontends** — desktop CLI, mobile-friendly web UI, and a private
  Telegram bot all share the same backend
- **Trust boundary isolation** — credentials never leave the layer that owns
  them (operator → edge worker → automation runner → storage provider)
- **Self-rotating tokens** — OAuth refresh tokens are refreshed and persisted
  back into encrypted storage on every successful transfer, so the system
  survives indefinite periods of inactivity

### Architecture

```
  ┌──────────────────────┐
  │  rdl.exe CLI         │
  │  Web UI              │ ──HTTPS──┐
  │  Telegram bot        │          │
  └──────────────────────┘          ▼
                              ┌──────────────────────┐
                              │  Edge worker         │
                              │  (auth + dispatch)   │
                              └──────────┬───────────┘
                                         │ workflow_dispatch
                                         ▼
                              ┌──────────────────────┐
                              │  Automation runner   │
                              │  (streaming proxy)   │
                              └──────────┬───────────┘
                                         │ chunked upload
                                         ▼
                              ┌──────────────────────┐
                              │  OneDrive            │
                              └──────────────────────┘
```

### Installation

#### Pre-built binary (Windows x64)

1. Download `rdl-x64.exe` from the [Releases](https://github.com/GHSFS/remote-dl/releases) page.
2. Place it on your `PATH` (e.g. `C:\Tools\`).
3. Verify: `rdl --version`.

#### Build from source

Requires Rust 1.75+ and the MSVC toolchain.

```bash
git clone https://github.com/GHSFS/remote-dl.git
cd remote-dl
cargo build --release
# Output: target/release/rdl.exe
```

### Quick start

```bash
# 1. Point the client at your backend
rdl config set worker https://your-worker.example.workers.dev

# 2. Authenticate (one-time; obtain a permanent token from the bot first)
rdl auth login --token <permanent-token>

# 3. Queue a download
rdl https://example.com/dataset.tar.zst

# 4. Track progress
rdl list
rdl status <job-id>
```

### CLI reference

```
USAGE:
    rdl <SUBCOMMAND>

SUBCOMMANDS:
    <url>                       Queue a URL for download
    list                        Show recent downloads
    status [job-id]             Show job status
    config <get|set> <key> [v]  Read/write client config
    auth <login|logout>         Manage authentication
    watch                       Run in clipboard-monitor mode
    help                        Print help
```

### Configuration

Config lives at `%APPDATA%\rdl\config.json`. Tokens are encrypted with the
Windows DPAPI so they cannot be read outside the originating user account.

| Key       | Description                              |
|-----------|------------------------------------------|
| `worker`  | Base URL of the deployed edge worker     |
| `token`   | Permanent auth token (DPAPI-encrypted)   |
| `folder`  | Default destination folder               |

### Backend setup

Step-by-step setup guides for the backend infrastructure
(edge worker, automation workflows, Telegram bot) are in [`docs/`](./docs/).

### FAQ

**Q. Why not just use `wget` / `curl` / `aria2c` locally?**
A. Because the data path goes through your home network and disk, which is the
exact bottleneck this project removes. `remote-dl` makes a 100 GB transfer cost
zero local disk and zero local bandwidth.

**Q. Can multiple users share an instance?**
A. No, this is intentionally a single-user tool. Multi-tenancy adds significant
complexity to auth, quotas, and abuse handling that is out of scope.

**Q. Are downloads resumable?**
A. Yes — uploads to OneDrive are chunked and auto-resume on the upload side.
The download side is one-shot per job; if it fails, re-queue.

### License

MIT. See [LICENSE](./LICENSE).

### Disclaimer

This is a personal tool intended for archiving content that you own or that
you are authorized to download. The operator is solely responsible for
complying with the terms of service of any source website and with applicable
copyright law. **Do not use this tool to redistribute, mirror, proxy, or share
content with third parties.**

---

## 한국어

### 개요

`remote-dl`은 큰 파일(영상, 아카이브, 데이터셋, 연구 말뭉치 등)을 **로컬을
거치지 않고 본인 클라우드 스토리지에 바로** 받기 위한 개인용 다운로드
매니저입니다.

데스크탑에 설치하는 작은 네이티브 CLI 클라이언트(`rdl.exe`)가 가벼운 서버리스
백엔드와 통신하면, 실제 바이트 전송은 전부 클라우드에서 이루어집니다. 따라서
사용자의 디스크와 가정용 회선은 데이터 경로에 들어가지 않습니다.

단일 사용자용 셀프호스팅 도구이며, 본인 클라우드 계정을 직접 연결합니다.
다른 사용자와 공유되는 자원은 없습니다.

### 특징

- **메모리 효율 스트리밍** — 파일 크기와 무관하게 클라이언트/백엔드 어느
  쪽에도 디스크 임시 저장 없이 파이프라인 처리
- **OneDrive 네이티브 지원** — Microsoft Graph API 기반 청크 업로드, 자동 재개
- **단일 사용자 인증** — 일회용 OTP + 폐기 가능한 영구 토큰. 계정 시스템 없음,
  외부 IdP 의존 없음
- **다중 프런트엔드** — 데스크탑 CLI, 모바일 친화적 웹 UI, 비공개 텔레그램
  봇이 동일 백엔드 공유
- **신뢰 경계 격리** — 자격증명은 그것을 소유한 계층(운영자 → 엣지 워커 →
  자동화 러너 → 스토리지)을 벗어나지 않음
- **자가 회전 토큰** — 전송이 성공할 때마다 OAuth refresh token이 갱신되어
  암호화 저장소에 다시 저장됨. 장기 비활성 상태에서도 동작 보장

### 아키텍처

```
  ┌──────────────────────┐
  │  rdl.exe CLI         │
  │  웹 UI               │ ──HTTPS──┐
  │  텔레그램 봇         │          │
  └──────────────────────┘          ▼
                              ┌──────────────────────┐
                              │  엣지 워커           │
                              │  (인증 + 디스패치)   │
                              └──────────┬───────────┘
                                         │ workflow_dispatch
                                         ▼
                              ┌──────────────────────┐
                              │  자동화 러너         │
                              │  (스트리밍 프록시)   │
                              └──────────┬───────────┘
                                         │ 청크 업로드
                                         ▼
                              ┌──────────────────────┐
                              │  OneDrive            │
                              └──────────────────────┘
```

### 설치

#### 사전 빌드 바이너리 (Windows x64)

1. [Releases](https://github.com/GHSFS/remote-dl/releases) 페이지에서
   `rdl-x64.exe` 다운로드
2. `PATH`에 포함된 경로(예: `C:\Tools\`)에 배치
3. 확인: `rdl --version`

#### 소스 빌드

Rust 1.75+ 및 MSVC 툴체인 필요.

```bash
git clone https://github.com/GHSFS/remote-dl.git
cd remote-dl
cargo build --release
# 결과물: target/release/rdl.exe
```

### 빠른 시작

```bash
# 1. 백엔드 주소 설정
rdl config set worker https://your-worker.example.workers.dev

# 2. 인증 (영구 토큰은 텔레그램 봇에서 먼저 발급)
rdl auth login --token <영구-토큰>

# 3. 다운로드 큐잉
rdl https://example.com/dataset.tar.zst

# 4. 진행 상황 확인
rdl list
rdl status <job-id>
```

### CLI 레퍼런스

```
사용법:
    rdl <SUBCOMMAND>

서브커맨드:
    <url>                       URL을 다운로드 큐에 등록
    list                        최근 다운로드 목록
    status [job-id]             특정 잡 상태
    config <get|set> <key> [v]  클라이언트 설정 조회/변경
    auth <login|logout>         인증 관리
    watch                       클립보드 감시 모드 실행
    help                        도움말 출력
```

### 설정

설정 파일은 `%APPDATA%\rdl\config.json`에 저장됩니다. 토큰은 Windows DPAPI로
암호화되므로 발급받은 사용자 계정 외에서는 복호화할 수 없습니다.

| 키        | 설명                                  |
|-----------|---------------------------------------|
| `worker`  | 배포된 엣지 워커의 베이스 URL         |
| `token`   | 영구 인증 토큰 (DPAPI 암호화)         |
| `folder`  | 기본 저장 폴더                        |

### 백엔드 셋업

엣지 워커, 자동화 워크플로우, 텔레그램 봇 등 백엔드 인프라 셋업 가이드는
[`docs/`](./docs/) 폴더를 참고하세요.

### FAQ

**Q. 그냥 로컬에서 `wget` / `curl` / `aria2c` 쓰면 되는 거 아닌가요?**
A. 그러면 데이터가 가정용 회선과 디스크를 통과하게 되는데, 본 프로젝트는
정확히 그 병목을 없애는 게 목적입니다. `remote-dl`을 쓰면 100GB 전송에
사용자 디스크와 회선 부담이 0입니다.

**Q. 여러 명이 같이 쓸 수 있나요?**
A. 의도적으로 단일 사용자 전용입니다. 멀티 테넌시는 인증/쿼터/어뷰즈 처리
복잡도가 크게 증가하므로 본 프로젝트의 범위 밖입니다.

**Q. 다운로드 재개가 되나요?**
A. OneDrive 업로드 쪽은 청크 단위 자동 재개가 됩니다. 다운로드 쪽은 잡당
1회 시도이며, 실패 시 재큐잉이 필요합니다.

### 라이선스

MIT. [LICENSE](./LICENSE) 참고.

### 면책

본 프로젝트는 사용자 본인이 소유하거나 다운로드 권한이 있는 컨텐츠를
아카이브하기 위한 개인용 도구입니다. 출처 사이트의 이용약관 및 저작권법
준수는 전적으로 운영자(사용자) 본인의 책임이며, **본 도구를 제3자에 대한
재배포, 미러링, 프록시, 공유 용도로 사용하지 마십시오.**
