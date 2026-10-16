<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <strong>🇯🇵 日本語</strong> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# remote-dl

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

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

### プロジェクト構成

```
remote-dl/
├── Cargo.toml             パッケージマニフェスト、サイズ最適化リリースプロファイル
├── rust-toolchain.toml    ツールチェイン固定 (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml     ビルドターゲット + 静的 MSVC CRT
├── README.md / LICENSE / CHANGELOG.md
├── src/
│   ├── main.rs            CLI エントリポイント、clap ディスパッチャ、カラー出力
│   ├── cli.rs             clap 定義: get / list / status / config / auth / watch
│   ├── api.rs             reqwest::blocking クライアント (ワーカー API ラッパー)
│   ├── config.rs          config.json 読み書き + DPAPI トークン保護
│   └── error.rs           クレート全体のエラー型
├── tests/integration.rs   assert_cmd ベースの CLI テスト
└── .github/workflows/
    ├── build.yml          rdl.exe リリースビルド CI
    ├── test.yml           fmt / clippy / test CI
    ├── download.yml       バックエンド自動化: URL → OneDrive ストリーミング
    └── download-hls.yml   HLS / ストリーミング派生 (yt-dlp + rclone rcat)
```

詳細なインストール、クイックスタート、CLI リファレンス、FAQ は
[English](#english) セクションを参照してください。

### ライセンス

MIT。[LICENSE](./LICENSE) を参照。
