<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <strong>🇰🇷 한국어</strong> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# remote-dl

> 클라우드 직통 스트리밍 다운로드 매니저 — 로컬 디스크 임시 저장 없이 클라우드 스토리지로 직접 URL 파이프 전송.

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#-설치-및-배포)

</div>

---

## 🛡️ 개요 및 핵심 역량

<details open>
<summary><b>🛡️ 개요 및 핵심 역량</b></summary>
<br>

`remote-dl`은 대용량 파일(영상, 아카이브, 데이터셋, 연구 코퍼스 등)을 **로컬 PC를 거치지 않고 본인 클라우드 스토리지에 바로** 다운로드하기 위한 개인용 스트리밍 다운로드 매니저입니다.

데스크탑에 설치되는 초경량 네이티브 CLI 클라이언트(`rdl.exe`)가 서버리스 백엔드와 통신하며, 실제 파일 다운로드와 클라우드 업로드 전송은 전부 클라우드 러너에서 수행됩니다. 사용자의 로컬 디스크 공간과 가정용 인터넷 회선 대역폭은 데이터 경로에 일절 사용되지 않습니다.

본 프로젝트는 단일 사용자용 셀프호스팅 도구입니다. 본인의 클라우드 스토리지 계정을 연결하여 사용하며, 타 사용자와 어떠한 자원이나 자격증명도 공유되지 않습니다.

</details>

---

## ⚡ 핵심 기능

<details open>
<summary><b>⚡ 핵심 기능</b></summary>
<br>

- **메모리 효율 스트리밍 파이프라인** — 파일 크기와 무관하게 클라이언트나 백엔드 어디에도 임시 디스크 버퍼링 없이 파이프라인으로 전송.
- **OneDrive 네이티브 청크 업로드** — Microsoft Graph API 기반 분할 업로드 및 전송 중단 시 자동 재개 지원.
- **단일 사용자 인증** — 일회용 OTP와 폐기 가능한 영구 토큰. 별도 계정 시스템이나 외부 IdP 의존성 배제.
- **다중 프런트엔드 인터페이스** — 데스크탑 CLI, 모바일 웹 UI, 비공개 텔레그램 봇이 단일 백엔드 완벽 공유.
- **신뢰 경계 격리** — 자격증명은 해당 계층(운영자 → 엣지 워커 → 자동화 러너 → 스토리지 제공자) 내부에서만 엄격히 격리.
- **자가 회전 토큰** — 전송 성공 시마다 OAuth refresh token이 자동 갱신되어 암호화 저장소에 재저장되므로 장기 미사용 후에도 영구 동작.

</details>

---

## 🏛️ 아키텍처 및 시스템 토폴로지

<details open>
<summary><b>🏛️ 아키텍처 및 시스템 토폴로지</b></summary>
<br>

```text
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

</details>

---

## 📂 저장소 디렉터리 구조

<details open>
<summary><b>📂 저장소 디렉터리 구조</b></summary>
<br>

```text
remote-dl/
├── Cargo.toml                 패키지 매니페스트. rdl 바이너리, 의존성, 크기 최적화 릴리즈 프로파일 정의
├── Cargo.lock                 고정된 전이적 의존성 버전
├── rust-toolchain.toml        툴체인 핀 (stable, x86_64-pc-windows-msvc)
├── .cargo/
│   └── config.toml            빌드 타겟 및 rustflags (정적 MSVC CRT 링크)
├── .gitignore                 target/, 시크릿, IDE 상태, 비공개 드래프트 제외
├── README.md                  영문 메인 안내 문서
├── LICENSE                    MIT 라이선스
├── CHANGELOG.md               변경 이력 문서
├── docs/
│   └── locales/               10개국 다국어 문서 (ko, zh-CN, ja, es, ru, vi, tr, de, pt-BR)
├── src/
│   ├── main.rs                CLI 진입점. clap 인자 파싱 및 컬러 터미널 출력
│   ├── cli.rs                 clap derive 서브커맨드 정의: get, list, status, config, auth, watch
│   ├── api.rs                 엣지 워커 통신용 reqwest::blocking 클라이언트
│   ├── config.rs              %APPDATA%\rdl\config.json 입출력 및 Windows DPAPI 암호화
│   └── error.rs               크레이트 표준 thiserror 열거형
├── tests/
│   └── integration.rs         assert_cmd 기반 CLI 통합 테스트
└── .github/workflows/
    ├── build.yml              CI: rdl 바이너리 릴리즈 빌드 및 x64 아티팩트 업로드
    ├── test.yml               CI: cargo fmt, clippy, cargo test 전수 검증
    ├── download.yml           백엔드 자동화: curl | rclone rcat 기반 OneDrive 스트리밍
    └── download-hls.yml       HLS / 스트리밍 미디어 전용 자동화 파이프라인 (yt-dlp)
```

</details>

---

## 💻 호환성 및 실행 환경

<details open>
<summary><b>💻 호환성 및 실행 환경</b></summary>
<br>

| 항목 | 지원 환경 |
|---|---|
| 운영체제 (클라이언트 바이너리) | Windows 10 1809+ 및 Windows 11 |
| 아키텍처 | x86_64 전용 |
| Rust 툴체인 | 1.75+ (`rust-toolchain.toml` 기반 stable 고정) |
| 링커 | MSVC (Visual Studio Build Tools 2022) |
| 클라우드 스토리지 | Microsoft OneDrive (개인용 및 비즈니스 계정, rclone 경유) |
| 텔레그램 클라이언트 | 웹훅 봇을 지원하는 모든 공식/서드파티 클라이언트 |

배포되는 클라이언트 바이너리는 MSVC 런타임을 정적으로 링크하여, Windows 기본 내장 DLL(`KERNEL32.DLL`, `USER32.DLL`, `ADVAPI32.DLL`, `CRYPT32.DLL`, `WS2_32.DLL`) 외에 외부 종속 DLL이 전혀 없습니다.

</details>

---

## 🔒 보안 고려사항

<details open>
<summary><b>🔒 보안 고려사항</b></summary>
<br>

- **로컬 토큰 암호화** — `%APPDATA%\rdl\config.json`에 저장되는 인증 토큰은 Windows DPAPI(`CryptProtectData`)로 암호화되어 생성한 Windows 계정 외부에서는 복호화할 수 없습니다.
- **전송 구간 암호화** — 클라이언트는 `https_only=true`로 강제되어 모든 토큰을 TLS `Authorization` 헤더로만 전송하며 URL에 노출하지 않습니다.
- **단일 사용자 인증 표면** — 허가된 단 1명의 텔레그램 사용자(`TG_OWNER_ID`)만 봇을 조작할 수 있으며, 웹훅 호출 역시 비밀 토큰 경로로만 보호됩니다.
- **워크플로우 파라미터 마스킹** — GitHub Actions 러너로 전달되는 모든 URL과 파일명은 `::add-mask::`로 마스킹되어 실행 로그에 기록되지 않습니다.
- **OAuth 리프레시 토큰 자동 갱신** — 전송 성공 시 갱신된 OneDrive 토큰이 리포지토리 `RCLONE_CONF` 시크릿에 자동 반영되어 수개월간 미사용 후에도 수동 개입 없이 유지됩니다.
- **텔레메트리 일절 없음** — CLI와 백엔드 모두 외부 제3자에게 어떠한 사용 통계나 로그도 전송하지 않습니다.

</details>

---

## 🛠️ 문제 해결 가이드

<details open>
<summary><b>🛠️ 문제 해결 가이드</b></summary>
<br>

| 증상 | 주요 원인 | 해결 방법 |
|---|---|---|
| `worker URL not set` | 워커 엔드포인트 주소 미설정 | `rdl config set worker https://<워커주소>.workers.dev` 실행 |
| `not authenticated — run rdl auth login` | 캐시된 인증 토큰 부재 | 텔레그램 봇에서 영구 토큰 발급 후 `rdl auth login --token <토큰>` 실행 |
| `server rejected credentials (401)` | 토큰 만료 또는 폐기 | 봇에서 새 토큰 발급 후 재로그인 |
| `rdl status <id>` 조회 시 `not found` | 잘못된 작업 ID 또는 삭제된 실행 | `rdl list` 명령어로 현재 활성 작업 ID 확인 |
| 워크플로우 OneDrive 업로드 단계 실패 | 90일 이상 미사용으로 토큰 무효화 | 로컬 PC에서 `rclone config reconnect onedrive:` 실행 후 `RCLONE_CONF` 시크릿 갱신 |
| 텔레그램 봇 무응답 | 웹훅 시크릿 불일치 또는 워커 코드 오류 | `https://api.telegram.org/bot<TOKEN>/getWebhookInfo` 확인 및 Cloudflare Worker 로그 분석 |

</details>

---

## 📦 설치 및 배포

<details open>
<summary><b>📦 설치 및 배포</b></summary>
<br>

### 사전 빌드된 바이너리 (Windows x64)

1. [Releases](https://github.com/GHSFS/remote-dl/releases) 페이지에서 최신 `rdl-x64.exe` 다운로드.
2. `PATH` 환경 변수에 등록된 디렉터리(예: `C:\Tools\`)에 복사.
3. 설치 확인: `rdl --version`.

### 소스코드 직접 빌드

Rust 1.75+ 및 MSVC 빌드 도구가 필요합니다.

```bash
git clone https://github.com/GHSFS/remote-dl.git
cd remote-dl
cargo build --release
# 바이너리 출력 위치: target/release/rdl.exe
```

</details>

---

## 🚀 빠른 시작 및 CLI 사용법

<details open>
<summary><b>🚀 빠른 시작 및 CLI 사용법</b></summary>
<br>

```bash
# 1. 엣지 워커 주소 등록
rdl config set worker https://your-worker.example.workers.dev

# 2. 영구 인증 토큰 로그인 (텔레그램 봇에서 사전 발급)
rdl auth login --token <영구-인증-토큰>

# 3. 클라우드 직통 다운로드 큐 등록
rdl https://example.com/dataset.tar.zst

# 4. 전송 진행 상황 및 상태 조회
rdl list
rdl status <작업-ID>
```

### CLI 명령어 상세 레퍼런스

```text
사용법:
    rdl [OPTIONS] <SUBCOMMAND>

서브커맨드:
    get, g <url>                URL을 다운로드 큐에 등록 (단축: rdl <url>)
    list, ls                    최근 다운로드 목록 및 진행 상태 조회
    status, st [job-id]         특정 작업의 상세 상태 및 로그 조회
    inspect <url>               사전 검사 (크기, Content-Type, Range 지원 여부)
    batch <file>                텍스트 파일 기반 일괄 다운로드 큐 등록
    ping                        워커 서버 연결 및 인증 지연 시간 측정
    config <get|set|path|show>  클라이언트 설정 조회 및 관리
    auth <login|logout>         DPAPI 영구 인증 토큰 관리
    watch                       클립보드 감시 모드 실행 (rdl-tray 연동)
    help                        도움말 출력

옵션:
    -j, --json                  결과를 기계 판독 가능한 JSON 포맷으로 출력
    -h, --help                  도움말 출력
    -V, --version               버전 정보 출력
```

</details>

---

## ⚙️ 설정 및 백엔드 인프라 구성

<details open>
<summary><b>⚙️ 설정 및 백엔드 인프라 구성</b></summary>
<br>

### 클라이언트 설정 파일

설정 파일은 `%APPDATA%\rdl\config.json`에 저장되며, 토큰은 Windows DPAPI로 하드웨어 세션에 암호화 귀속됩니다.

| 설정 키 | 설명 |
|---|---|
| `worker` | 배포된 Cloudflare 엣지 워커의 베이스 URL |
| `token` | 영구 인증 토큰 (DPAPI 암호화 보관) |
| `folder` | OneDrive 기본 저장 폴더명 |

### 백엔드 자동화 구성

백엔드 파이프라인은 GitHub Actions 워크플로우([`download.yml`](./.github/workflows/download.yml), [`download-hls.yml`](./.github/workflows/download-hls.yml))를 통해 동작합니다:

1. 로컬 환경에서 OneDrive용 rclone 설정을 완료(`rclone config`)한 뒤, 생성된 `rclone.conf` 전문을 GitHub 저장소 `RCLONE_CONF` 시크릿에 등록합니다.
2. `repo` 및 `workflow` 권한을 가진 GitHub Personal Access Token을 발급받아 `GH_PAT` 시크릿에 등록합니다.
3. `workflow_dispatch`를 트리거하는 Cloudflare Worker를 배포하고 `rdl config set worker <워커주소>`로 연결합니다.

</details>

---

## ❓ FAQ 및 자주 묻는 질문

<details open>
<summary><b>❓ FAQ 및 자주 묻는 질문</b></summary>
<br>

**Q. 로컬에서 `wget`이나 `curl`, `aria2c`를 쓰는 것과 무엇이 다른가요?**
A. 로컬 도구는 수십~수백 GB의 데이터가 가정용 회선과 PC 디스크를 거쳐야 하므로 병목이 발생합니다. `remote-dl`은 100 GB 파일 전송에도 사용자 로컬 디스크 공간 0B, 가정용 네트워크 트래픽 0B로 전송을 완수합니다.

**Q. 여러 사용자가 하나의 인스턴스를 공유할 수 있나요?**
A. 단일 사용자 전용으로 설계되었습니다. 멀티 테넌시는 인증, 쿼터, 어뷰징 제어 복잡도를 급격히 증가시키므로 단일 소유자 보안 격리를 원칙으로 합니다.

**Q. 다운로드 재개가 지원되나요?**
A. OneDrive 업로드 파이프라인은 청크 단위 자동 재개를 완벽 지원합니다. 소스 URL 다운로드는 작업당 1회 시도되며 오류 발생 시 재등록합니다.

</details>

---

## 🤝 기여 및 감사의 글

<details open>
<summary><b>🤝 기여 및 감사의 글</b></summary>
<br>

풀 리퀘스트(PR) 제출 전 아래 3대 검증을 통과해야 합니다:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

### 감사의 글

- [`clap`](https://crates.io/crates/clap) — derive 기반 초고속 CLI 인자 파싱
- [`reqwest`](https://crates.io/crates/reqwest) + [`rustls`](https://crates.io/crates/rustls) — TLS 전용 HTTP 클라이언트
- [`windows`](https://crates.io/crates/windows) — Microsoft 공식 Win32 바인딩 (DPAPI)
- [`directories`](https://crates.io/crates/directories) — 크로스 플랫폼 표준 설정 경로 탐색
- [`rclone`](https://rclone.org) — GitHub Actions 클라우드 환경 내 OneDrive 청크 업로드 엔진

</details>

---

## 📄 라이선스 및 면책 조항

<details open>
<summary><b>📄 라이선스 및 면책 조항</b></summary>
<br>

본 프로젝트는 **MIT License** ([LICENSE](../../LICENSE)) 하에 배포됩니다.

### 면책 조항 (Disclaimer)

본 도구는 사용자가 소유하고 있거나 합법적으로 다운로드할 권한이 있는 콘텐츠를 개인적으로 보관(아카이빙)하기 위한 목적의 도구입니다. 운영자는 대상 웹사이트의 서비스 약관 및 관련 저작권법을 준수할 전적인 책임이 있습니다. **본 도구를 무단 재배포, 미러링, 프록시 또는 제3자 공유 목적으로 사용하지 마십시오.**

</details>
