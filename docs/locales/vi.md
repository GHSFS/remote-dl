<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <strong>🇻🇳 Tiếng Việt</strong> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# remote-dl

> Trình quản lý tải xuống truyền phát trực tiếp lên đám mây — chuyển hướng URL trực tiếp vào bộ nhớ đám mây cá nhân mà không lưu tạm trên đĩa cục bộ hay tiêu tốn băng thông gia đình.

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#-cài-đặt-và-triển-khai)

</div>

---

## 🛡️ Tổng quan dự án và Khả năng cốt lõi

<details open>
<summary><b>🛡️ Tổng quan dự án và Khả năng cốt lõi</b></summary>
<br>

`remote-dl` là trình quản lý tải xuống cá nhân dành cho người vận hành cần lấy các tệp dung lượng lớn (video độ phân giải cao, tệp nén lưu trữ, tập dữ liệu nghiên cứu) **trực tiếp vào bộ nhớ đám mây riêng của họ** mà không phải trung chuyển hoặc lưu tạm qua ổ đĩa máy tính cục bộ.

Một ứng dụng CLI gốc nhỏ gọn (`rdl.exe`) trên máy tính để bàn sẽ điều phối tác vụ với một backend không máy chủ (Serverless) nhẹ. Quá trình truyền byte dữ liệu thực tế diễn ra hoàn toàn trong môi trường đám mây; do đó, dung lượng ổ đĩa cục bộ và kết nối mạng gia đình của bạn hoàn toàn không bị ảnh hưởng.

Đây là công cụ **tự lưu trữ dành riêng cho một người dùng (single-user)**. Bạn tự kết nối tài khoản đám mây của chính mình; không chia sẻ bất kỳ dữ liệu hay tài nguyên nào với người dùng khác.

</details>

---

## ⚡ Tính năng nổi bật

<details open>
<summary><b>⚡ Tính năng nổi bật</b></summary>
<br>

- **Truyền phát trực tiếp tiết kiệm bộ nhớ** — các tệp ở mọi kích thước đều lưu thông qua đường ống truyền phát liên tục; không ghi tạm thời lên đĩa ở cả phía máy khách và máy chủ.
- **Tương thích gốc với OneDrive** — tải lên theo từng phân đoạn (chunks) hỗ trợ tự động tiếp tục khi gặp sự cố thông qua Microsoft Graph API.
- **Xác thực an toàn cho người dùng đơn** — mật khẩu một lần (OTP) kết hợp với mã token vĩnh viễn có thể thu hồi; loại bỏ hệ thống tài khoản phức tạp và nhà cung cấp danh tính bên thứ ba.
- **Đa giao diện đồng bộ** — CLI máy tính để bàn, giao diện Web tối ưu cho thiết bị di động và bot Telegram riêng tư đều dùng chung một backend duy nhất.
- **Phân tách ranh giới tin cậy nghiêm ngặt** — thông tin xác thực không bao giờ rời khỏi tầng quản lý của chúng (người vận hành → Edge Worker → trình chạy tự động hóa → nhà cung cấp đám mây).
- **Tự động xoay vòng mã xác thực** — mã làm mới OAuth được tự động gia hạn và lưu trữ lại dưới dạng mã hóa sau mỗi lần truyền tệp thành công, đảm bảo hệ thống luôn sẵn sàng hoạt động ngay cả sau nhiều tháng không sử dụng.

</details>

---

## 🏛️ Kiến trúc và Sơ đồ cấu trúc hệ thống

<details open>
<summary><b>🏛️ Kiến trúc và Sơ đồ cấu trúc hệ thống</b></summary>
<br>

```text
  ┌──────────────────────┐
  │  rdl.exe (CLI)       │
  │  Giao diện Web       │ ──HTTPS──┐
  │  Bot Telegram        │          │
  └──────────────────────┘          ▼
                               ┌──────────────────────┐
                               │  Edge Worker         │
                               │  (Xác thực + Điều phối)
                               └──────────┬───────────┘
                                          │ workflow_dispatch
                                          ▼
                               ┌──────────────────────┐
                               │  Trình chạy đám mây  │
                               │  (Proxy truyền phát) │
                               └──────────┬───────────┘
                                          │ Tải lên phân đoạn
                                          ▼
                               ┌──────────────────────┐
                               │  OneDrive cá nhân    │
                               └──────────────────────┘
```

</details>

---

## 📂 Cấu trúc thư mục kho lưu trữ

<details open>
<summary><b>📂 Cấu trúc thư mục kho lưu trữ</b></summary>
<br>

```text
remote-dl/
├── Cargo.toml                 Tệp kê khai gói: định nghĩa tệp nhị phân rdl, các phụ thuộc và cấu hình bản phát hành tối ưu dung lượng
├── Cargo.lock                 Các phiên bản phụ thuộc bắc cầu được khóa chính xác
├── rust-toolchain.toml        Khóa phiên bản trình biên dịch Rust (stable, x86_64-pc-windows-msvc)
├── .cargo/
│   └── config.toml            Cấu hình mục tiêu xây dựng và tham số rustflags (liên kết tĩnh MSVC CRT)
├── .gitignore                 Loại trừ thư mục target/, bí mật, cấu hình IDE và các bản nháp
├── README.md                  Tài liệu chính bằng tiếng Anh
├── LICENSE                    Giấy phép mã nguồn mở MIT
├── CHANGELOG.md               Nhật ký thay đổi theo định dạng Keep-a-Changelog
├── docs/
│   └── locales/               Tài liệu được bản địa hóa (ko, zh-CN, ja, es, ru, vi, tr, de, pt-BR)
├── src/
│   ├── main.rs                Điểm vào CLI: phân tích tham số clap, điều phối lệnh và xuất màu giao diện
│   ├── cli.rs                 Định nghĩa các lệnh clap: get, list, status, config, auth, watch
│   ├── api.rs                 Máy khách REST API dùng reqwest::blocking để giao tiếp với Edge Worker
│   ├── config.rs              Đọc và ghi %APPDATA%\rdl\config.json với mã hóa phần cứng Windows DPAPI
│   └── error.rs               Enum và cấu trúc quản lý lỗi tập trung trên nền tảng thiserror
├── tests/
│   └── integration.rs         Kiểm thử tích hợp CLI với assert_cmd (--version, chu trình cấu hình, lỗi hệ thống)
└── .github/workflows/
    ├── build.yml              CI: chạy cargo build --release để xuất bản tệp nhị phân rdl cho kiến trúc x64
    ├── test.yml               CI: kiểm tra nghiêm ngặt với cargo fmt, cargo clippy và cargo test --all-targets
    ├── download.yml           Quy trình đám mây: đường ống trực tiếp từ URL sang OneDrive (curl | rclone rcat)
    └── download-hls.yml       Quy trình mở rộng cho video truyền phát HLS (yt-dlp | rclone rcat)
```

</details>

---

## 💻 Khả năng tương thích và Môi trường thực thi

<details open>
<summary><b>💻 Khả năng tương thích và Môi trường thực thi</b></summary>
<br>

| Hạng mục | Mức độ hỗ trợ |
|---|---|
| Hệ điều hành (tệp nhị phân máy khách) | Windows 10 (bản dựng 1809 trở lên) và Windows 11 |
| Kiến trúc bộ xử lý | Chỉ hỗ trợ x86_64 |
| Chuỗi công cụ Rust | 1.75+ (được ghim cố định bản stable trong `rust-toolchain.toml`) |
| Trình liên kết (Linker) | MSVC (Visual Studio Build Tools 2022) |
| Bộ nhớ đám mây | Microsoft OneDrive (Cá nhân hoặc Doanh nghiệp) thông qua rclone |
| Ứng dụng Telegram | Bất kỳ ứng dụng nào hỗ trợ bot hoạt động bằng Webhook |

Tệp thực thi nhị phân được phát hành được liên kết tĩnh với thư viện thời gian chạy của MSVC, do đó hoàn toàn không có phụ thuộc DLL bổ sung nào ngoài các thư viện hệ thống Windows tiêu chuẩn (`KERNEL32.DLL`, `USER32.DLL`, `ADVAPI32.DLL`, `CRYPT32.DLL`, `WS2_32.DLL`).

</details>

---

## 🔒 Xem xét bảo mật và Các nguyên tắc bất biến

<details open>
<summary><b>🔒 Xem xét bảo mật và Các nguyên tắc bất biến</b></summary>
<br>

- **Mã hóa khi lưu trữ (Token at rest)** — Mã xác thực lưu trong `%APPDATA%\rdl\config.json` được mã hóa an toàn bằng phần cứng Windows DPAPI (`CryptProtectData`). Chỉ tài khoản người dùng Windows tạo ra mã mới có quyền giải mã.
- **Bảo vệ trong quá trình truyền tải (Token in transit)** — Máy khách `reqwest` được thiết lập với `https_only=true`; mã token chỉ truyền qua giao thức mã hóa TLS trong tiêu đề `Authorization`, tuyệt đối không nằm trên tham số URL.
- **Thu hẹp bề mặt tấn công** — Chỉ duy nhất người dùng Telegram hợp lệ (`TG_OWNER_ID`) mới có quyền phát lệnh cho bot; luồng nhận Webhook yêu cầu khóa bí mật trong đường dẫn URL chỉ chia sẻ riêng với Telegram.
- **Che dấu thông tin nhạy cảm trong nhật ký** — Mọi liên kết URL và tên tệp gửi tới GitHub Actions đều được áp dụng lệnh `::add-mask::`, ngăn chặn rò rỉ dữ liệu trên nhật ký xây dựng công khai.
- **Tự động làm mới chứng chỉ OAuth** — Sau mỗi phiên tải hoàn tất, mã làm mới của OneDrive sẽ tự động xoay vòng và cập nhật vào bí mật kho lưu trữ `RCLONE_CONF`, duy trì tính hợp lệ dù ngừng hoạt động thời gian dài.
- **Không thu thập dữ liệu (Zero Telemetry)** — Cả máy khách CLI và hệ thống máy chủ phụ trợ đều không gửi bất kỳ dữ liệu chẩn đoán hay đo lường sử dụng nào về máy chủ của bên thứ ba.

</details>

---

## 🛠️ Bảng chẩn đoán và Xử lý sự cố

<details open>
<summary><b>🛠️ Bảng chẩn đoán và Xử lý sự cố</b></summary>
<br>

| Triệu chứng | Nguyên nhân tiềm ẩn | Hướng dẫn xử lý |
|---|---|---|
| `worker URL not set` | Lần đầu chạy trên máy tính này mà chưa thiết lập cấu hình | Chạy lệnh `rdl config set worker https://<worker-cua-ban>.workers.dev` |
| `not authenticated — run rdl auth login` | Chưa có token xác thực trong bộ nhớ cục bộ | Tạo mã vĩnh viễn qua bot Telegram rồi chạy `rdl auth login --token <ma-token>` |
| `server rejected credentials (401)` | Token đã bị thu hồi hoặc đã hết hiệu lực | Yêu cầu cấp mã mới từ bot Telegram và tiến hành đăng nhập lại |
| `rdl status <id>` báo lỗi `not found` | Sai ID tác vụ hoặc nhật ký tác vụ đã bị dọn dẹp | Chạy `rdl list` để kiểm tra danh sách các tác vụ đang hoạt động |
| Quy trình thất bại ở bước tải lên OneDrive | Token của OneDrive hết hạn do không hoạt động (>90 ngày) | Chạy `rclone config reconnect onedrive:` trên máy tính và cập nhật bí mật `RCLONE_CONF` |
| Bot Telegram ngừng phản hồi tin nhắn | Sai mã bí mật Webhook hoặc Worker bị lỗi mã nguồn | Truy cập `https://api.telegram.org/bot<TOKEN>/getWebhookInfo` để kiểm tra; xem nhật ký Worker |

</details>

---

## 📦 Cài đặt và Triển khai

<details open>
<summary><b>📦 Cài đặt và Triển khai</b></summary>
<br>

### Cài đặt tệp nhị phân dựng sẵn (Windows x64)

1. Tải về tệp thực thi `rdl-x64.exe` từ [trang phát hành chính thức (Releases)](https://github.com/GHSFS/remote-dl/releases).
2. Di chuyển tệp vào thư mục nằm trong biến môi trường `PATH` (ví dụ: `C:\Tools\`).
3. Kiểm tra cài đặt trong terminal: `rdl --version`.

### Biên dịch từ mã nguồn

Yêu cầu đã cài đặt Rust 1.75+ và bộ công cụ xây dựng Visual Studio MSVC.

```bash
git clone https://github.com/GHSFS/remote-dl.git
cd remote-dl
cargo build --release
# Tệp đầu ra: target/release/rdl.exe
```

</details>

---

## 🚀 Hướng dẫn nhanh và Tài liệu lệnh CLI

<details open>
<summary><b>🚀 Hướng dẫn nhanh và Tài liệu lệnh CLI</b></summary>
<br>

```bash
# 1. Trỏ máy khách đến dịch vụ backend của bạn
rdl config set worker https://your-worker.example.workers.dev

# 2. Xác thực tài khoản (chỉ cần làm một lần; lấy token từ bot trước)
rdl auth login --token <permanent-token>

# 3. Đưa URL vào hàng đợi tải xuống
rdl https://example.com/dataset.tar.zst

# 4. Kiểm tra tiến độ và trạng thái
rdl list
rdl status <job-id>
```

### Thông số kỹ thuật các lệnh CLI

```text
USAGE:
    rdl [OPTIONS] <SUBCOMMAND>

SUBCOMMANDS:
    get, g <url>                Thêm URL vào hàng đợi tải xuống (--name, --folder, --json)
    list, ls                    Hiển thị các tác vụ gần đây và tiến độ (--limit, --json)
    status, st [job-id]         Xem trạng thái chi tiết theo thời gian thực của tác vụ (--json)
    inspect <url>               Kiểm tra trước: tiêu đề HTTP, kích thước tệp và hỗ trợ Range (--json)
    batch <file>                Xếp hàng tải hàng loạt các liên kết từ tệp văn bản (--folder, --json)
    ping                        Kiểm tra kết nối và tính hợp lệ của token với Edge Worker (--json)
    config <get|set|path|show>  Đọc, ghi, định vị và chẩn đoán tệp cấu hình máy khách
    auth <login|logout>         Quản lý mã xác thực vĩnh viễn được bảo vệ bởi Windows DPAPI
    watch                       Chạy ở chế độ theo dõi bộ nhớ tạm (dành cho tiện ích rdl-tray)
    help                        Hiển thị trợ giúp

OPTIONS:
    -j, --json                  Xuất kết quả ở định dạng có cấu trúc JSON
    -h, --help                  Hiển thị trợ giúp
    -V, --version               Hiển thị thông tin phiên bản
```

</details>

---

## ⚙️ Cấu hình chi tiết và Thiết lập Backend

<details open>
<summary><b>⚙️ Cấu hình chi tiết và Thiết lập Backend</b></summary>
<br>

### Cấu hình máy khách cục bộ

Tệp cấu hình được đặt tại `%APPDATA%\rdl\config.json`. Dữ liệu nhạy cảm được bảo vệ bởi Windows DPAPI nhằm ngăn chặn hành vi truy cập trái phép từ tài khoản khác hoặc tiến trình độc hại.

| Khóa | Ý nghĩa |
|---|---|
| `worker` | Địa chỉ URL gốc của Cloudflare Edge Worker đã triển khai |
| `token` | Mã token xác thực vĩnh viễn (được mã hóa bởi DPAPI) |
| `folder` | Tên thư mục đích lưu trữ mặc định trên OneDrive |

### Thiết lập quy trình tự động hóa Backend

Quy trình đám mây hoạt động thông qua GitHub Actions (xem [`download.yml`](./.github/workflows/download.yml) và [`download-hls.yml`](./.github/workflows/download-hls.yml)):

1. Cấu hình `rclone` cục bộ cho OneDrive (`rclone config`) và dán toàn bộ nội dung tệp `rclone.conf` tạo được vào bí mật kho lưu trữ có tên `RCLONE_CONF`.
2. Tạo GitHub Personal Access Token (PAT) với quyền hạn `repo` và `workflow`, sau đó lưu vào bí mật `GH_PAT` (dùng để lưu tự động các token OAuth được làm mới).
3. Triển khai Cloudflare Worker để kích hoạt sự kiện `workflow_dispatch` trên kho lưu trữ, rồi kết nối trên máy tính bằng `rdl config set worker <dia-chi-worker>`.

</details>

---

## ❓ Câu hỏi thường gặp (FAQ)

<details open>
<summary><b>❓ Câu hỏi thường gặp (FAQ)</b></summary>
<br>

**H: Điểm khác biệt so với việc dùng `wget`, `curl` hay `aria2c` ngay trên máy cục bộ là gì?**
Đ: Các công cụ cục bộ bắt buộc toàn bộ dữ liệu (thường là hàng chục hay hàng trăm GB) phải đi qua đường truyền internet gia đình và ghi trực tiếp vào ổ đĩa máy tính của bạn. `remote-dl` giúp chuyển tải tệp 100 GB mà hoàn toàn tiêu tốn 0 byte ổ cứng và 0 byte băng thông mạng gia đình.

**H: Nhiều người dùng có thể dùng chung một máy chủ phụ trợ không?**
Đ: Không. Công cụ được thiết kế chuyên biệt cho một người dùng duy nhất. Việc hỗ trợ đa người dùng sẽ làm phát sinh độ phức tạp lớn về quản lý hạn ngạch, phân tách quyền và phòng chống lạm dụng, đi ngược lại tôn chỉ tinh gọn của dự án.

**H: Tác vụ có thể tự động tiếp tục khi bị gián đoạn không?**
Đ: Có. Quá trình truyền tải lên OneDrive được phân đoạn và hỗ trợ phục hồi tự động khi có sự cố mạng. Quá trình lấy tệp từ nguồn gốc là truyền thẳng một lần; nếu nguồn gốc gặp lỗi đột ngột, bạn chỉ cần đưa URL vào hàng đợi một lần nữa.

</details>

---

## 🤝 Đóng góp và Lời cảm ơn

<details open>
<summary><b>🤝 Đóng góp và Lời cảm ơn</b></summary>
<br>

Trước khi gửi Pull Request (PR), hãy đảm bảo dự án vượt qua cả 3 bài kiểm tra chất lượng sau:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

### Cảm ơn các thư viện nền tảng

- [`clap`](https://crates.io/crates/clap) — thư viện phân tích cú pháp tham số dòng lệnh cực nhanh bằng derive macro
- [`reqwest`](https://crates.io/crates/reqwest) + [`rustls`](https://crates.io/crates/rustls) — máy khách HTTP tập trung vào bảo mật TLS hiện đại
- [`windows`](https://crates.io/crates/windows) — các liên kết chính thức cho Microsoft Win32 API (DPAPI)
- [`directories`](https://crates.io/crates/directories) — tự động xác định các đường dẫn thư mục chuẩn theo hệ điều hành
- [`rclone`](https://rclone.org) — công cụ tải lên theo khối mạnh mẽ và tin cậy trên môi trường đám mây

</details>

---

## 📄 Giấy phép và Tuyên bố từ chối trách nhiệm

<details open>
<summary><b>📄 Giấy phép và Tuyên bố từ chối trách nhiệm</b></summary>
<br>

Dự án này được phân phối theo các điều khoản của **Giấy phép MIT** (xem [LICENSE](../../LICENSE)).

### Tuyên bố từ chối trách nhiệm (Disclaimer)

Công cụ này chỉ nhằm phục vụ mục đích lưu trữ cá nhân đối với các nội dung mà người vận hành sở hữu quyền hợp pháp hoặc đã được cấp phép tải xuống rõ ràng. Người dùng hoàn toàn chịu trách nhiệm tuân thủ các điều khoản dịch vụ của trang web nguồn và luật sở hữu trí tuệ hiện hành. **Nghiêm cấm sử dụng công cụ này vào mục đích tái phân phối trái phép, thiết lập máy chủ phản chiếu công cộng, proxy thương mại hoặc chia sẻ tệp bừa bãi cho bên thứ ba.**

</details>
