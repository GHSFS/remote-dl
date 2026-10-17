<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <strong>🇯🇵 日本語</strong> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# remote-dl

> クラウド直結型ストリーミング・ダウンロードマネージャー —— ローカルディスクや家庭内帯域を経由せず、Web上のURLを個人のクラウドストレージへ直接パイプライン転送。

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#-インストールとデプロイ)

</div>

---

## 🛡️ プロジェクト概要と主要機能

<details open>
<summary><b>🛡️ プロジェクト概要と主要機能</b></summary>
<br>

`remote-dl` は、大容量ファイル（高解像度動画、アーカイブ、データセット、研究コーパスなど）をローカルPCに一時ダウンロードすることなく、**自身のクラウドストレージへ直接ストリーミング転送**したいオペレーターのための個人専用ダウンロードマネージャーです。

デスクトップ上のネイティブCLIクライアント（`rdl.exe`）が軽量なサーバーレスバックエンドと連携します。実際のデータバイト転送は完全にクラウドアスフィア内で完結するため、ローカルマシンのストレージ容量を一切消費せず、自宅のインターネット回線帯域も圧迫しません。

本ツールは**完全なシングルユーザー・セルフホスト設計**です。ユーザー自身のクラウドストレージアカウントを直接連携させるため、第三者とリソースを共有することは一切ありません。

</details>

---

## ⚡ コア機能

<details open>
<summary><b>⚡ コア機能</b></summary>
<br>

- **高メモリ効率ストリーミングパイプライン** —— ファイルサイズにかかわらず、データをメモリ上のストリームとして直結転送。クライアント側・サーバー側ともにディスクへの一時保存（ステージング）を行いません。
- **OneDriveネイティブ対応** —— Microsoft Graph APIを活用したチャンク分割およびレジューム（中断再開）対応アップロード。
- **シンプルかつ堅牢なシングルユーザー認証** —— ワンタイムパスワード（OTP）と取り消し可能な永続トークンの組み合わせ。複雑なアカウント基盤や外部のIDプロバイダーへの依存を排除。
- **マルチフロントエンド統合** —— デスクトップCLI、モバイル対応Web UI、プライベートTelegram Botのすべてが同一のバックエンドをシームレスに共有。
- **厳格な信頼境界（Trust Boundary）の隔離** —— 認証情報は各レイヤー（オペレーター → Edge Worker → 自動化ランナー → ストレージプロバイダー）内に厳格に隔離され、不必要に漏洩しません。
- **トークンの自己自動ローテーション** —— データ転送成功のたびにOAuthリフレッシュトークンを自動更新して暗号化リポジトリに再保存するため、長期間利用しない場合でも永続的な稼働性を維持。

</details>

---

## 🏛️ システムアーキテクチャとトポロジー

<details open>
<summary><b>🏛️ システムアーキテクチャとトポロジー</b></summary>
<br>

```text
  ┌──────────────────────┐
  │  rdl.exe CLI         │
  │  Web 管理 UI         │ ──HTTPS──┐
  │  Telegram Bot        │          │
  └──────────────────────┘          ▼
                               ┌──────────────────────┐
                               │  Edge Worker         │
                               │  (認証検証 + 配信)   │
                               └──────────┬───────────┘
                                          │ workflow_dispatch
                                          ▼
                               ┌──────────────────────┐
                               │  クラウド自動化ランナー│
                               │  (ストリーミング中継)│
                               └──────────┬───────────┘
                                          │ チャンク転送
                                          ▼
                               ┌──────────────────────┐
                               │  OneDrive ストレージ │
                               └──────────────────────┘
```

</details>

---

## 📂 リポジトリ構成

<details open>
<summary><b>📂 リポジトリ構成</b></summary>
<br>

```text
remote-dl/
├── Cargo.toml                 パッケージマニフェスト：rdlバイナリ、依存関係、極限まで最適化されたリリースプロファイルを定義
├── Cargo.lock                 推移的依存関係の正確なバージョンロックファイル
├── rust-toolchain.toml        ビルドツールチェーン固定定義 (stable, x86_64-pc-windows-msvc)
├── .cargo/
│   └── config.toml            ビルドターゲットとrustflags最適化設定 (MSVC CRT静的リンク)
├── .gitignore                 target/、秘密鍵、IDE設定、下書きの除外設定
├── README.md                  英語メインドキュメント
├── LICENSE                    MIT ライセンス
├── CHANGELOG.md               Keep-a-Changelog 準拠の変更履歴
├── docs/
│   └── locales/               多言語ローカライズドキュメント群 (ko, zh-CN, ja, es, ru, vi, tr, de, pt-BR)
├── src/
│   ├── main.rs                CLIエントリーポイント：clap引数解析、コマンド配信、カラー出力
│   ├── cli.rs                 clap deriveによるコマンド構造定義：get, list, status, config, auth, watch
│   ├── api.rs                 reqwest::blockingを使用したEdge Worker REST APIクライアント
│   ├── config.rs              %APPDATA%\rdl\config.json の読み書きおよびWindows DPAPI暗号化処理
│   └── error.rs               thiserrorクレートによる統一的エラーハンドリング定義
├── tests/
│   └── integration.rs         assert_cmdを使用したCLI結合テスト (--version, 設定往復, 異常系処理)
└── .github/workflows/
    ├── build.yml              CI: cargo build --release によるバイナリ生成とx64アーティファクト保存
    ├── test.yml               CI: cargo fmt, cargo clippy, cargo test --all-targets の厳格検証
    ├── download.yml           バックエンド自動化ワークフロー：URLからOneDriveへの直結ストリーム (curl | rclone rcat)
    └── download-hls.yml       HLS / ストリーミング動画自動化バリアント (yt-dlp | rclone rcat)
```

</details>

---

## 💻 互換性と動作環境

<details open>
<summary><b>💻 互換性と動作環境</b></summary>
<br>

| 項目 | サポート状況 |
|---|---|
| 対応OS（クライアントバイナリ） | Windows 10 1809以降 および Windows 11 |
| アーキテクチャ | x86_64 のみ |
| Rust ツールチェーン | 1.75+（`rust-toolchain.toml` により安定版に固定） |
| リンカー | MSVC（Visual Studio Build Tools 2022） |
| クラウドストレージ | 個人用または法人用 Microsoft OneDrive（rclone 経由） |
| Telegram クライアント | Webhook Bot をサポートするすべての公式およびサードパーティ製アプリ |

提供されるクライアントバイナリはMSVCランタイムに静的リンクされているため、Windowsの標準システムDLL（`KERNEL32.DLL`, `USER32.DLL`, `ADVAPI32.DLL`, `CRYPT32.DLL`, `WS2_32.DLL`）以外の追加DLL依存関係は一切ありません。

</details>

---

## 🔒 セキュリティと設計原則

<details open>
<summary><b>🔒 セキュリティと設計原則</b></summary>
<br>

- **保存時トークンの暗号化（Token at rest）** —— `%APPDATA%\rdl\config.json` に保存されるベアラートークンは、Windows DPAPI（`CryptProtectData`）によって強力に暗号化されます。このトークンを復号できるのは、暗号化した同一のWindowsユーザーセッションのみです。
- **通信経路の完全保護（Token in transit）** —— `reqwest` クライアントは `https_only=true` でビルドされており、トークンは常にTLSで保護された `Authorization` ヘッダー経由で送信され、URLパラメータに平文で露出することはありません。
- **認証アタックサーフェスの最小化** —— Botの操作権限は単一のTelegramユーザー（`TG_OWNER_ID`）のみに制限され、Webhookの受信にはTelegramとの間でのみ共有されるシークレットURLパスが要求されます。
- **機密情報のログマスキング** —— GitHub Actionsワークフローに渡されるダウンロードURLおよびファイル名は、すべて `::add-mask::` によって完全に保護され、ビルドログへの露出を防止します。
- **OAuthトークンの自動更新・自己修復** —— ワークフローが成功するたびにOneDriveのリフレッシュトークンが自動更新され、リポジトリの `RCLONE_CONF` Secretに書き戻されるため、数ヶ月間の放置でも期限切れになりません。
- **ゼロテレメトリーの徹底** —— CLIクライアントおよびバックエンドの双方において、外部サーバーへの利用状況追跡やテレメトリ送信は一切行いません。

</details>

---

## 🛠️ トラブルシューティング マトリクス

<details open>
<summary><b>🛠️ トラブルシューティング マトリクス</b></summary>
<br>

| 症状 | 主な原因 | 推奨される解決策 |
|---|---|---|
| `worker URL not set` | このマシンで初めて起動し、初期設定が未完了 | `rdl config set worker https://<Worker名>.workers.dev` を実行 |
| `not authenticated — run rdl auth login` | ローカルに有効なトークンが存在しない | Telegram Botから永続トークンを発行し、`rdl auth login --token <トークン>` を実行 |
| `server rejected credentials (401)` | トークンが失効したか手動で無効化された | Telegram Botから新しいトークンを再発行してログインし直す |
| `rdl status <id>` で `not found` が返る | ジョブIDの指定ミス、または古いログが削除済み | `rdl list` を実行して現在有効なジョブIDを確認 |
| ワークフローの OneDrive アップロードで失敗する | 90日以上アクセスがなくOneDriveトークンが期限切れ | デスクトップで `rclone config reconnect onedrive:` を実行後、`RCLONE_CONF` Secretを更新 |
| Telegram Bot から応答が返らない | Webhook シークレットの不一致または Worker スクリプトのエラー | `https://api.telegram.org/bot<TOKEN>/getWebhookInfo` で状態を確認し、Workerの実行ログを点検 |

</details>

---

## 📦 インストールとデプロイ

<details open>
<summary><b>📦 インストールとデプロイ</b></summary>
<br>

### プリビルドバイナリの導入 (Windows x64)

1. 公式 [Releases ページ](https://github.com/GHSFS/remote-dl/releases) から最新の `rdl-x64.exe` をダウンロードします。
2. システムの環境変数 `PATH` に含まれるディレクトリ（例: `C:\Tools\`）に配置します。
3. 動作確認：コマンドプロンプトやPowerShellで `rdl --version` を実行します。

### ソースコードからのビルド

Rust 1.75以上および Visual Studio MSVC ビルドツールが必要です。

```bash
git clone https://github.com/GHSFS/remote-dl.git
cd remote-dl
cargo build --release
# 生成先: target/release/rdl.exe
```

</details>

---

## 🚀 クイックスタートと CLI コマンドリファレンス

<details open>
<summary><b>🚀 クイックスタートと CLI コマンドリファレンス</b></summary>
<br>

```bash
# 1. クライアントの接続先バックエンドを設定
rdl config set worker https://your-worker.example.workers.dev

# 2. 認証の完了（初回のみ；事前にTelegram Botから永続トークンを取得）
rdl auth login --token <permanent-token>

# 3. ダウンロードタスクをキューに投入
rdl https://example.com/dataset.tar.zst

# 4. 進行状況とステータスの確認
rdl list
rdl status <job-id>
```

### CLI コマンド仕様詳細

```text
USAGE:
    rdl [OPTIONS] <SUBCOMMAND>

SUBCOMMANDS:
    get, g <url>                URLをダウンロードキューに登録 (--name, --folder, --json)
    list, ls                    直近のダウンロード履歴と進行状況を表示 (--limit, --json)
    status, st [job-id]         特定タスクのリアルタイム詳細ステータスを表示 (--json)
    inspect <url>               転送前の事前検証：レスポンスヘッダー、サイズ、Range分割対応の確認 (--json)
    batch <file>                テキストファイルからURLを一括読み込み・登録 (--folder, --json)
    ping                        Edge Workerとの疎通および認証の有効性をテスト (--json)
    config <get|set|path|show>  クライアント設定ファイルの読み取り・設定・パス確認・診断
    auth <login|logout>         Windows DPAPIハードウェア暗号化による永続認証トークンの管理
    watch                       クリップボード監視モードで常駐起動（rdl-tray トレイ常駐連携）
    help                        ヘルプメッセージの表示

OPTIONS:
    -j, --json                  機械可読な JSON 形式で出力
    -h, --help                  ヘルプメッセージの表示
    -V, --version               バージョン情報の表示
```

</details>

---

## ⚙️ 詳細設定とバックエンド環境構築

<details open>
<summary><b>⚙️ 詳細設定とバックエンド環境構築</b></summary>
<br>

### クライアント設定の仕様

設定ファイルは標準パス `%APPDATA%\rdl\config.json` に格納されます。保存されるトークンはWindows DPAPIによって保護されているため、別ユーザーや不正プロセスによる盗聴を防ぎます。

| 設定項目 | 内容説明 |
|---|---|
| `worker` | デプロイした Cloudflare Edge Worker のベースURL |
| `token` | 永続認証トークン（DPAPI により暗号化保護） |
| `folder` | OneDrive 上のデフォルト保存先フォルダー名 |

### バックエンド自動化パイプラインの構築

バックエンドの自動化処理は GitHub Actions ワークフロー（[`download.yml`](./.github/workflows/download.yml) および [`download-hls.yml`](./.github/workflows/download-hls.yml)）を利用して動作します：

1. ローカルマシンで `rclone config` を実行して OneDrive との連携設定を行い、生成された `rclone.conf` の全文を GitHub リポジトリの `RCLONE_CONF` Secret に登録します。
2. `repo` および `workflow` スコープを持つ GitHub Personal Access Token (PAT) を作成し、リポジトリの `GH_PAT` Secret に登録します（OAuthトークンの自動更新時に使用）。
3. Cloudflare Edge Worker をデプロイしてリポジトリの `workflow_dispatch` イベントを発火できるように設定し、ローカルで `rdl config set worker <WorkerURL>` を設定して接続します。

</details>

---

## ❓ よくある質問 (FAQ)

<details open>
<summary><b>❓ よくある質問 (FAQ)</b></summary>
<br>

**Q. ローカルで `wget` や `curl`、`aria2c` を実行するのと何が違いますか？**
A. ローカル環境での実行は、数十〜数百GBに及ぶデータが自宅の通信回線とPCのストレージを経由するため、ネットワーク帯域とディスク寿命の大きなボトルネックとなります。`remote-dl` は、100GBのファイル転送であってもローカルディスク消費0バイト、自宅ネットワーク帯域消費0バイトで完遂します。

**Q. 1つのバックエンドインスタンスを複数人で共有できますか？**
A. いいえ、本ツールは単一ユーザー専用として設計されています。マルチテナント化はクォータ制限、認証分離、不正利用対策などの複雑性を大幅に増大させるため、完全なシングルユーザー設計を維持しています。

**Q. ダウンロードの中断再開（レジューム）には対応していますか？**
A. はい、対応しています。OneDrive へのアップロード処理はチャンク分割ストリーミングで行われ、アップロード側での自動再開が完全にサポートされています。ソースURLからのダウンロードはジョブごとに1回実行され、万が一失敗した場合は再登録が可能です。

</details>

---

## 🤝 コントリビューションと謝辞

<details open>
<summary><b>🤝 コントリビューションと謝辞</b></summary>
<br>

Pull Request (PR) を作成する前に、以下の3つの検証コマンドをローカルでパスすることを確認してください：

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

### 謝辞・主要ライブラリ

- [`clap`](https://crates.io/crates/clap) — derive マクロによる高速かつ堅牢な CLI 引数解析
- [`reqwest`](https://crates.io/crates/reqwest) + [`rustls`](https://crates.io/crates/rustls) — モダン TLS 専用の高性能 HTTP クライアント
- [`windows`](https://crates.io/crates/windows) — Microsoft 公式 Win32 バインディング (DPAPI)
- [`directories`](https://crates.io/crates/directories) — OS標準に準拠した設定ディレクトリの自動解決
- [`rclone`](https://rclone.org) — GitHub Actions クラウド環境内での強力なチャンク分割アップロードエンジン

</details>

---

## 📄 ライセンスと免責事項

<details open>
<summary><b>📄 ライセンスと免責事項</b></summary>
<br>

本ソフトウェアは **MIT License** の下で公開されています（詳細は [LICENSE](../../LICENSE) を参照）。

### 免責事項 (Disclaimer)

本ツールは、利用者が正当な所有権を有しているコンテンツ、または法的にダウンロードを許可されているコンテンツを個人利用目的でアーカイブ（保存）することを前提として作成されています。転送元Webサイトの利用規約および適用される著作権法を遵守する責任は、すべて本ツールの運用者にあります。**本ツールを使用した無断再配布、ミラーリング、プロキシ運用、または第三者への不正共有行為を固く禁じます。**

</details>
