<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <strong>🇰🇷 한국어</strong> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# remote-dl

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

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

### 프로젝트 구조

```
remote-dl/
├── Cargo.toml             패키지 매니페스트, 크기 최적화 release 프로파일
├── rust-toolchain.toml    툴체인 고정 (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml     빌드 타겟 + 정적 MSVC CRT
├── README.md / LICENSE / CHANGELOG.md
├── src/
│   ├── main.rs            CLI 진입점, clap 디스패처, 컬러 출력
│   ├── cli.rs             clap 정의: get / list / status / config / auth / watch
│   ├── api.rs             reqwest::blocking 클라이언트 (워커 API 래핑)
│   ├── config.rs          config.json 읽기/쓰기 + DPAPI 토큰 보호
│   └── error.rs           크레이트 전체 에러 타입
├── tests/integration.rs   assert_cmd 기반 CLI 테스트
└── .github/workflows/
    ├── build.yml          rdl.exe 릴리스 빌드 CI
    ├── test.yml           fmt / clippy / test CI
    ├── download.yml       백엔드 자동화: URL → OneDrive 스트리밍
    └── download-hls.yml   HLS / 스트리밍 변형 (yt-dlp + rclone rcat)
```

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

### 호환성

| 항목 | 지원 사양 |
|---|---|
| 운영체제 (클라이언트 바이너리) | Windows 10 1809+ 및 Windows 11 |
| 아키텍처 | x86_64 전용 |
| Rust 툴체인 | 1.75+ (`rust-toolchain.toml`에서 stable 고정) |
| 링커 | MSVC (Visual Studio Build Tools 2022) |
| 클라우드 스토리지 | Microsoft OneDrive (개인용 또는 비즈니스, rclone 경유) |
| 텔레그램 클라이언트 | 웹훅 봇을 지원하는 모든 버전 |

배포 바이너리는 MSVC 런타임을 정적 링크하므로 Windows 기본 시스템 DLL(`KERNEL32.DLL`, `USER32.DLL`, `ADVAPI32.DLL`, `CRYPT32.DLL`, `WS2_32.DLL`) 외에 별도의 외부 DLL 의존성이 없습니다.

### 보안 고려사항

- **저장 시 토큰 암호화** — `%APPDATA%\rdl\config.json`에 저장되는 Bearer 토큰은 Windows DPAPI(`CryptProtectData`)로 암호화되어, 생성한 Windows 사용자 계정에서만 복호화할 수 있습니다.
- **전송 구간 암호화** — `reqwest`는 `https_only=true`로 빌드되어 TLS 연결만 허용하며, 토큰은 URL 쿼리가 아닌 `Authorization` 헤더로만 전송됩니다.
- **단일 사용자 인증 표면** — 지정된 텔레그램 소유자(`TG_OWNER_ID`)만 봇을 제어할 수 있으며, 웹훅 경로에도 비밀 토큰이 포함됩니다.
- **워크플로우 입력 마스킹** — GitHub Actions 워크플로우로 전달되는 모든 URL과 파일명은 `::add-mask::`로 마스킹되어 실행 로그에 노출되지 않습니다.
- **OAuth 자동 갱신** — 전송 성공 시 갱신된 OneDrive 리프레시 토큰을 `RCLONE_CONF` 시크릿에 자동 반영하여 장기 미사용 후에도 수동 재인증 없이 유지됩니다.
- **텔레메트리 없음** — CLI와 백엔드 모두 외부 제3자에게 어떠한 사용 통계도 전송하지 않습니다.

### 문제 해결

| 증상 | 주요 원인 | 해결 방법 |
|---|---|---|
| `worker URL not set` | 최초 실행 시 워커 주소 미설정 | `rdl config set worker https://<워커주소>.workers.dev` 실행 |
| `not authenticated — run rdl auth login` | 저장된 토큰 없음 | 텔레그램 봇에서 영구 토큰 발급 후 `rdl auth login --token <토큰>` 실행 |
| `server rejected credentials (401)` | 토큰 폐기 또는 만료 | 봇에서 새 토큰을 발급받아 다시 로그인 |
| `rdl status <id>` 실행 시 `not found` | 잘못된 작업 ID 또는 삭제된 실행 기록 | `rdl list`로 최근 작업 ID 확인 |
| 워크플로우 OneDrive 단계 실패 | 90일 이상 미사용으로 토큰 만료 | 데스크탑에서 `rclone config reconnect onedrive:` 수행 후 `RCLONE_CONF` 시크릿 갱신 |
| 텔레그램 봇 무응답 | 웹훅 시크릿 불일치 또는 워커 오류 | `getWebhookInfo` API 및 Cloudflare Worker 로그 확인 |

### 기여하기

PR 제출 전 아래 세 가지 검사를 통과해야 합니다:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

### 감사의 글

- [`clap`](https://crates.io/crates/clap) — derive 기반 CLI 인자 파싱
- [`reqwest`](https://crates.io/crates/reqwest) + [`rustls`](https://crates.io/crates/rustls) — TLS 전용 HTTP 클라이언트
- [`windows`](https://crates.io/crates/windows) — 공식 Win32 바인딩 (DPAPI 암호화/복호화)
- [`directories`](https://crates.io/crates/directories) — 크로스 플랫폼 설정 경로 탐색
- [`rclone`](https://rclone.org) — GitHub Actions 내부 OneDrive 청크 업로드 수행

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
    rdl [OPTIONS] <SUBCOMMAND>

서브커맨드:
    get, g <url>                URL을 다운로드 큐에 등록 (단축: rdl <url>)
    inspect <url>               사전 검사 (크기, Content-Type, Range 지원 여부)
    batch <file>                텍스트 파일 기반 일괄 다운로드 큐 등록
    list, ls                    최근 다운로드 목록 조회
    status, st [job-id]         작업 상태 조회
    ping                        워커 서버 연결 및 지연 시간 측정
    config <get|set|path|show>  클라이언트 설정 조회 및 관리
    auth <login|logout>         토큰 인증 관리 (Windows DPAPI 암호화)
    watch                       클립보드 감시 모드 실행
    help                        도움말 출력

옵션:
    -j, --json                  결과를 JSON 포맷으로 출력
    -h, --help                  도움말 출력
    -V, --version               버전 정보 출력
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

백엔드 자동화는 [`.github/workflows/download.yml`](./.github/workflows/download.yml)(일반 스트림) 및 [`.github/workflows/download-hls.yml`](./.github/workflows/download-hls.yml)(HLS / `yt-dlp` 스트림)을 통해 동작합니다:

1. 로컬에서 OneDrive용 `rclone config`를 완료한 뒤 생성된 `rclone.conf` 전문을 레포지토리의 `RCLONE_CONF` 시크릿에 등록합니다.
2. `repo` 및 `workflow` 권한을 가진 GitHub PAT를 발급해 `GH_PAT` 시크릿에 등록합니다(전송 완료 시 갱신된 OAuth 토큰을 `RCLONE_CONF`에 자동 저장하는 데 사용).
3. `workflow_dispatch`를 트리거하는 Cloudflare Worker를 배포하고 `rdl config set worker <URL>`로 연결합니다.

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
