<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <strong>🇻🇳 Tiếng Việt</strong> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# remote-dl

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

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

### Cấu trúc dự án

```
remote-dl/
├── Cargo.toml             Manifest gói, profile release tối ưu kích thước
├── rust-toolchain.toml    Cố định toolchain (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml     Mục tiêu build + MSVC CRT tĩnh
├── README.md / LICENSE / CHANGELOG.md
├── src/
│   ├── main.rs            Điểm vào CLI, dispatcher clap, output có màu
│   ├── cli.rs             Định nghĩa clap: get / list / status / config / auth / watch
│   ├── api.rs             Client reqwest::blocking (bọc API của worker)
│   ├── config.rs          Đọc/ghi config.json + bảo vệ token bằng DPAPI
│   └── error.rs           Kiểu lỗi của crate
├── tests/integration.rs   Test CLI dựa trên assert_cmd
└── .github/workflows/
    ├── build.yml          CI build release rdl.exe
    ├── test.yml           CI fmt / clippy / test
    ├── download.yml       Tự động hóa backend: URL → OneDrive streaming
    └── download-hls.yml   Biến thể HLS / streaming (yt-dlp + rclone rcat)
```

Hướng dẫn cài đặt chi tiết, khởi động nhanh, tham chiếu CLI và FAQ có ở phần
[English](#english).

### Giấy phép

MIT. Xem [LICENSE](./LICENSE).
