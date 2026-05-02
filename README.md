# remote-dl

> Stream-to-cloud download manager — pipe URLs directly to your personal cloud storage without staging on local disk.

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

[English](#english) · [한국어](#한국어) · [日本語](#日本語) · [中文](#中文) · [Русский](#русский) · [Tiếng Việt](#tiếng-việt) · [Türkçe](#türkçe) · [Deutsch](#deutsch) · [Español](#español) · [Português](#português)

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

---

## 日本語

### 概要

`remote-dl` は、大きなファイル(動画、アーカイブ、データセット、研究用
コーパス)を**ローカルマシンを経由せずに自分のクラウドストレージに直接**
取得したいユーザー向けのパーソナルダウンロードマネージャです。

デスクトップにインストールする小さなネイティブ CLI クライアント
(`rdl.exe`)が軽量なサーバーレスバックエンドと通信し、実際のバイト転送は
すべてクラウド側で行われます。そのためユーザーのディスクと家庭用回線は
データ経路に含まれません。

シングルユーザーのセルフホスト型ツールであり、自分のクラウドアカウントを
直接接続します。他のユーザーと共有されるリソースはありません。

### 特徴

- **メモリ効率の良いストリーミング** — ファイルサイズに関係なく、クライアント
  /バックエンドのどちらにもディスク一時保存なしでパイプライン処理
- **OneDrive ネイティブ対応** — Microsoft Graph API ベースのチャンクアップ
  ロード、自動再開
- **シングルユーザー認証** — 使い捨て OTP + 取り消し可能な永続トークン。
  アカウントシステムなし、外部 IdP 依存なし
- **複数のフロントエンド** — デスクトップ CLI、モバイル対応 Web UI、非公開
  Telegram ボットが同じバックエンドを共有
- **信頼境界の分離** — 認証情報はそれを所有する層(運用者 → エッジワーカー →
  オートメーションランナー → ストレージ)を超えない
- **自己回転トークン** — 転送が成功するたびに OAuth refresh token が更新され、
  暗号化ストレージに保存される

詳細なインストール、クイックスタート、CLI リファレンス、FAQ は
[English](#english) セクションを参照してください。

### ライセンス

MIT。[LICENSE](./LICENSE) を参照。

---

## 中文

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

完整的安装、快速入门、CLI 参考和 FAQ 请参见 [English](#english) 部分。

### 许可证

MIT。详见 [LICENSE](./LICENSE)。

---

## Русский

### Обзор

`remote-dl` — персональный менеджер загрузок для пользователей, которые хотят
скачивать большие файлы (видео, архивы, наборы данных, исследовательские
корпуса) **напрямую в собственное облачное хранилище**, не буферизуя их
сначала через локальную машину.

Маленький нативный CLI-клиент (`rdl.exe`) на вашем рабочем столе общается с
лёгким бессерверным бэкендом. Вся передача байтов происходит исключительно в
облаке, поэтому ваш диск и домашнее соединение не задействованы в пути
данных.

Это однопользовательский self-hosted инструмент. Вы подключаете собственную
учётную запись облачного хранилища; никакие ресурсы не делятся с другими
пользователями.

### Возможности

- **Память-эффективная потоковая передача** — файлы любого размера
  пропускаются через потоковый конвейер; ничего не сохраняется на диск ни на
  клиенте, ни на бэкенде
- **Нативная поддержка OneDrive** — фрагментная загрузка с возобновлением
  через Microsoft Graph API
- **Однопользовательская аутентификация** — одноразовые пароли и отзываемые
  постоянные токены; никакой системы учётных записей, никакой зависимости от
  сторонних провайдеров
- **Несколько фронтендов** — настольный CLI, мобильно-дружественный веб-UI и
  приватный Telegram-бот используют один и тот же бэкенд
- **Изоляция границ доверия** — учётные данные не покидают слой, которому они
  принадлежат (оператор → edge worker → automation runner → провайдер хранения)
- **Самообновляющиеся токены** — refresh token OAuth обновляется и сохраняется
  обратно в зашифрованное хранилище после каждой успешной передачи

Подробные инструкции по установке, быстрому старту, CLI-справочник и FAQ см.
в разделе [English](#english).

### Лицензия

MIT. См. [LICENSE](./LICENSE).

---

## Tiếng Việt

### Tổng quan

`remote-dl` là một trình quản lý tải xuống cá nhân dành cho người dùng muốn
tải các tệp lớn (video, archive, bộ dữ liệu, kho ngữ liệu nghiên cứu) **trực
tiếp vào kho lưu trữ đám mây của riêng họ** mà không cần đệm qua máy cục bộ
trước.

Một client CLI native nhỏ (`rdl.exe`) trên máy tính của bạn giao tiếp với một
backend serverless nhẹ. Việc truyền byte thực tế hoàn toàn diễn ra trên đám
mây, vì vậy ổ đĩa và kết nối nhà của bạn không nằm trên đường truyền dữ liệu.

Đây là công cụ self-hosted dành cho một người dùng. Bạn mang theo tài khoản
lưu trữ đám mây của riêng mình; không có tài nguyên nào được chia sẻ với
người dùng khác.

### Tính năng

- **Truyền dòng tiết kiệm bộ nhớ** — tệp ở bất kỳ kích thước nào đều đi qua
  pipeline luồng; không có gì được lưu tạm vào đĩa ở client hoặc backend
- **Hỗ trợ OneDrive native** — upload theo khối, có thể tiếp tục thông qua
  Microsoft Graph API
- **Xác thực một người dùng** — mật khẩu một lần + token vĩnh viễn có thể
  thu hồi; không có hệ thống tài khoản, không phụ thuộc vào IdP bên thứ ba
- **Nhiều frontend** — CLI desktop, web UI thân thiện với di động và bot
  Telegram riêng tư đều dùng chung backend
- **Cô lập biên giới tin cậy** — thông tin đăng nhập không rời khỏi tầng sở
  hữu chúng (người vận hành → edge worker → automation runner → nhà cung cấp
  lưu trữ)
- **Token tự động xoay vòng** — refresh token OAuth được làm mới và lưu lại
  vào kho mã hoá sau mỗi lần truyền thành công

Hướng dẫn cài đặt chi tiết, khởi động nhanh, tham chiếu CLI và FAQ có ở phần
[English](#english).

### Giấy phép

MIT. Xem [LICENSE](./LICENSE).

---

## Türkçe

### Genel Bakış

`remote-dl`, büyük dosyaları (videolar, arşivler, veri kümeleri, araştırma
külliyatları) önce yerel bir makineden geçirmeden **doğrudan kendi bulut
depolama alanlarına** çekmek isteyen kullanıcılar için tasarlanmış kişisel
bir indirme yöneticisidir.

Masaüstündeki küçük yerel bir CLI istemcisi (`rdl.exe`), hafif bir sunucusuz
arka uçla iletişim kurar. Gerçek bayt aktarımı tamamen bulutta gerçekleşir,
bu nedenle diskiniz ve ev bağlantınız veri yolunda yer almaz.

Bu, tek kullanıcılı, kendinden barındırılan bir araçtır. Kendi bulut depolama
hesabınızı getirirsiniz; başka kullanıcılarla paylaşılan hiçbir kaynak yoktur.

### Özellikler

- **Bellek verimli akış** — herhangi bir boyuttaki dosyalar bir akış hattından
  geçer; ne istemcide ne de arka uçta diske aşamalandırma yapılmaz
- **Yerel OneDrive desteği** — Microsoft Graph API üzerinden parça parça,
  devam ettirilebilir yüklemeler
- **Tek kullanıcılı kimlik doğrulama** — tek seferlik şifreler ve iptal
  edilebilir kalıcı tokenlar; hesap sistemi yok, üçüncü taraf IdP bağımlılığı
  yok
- **Birden çok ön uç** — masaüstü CLI, mobil dostu web arayüzü ve özel
  Telegram botu aynı arka ucu paylaşır
- **Güven sınırı izolasyonu** — kimlik bilgileri ait oldukları katmanı terk
  etmez (operatör → edge worker → otomasyon çalıştırıcı → depolama sağlayıcı)
- **Kendi kendini yenileyen tokenlar** — başarılı her aktarımdan sonra OAuth
  refresh token yenilenir ve şifrelenmiş depolamaya geri kaydedilir

Ayrıntılı kurulum, hızlı başlangıç, CLI referansı ve SSS için [English](#english)
bölümüne bakın.

### Lisans

MIT. [LICENSE](./LICENSE) dosyasına bakın.

---

## Deutsch

### Überblick

`remote-dl` ist ein persönlicher Download-Manager für Nutzer, die große
Dateien (Videos, Archive, Datensätze, Forschungskorpora) **direkt in ihren
eigenen Cloud-Speicher** abrufen möchten, ohne sie zuerst über einen lokalen
Rechner zu puffern.

Ein kleiner nativer CLI-Client (`rdl.exe`) auf deinem Desktop kommuniziert
mit einem leichtgewichtigen serverlosen Backend. Die eigentliche
Byte-Übertragung erfolgt vollständig in der Cloud, sodass weder deine
Festplatte noch deine Heimverbindung im Datenpfad liegen.

Dies ist ein Single-User-Self-Hosted-Tool. Du bringst deinen eigenen
Cloud-Speicher-Account mit; es werden keine Ressourcen mit anderen Nutzern
geteilt.

### Funktionen

- **Speichereffizientes Streaming** — Dateien jeder Größe fließen durch eine
  Streaming-Pipeline; weder im Client noch im Backend wird etwas auf die
  Festplatte ausgelagert
- **Native OneDrive-Unterstützung** — Chunked, wiederaufnehmbare Uploads über
  die Microsoft Graph API
- **Single-User-Authentifizierung** — Einmalpasswörter und widerrufbare
  permanente Tokens; kein Kontosystem, keine Abhängigkeit von Drittanbieter-IdPs
- **Mehrere Frontends** — Desktop-CLI, mobilfreundliche Web-UI und privater
  Telegram-Bot teilen sich dasselbe Backend
- **Trust-Boundary-Isolation** — Anmeldedaten verlassen nie die Ebene, der
  sie gehören (Betreiber → Edge Worker → Automation Runner → Storage-Provider)
- **Selbst rotierende Tokens** — OAuth-Refresh-Tokens werden bei jeder
  erfolgreichen Übertragung erneuert und in den verschlüsselten Speicher
  zurückgeschrieben

Ausführliche Installations-, Schnellstart-, CLI-Referenz- und FAQ-Anleitungen
findest du im Abschnitt [English](#english).

### Lizenz

MIT. Siehe [LICENSE](./LICENSE).

---

## Español

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

Para instrucciones detalladas de instalación, inicio rápido, referencia de
CLI y FAQ, consulta la sección [English](#english).

### Licencia

MIT. Consulta [LICENSE](./LICENSE).

---

## Português

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

Para instruções detalhadas de instalação, início rápido, referência de CLI
e FAQ, consulte a seção [English](#english).

### Licença

MIT. Veja [LICENSE](./LICENSE).
