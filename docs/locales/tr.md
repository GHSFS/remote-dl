<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <strong>🇹🇷 Türkçe</strong> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# remote-dl

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

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

### Proje yapısı

```
remote-dl/
├── Cargo.toml             Paket manifesti, boyut için optimize edilmiş release
├── rust-toolchain.toml    Toolchain sabitleme (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml     Derleme hedefi + statik MSVC CRT
├── README.md / LICENSE / CHANGELOG.md
├── src/
│   ├── main.rs            CLI giriş noktası, clap dispatcher, renkli çıktı
│   ├── cli.rs             clap tanımları: get / list / status / config / auth / watch
│   ├── api.rs             reqwest::blocking istemcisi (worker API sarmalı)
│   ├── config.rs          config.json okuma/yazma + DPAPI ile token koruması
│   └── error.rs           Crate genelinde hata türü
├── tests/integration.rs   assert_cmd tabanlı CLI testleri
└── .github/workflows/
    ├── build.yml          rdl.exe release build CI
    ├── test.yml           fmt / clippy / test CI
    ├── download.yml       Arka uç otomasyonu: URL → OneDrive streaming
    └── download-hls.yml   HLS / streaming varyantı (yt-dlp + rclone rcat)
```

Ayrıntılı kurulum, hızlı başlangıç, CLI referansı ve SSS için [English](#english)
bölümüne bakın.

### Lisans

MIT. [LICENSE](./LICENSE) dosyasına bakın.
