<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <strong>🇹🇷 Türkçe</strong> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# remote-dl

> Buluta doğrudan akışlı indirme yöneticisi — URL'leri yerel diskinizde depolamadan ve ev internetinizi yormadan doğrudan kişisel bulut depolama alanınıza aktarın.

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#-kurulum-ve-dağıtım)

</div>

---

## 🛡️ Projeye Genel Bakış ve Temel Yetenekler

<details open>
<summary><b>🛡️ Projeye Genel Bakış ve Temel Yetenekler</b></summary>
<br>

`remote-dl`, büyük dosyaları (yüksek çözünürlüklü videolar, arşivler, veri kümeleri, araştırma kaynakları) yerel bilgisayar diski üzerinden aktarmak zorunda kalmadan **doğrudan kendi bulut depolama alanına** aktarmak isteyen operatörler için tasarlanmış kişisel bir indirme yöneticisidir.

Masaüstünüzdeki hafif yerel CLI istemcisi (`rdl.exe`), sunucusuz (serverless) bir arka uç ile koordineli çalışır. Gerçek veri bayt aktarımı tamamen bulut altyapısı içinde gerçekleştiğinden, ne yerel diskiniz ne de ev internet bağlantınız veri yolunda kullanılmaz.

Bu araç **tek kullanıcılı ve bağımsız barındırılan (self-hosted)** bir sistemdir. Kendi bulut depolama hesabınızı bağlarsınız; hiçbir veri veya kaynak üçüncü şahıslarla paylaşılmaz.

</details>

---

## ⚡ Temel Özellikler

<details open>
<summary><b>⚡ Temel Özellikler</b></summary>
<br>

- **Yüksek bellek verimli doğrudan akış** — her boyuttaki dosya kesintisiz bir akış hattı üzerinden aktarılır; ne istemcide ne de sunucuda diske geçici yazma yapılmaz.
- **Yerel OneDrive entegrasyonu** — Microsoft Graph API üzerinden parçalı (chunked) ve otomatik sürdürülebilir yükleme desteği.
- **Güçlü tek kullanıcılı kimlik doğrulama** — tek kullanımlık şifreler (OTP) ve iptal edilebilir kalıcı anahtarlar; karmaşık hesap sistemlerine veya harici kimlik sağlayıcılarına ihtiyaç duymaz.
- **Çoklu birleşik arabirimler** — masaüstü CLI, mobil uyumlu Web arayüzü ve özel Telegram botu aynı arka ucu sorunsuz şekilde paylaşır.
- **Katı güven sınırı yalıtımı** — kimlik bilgileri ait oldukları katmandan asla dışarı çıkmaz (operatör → Edge Worker → otomasyon çalıştırıcı → depolama sağlayıcısı).
- **Otomatik token rotasyonu** — OAuth yenileme anahtarları her başarılı aktarımda otomatik olarak yenilenir ve şifreli olarak güncellenir; böylece sistem aylarca kullanılmasa bile erişimini kaybetmez.

</details>

---

## 🏛️ Sistem Mimarisi ve Topoloji

<details open>
<summary><b>🏛️ Sistem Mimarisi ve Topoloji</b></summary>
<br>

```text
  ┌──────────────────────┐
  │  rdl.exe CLI         │
  │  Web Arayüzü         │ ──HTTPS──┐
  │  Telegram Botu       │          │
  └──────────────────────┘          ▼
                               ┌──────────────────────┐
                               │  Edge Worker         │
                               │  (Kimlik + Dağıtım)  │
                               └──────────┬───────────┘
                                          │ workflow_dispatch
                                          ▼
                               ┌──────────────────────┐
                               │  Bulut Çalıştırıcı   │
                               │  (Akış Aracısı)      │
                               └──────────┬───────────┘
                                          │ Parçalı Yükleme
                                          ▼
                               ┌──────────────────────┐
                               │  Kişisel OneDrive    │
                               └──────────────────────┘
```

</details>

---

## 📂 Depo Dizin Yapısı

<details open>
<summary><b>📂 Depo Dizin Yapısı</b></summary>
<br>

```text
remote-dl/
├── Cargo.toml                 Paket bildirimi: rdl ikilisini, bağımlılıkları ve boyut optimizasyonlu sürüm profilini tanımlar
├── Cargo.lock                 Tüm geçişli bağımlılıkların sabitlenmiş tam sürümleri
├── rust-toolchain.toml        Rust derleyici araç zinciri sabitlemesi (stable, x86_64-pc-windows-msvc)
├── .cargo/
│   └── config.toml            Derleme hedefi ve rustflags parametreleri (statik MSVC CRT bağlama)
├── .gitignore                 target/ dizini, gizli anahtarlar, IDE yapılandırmaları ve taslakları hariç tutar
├── README.md                  İngilizce ana dokümantasyon
├── LICENSE                    MIT açık kaynak lisansı
├── CHANGELOG.md               Keep-a-Changelog standardında sürüm değişiklik günlüğü
├── docs/
│   └── locales/               Yerelleştirilmiş çok dilli belgeler (ko, zh-CN, ja, es, ru, vi, tr, de, pt-BR)
├── src/
│   ├── main.rs                CLI giriş noktası: clap argüman analizi, komut yönlendirme ve renkli konsol çıktısı
│   ├── cli.rs                 clap derive komut tanımları: get, list, status, config, auth, watch
│   ├── api.rs                 Edge Worker ile iletişim için reqwest::blocking tabanlı REST API istemcisi
│   ├── config.rs              %APPDATA%\rdl\config.json dosyasının Windows DPAPI ile şifreli okunması ve yazılması
│   └── error.rs               thiserror kütüphanesi ile merkezi ve birleşik hata yönetimi
├── tests/
│   └── integration.rs         assert_cmd ile CLI entegrasyon testleri (--version, yapılandırma döngüsü, hata durumları)
└── .github/workflows/
    ├── build.yml              CI: cargo build --release çalıştırır ve x64 ikili yapı çıktısını depolar
    ├── test.yml               CI: cargo fmt, cargo clippy ve cargo test --all-targets kontrollerini yürütür
    ├── download.yml           Bulut otomasyonu: URL'den OneDrive'a doğrudan akış hattı (curl | rclone rcat)
    └── download-hls.yml       HLS akış videoları için özel otomasyon hattı (yt-dlp | rclone rcat)
```

</details>

---

## 💻 Uyumluluk ve Çalışma Zamanı Gereksinimleri

<details open>
<summary><b>💻 Uyumluluk ve Çalışma Zamanı Gereksinimleri</b></summary>
<br>

| Boyut | Destek Kapsamı |
|---|---|
| İşletim sistemi (istemci ikilisi) | Windows 10 (Sürüm 1809+) ve Windows 11 |
| İşlemci mimarisi | Yalnızca x86_64 |
| Rust araç zinciri | 1.75+ (`rust-toolchain.toml` ile stabil sürüme sabitlenmiştir) |
| Bağlayıcı (Linker) | MSVC (Visual Studio Build Tools 2022) |
| Bulut depolama | Microsoft OneDrive (Bireysel veya Kurumsal) rclone aracılığıyla |
| Telegram istemcisi | Webhook destekli botları çalıştırabilen tüm resmi ve üçüncü taraf istemciler |

Dağıtılan istemci ikili dosyası MSVC çalışma zamanına statik olarak bağlanmıştır; bu sayede standart Windows sistem DLL'leri (`KERNEL32.DLL`, `USER32.DLL`, `ADVAPI32.DLL`, `CRYPT32.DLL`, `WS2_32.DLL`) haricinde hiçbir ek DLL bağımlılığı bulunmaz.

</details>

---

## 🔒 Güvenlik İlkeleri ve Değişmezler

<details open>
<summary><b>🔒 Güvenlik İlkeleri ve Değişmezler</b></summary>
<br>

- **Bekleme anında şifreleme (Token at rest)** — `%APPDATA%\rdl\config.json` dosyasındaki erişim anahtarı Windows DPAPI (`CryptProtectData`) ile şifrelenir. Yalnızca bu anahtarı oluşturan Windows kullanıcı oturumu tarafından çözülebilir.
- **İletim anında şifreleme (Token in transit)** — `reqwest` istemcisi `https_only=true` ile derlenir; anahtarlar yalnızca TLS korumalı `Authorization` başlığında iletilir, asla URL parametrelerinde yer almaz.
- **Kısıtlı yetki yüzeyi** — Yalnızca tanımlı Telegram kullanıcısı (`TG_OWNER_ID`) bota komut gönderebilir; Webhook çağrıları yalnızca Telegram ile paylaşılan gizli bir URL yolu gerektirir.
- **Günlüklerde hassas veri maskeleme** — GitHub Actions iş akışlarına iletilen her URL ve dosya adı `::add-mask::` ile gizlenir; böylece herkese açık derleme günlüklerinde görünmesi engellenir.
- **Kendini yenileyen OAuth anahtar döngüsü** — Her başarılı aktarımın ardından OneDrive yenileme anahtarı otomatik olarak yenilenir ve depodaki `RCLONE_CONF` gizli anahtarına yazılır.
- **Sıfır telemetri** — Ne CLI istemcisi ne de bulut arka uç servisleri üçüncü taraf sunuculara herhangi bir kullanım verisi veya günlük göndermez.

</details>

---

## 🛠️ Sorun Giderme Matrisi

<details open>
<summary><b>🛠️ Sorun Giderme Matrisi</b></summary>
<br>

| Belirti | Olası Neden | Çözüm Adımı |
|---|---|---|
| `worker URL not set` | Bu makinede ilk kez çalıştırılıyor ve yapılandırma yapılmamış | `rdl config set worker https://<worker-adiniz>.workers.dev` komutunu çalıştırın |
| `not authenticated — run rdl auth login` | Yerel önbellekte geçerli bir anahtar yok | Telegram botundan kalıcı anahtar oluşturun ve `rdl auth login --token <anahtar>` çalıştırın |
| `server rejected credentials (401)` | Anahtar iptal edilmiş veya süresi dolmuş | Telegram botundan yeni bir anahtar talep edip tekrar giriş yapın |
| `rdl status <id>` çıktısı `not found` veriyor | Hatalı görev kimliği girilmiş veya günlükler temizlenmiş | Aktif görevleri ve kimlikleri listelemek için `rdl list` komutunu çalıştırın |
| İş akışı OneDrive adımında başarısız oluyor | Hareketsizlik sebebiyle OneDrive anahtarının süresi dolmuş (>90 gün) | Bilgisayarınızda `rclone config reconnect onedrive:` çalıştırıp `RCLONE_CONF` gizli anahtarını güncelleyin |
| Telegram botu komutlara yanıt vermiyor | Webhook gizli anahtarı uyuşmazlığı veya Worker betik hatası | `https://api.telegram.org/bot<TOKEN>/getWebhookInfo` ile durumu inceleyin; Worker günlüklerini kontrol edin |

</details>

---

## 📦 Kurulum ve Dağıtım

<details open>
<summary><b>📦 Kurulum ve Dağıtım</b></summary>
<br>

### Önceden derlenmiş ikili kurulumu (Windows x64)

1. [Resmi Sürümler sayfasından](https://github.com/GHSFS/remote-dl/releases) `rdl-x64.exe` dosyasını indirin.
2. Dosyayı sistem ortam değişkeniniz olan `PATH` içinde yer alan bir dizine yerleştirin (örneğin `C:\Tools\`).
3. Terminalde doğrulayın: `rdl --version`.

### Kaynak koddan derleme

Rust 1.75+ ve Visual Studio MSVC derleme araçları gerektirir.

```bash
git clone https://github.com/GHSFS/remote-dl.git
cd remote-dl
cargo build --release
# Çıktı ikili dosyası: target/release/rdl.exe
```

</details>

---

## 🚀 Hızlı Başlangıç ve CLI Komut Referansı

<details open>
<summary><b>🚀 Hızlı Başlangıç ve CLI Komut Referansı</b></summary>
<br>

```bash
# 1. İstemcinizi arka uç servisinize bağlayın
rdl config set worker https://your-worker.example.workers.dev

# 2. Kimlik doğrulamasını tamamlayın (tek seferlik; anahtarı bottan alın)
rdl auth login --token <permanent-token>

# 3. İndirme bağlantısını bulut kuyruğuna ekleyin
rdl https://example.com/dataset.tar.zst

# 4. İlerlemeyi ve durumu takip edin
rdl list
rdl status <job-id>
```

### Detaylı CLI Komut Belirtimleri

```text
USAGE:
    rdl [OPTIONS] <SUBCOMMAND>

SUBCOMMANDS:
    get, g <url>                Bir URL'yi bulut indirme kuyruğuna ekler (--name, --folder, --json)
    list, ls                    Son indirmeleri ve ilerlemelerini listeler (--limit, --json)
    status, st [job-id]         Belirli bir görevin anlık detaylı durumunu gösterir (--json)
    inspect <url>               Ön kontrol: HTTP başlıkları, dosya boyutu ve Range desteği (--json)
    batch <file>                Bir metin dosyasındaki URL'leri toplu olarak kuyruğa ekler (--folder, --json)
    ping                        Edge Worker ile bağlantıyı ve kimlik doğrulamasını test eder (--json)
    config <get|set|path|show>  İstemci yapılandırma dosyasını okur, yazar, konumlandırır ve tanılar
    auth <login|logout>         Windows DPAPI ile korunan kalıcı kimlik anahtarını yönetir
    watch                       Pano izleme modunda arka planda çalışır (rdl-tray eşlikçi modu)
    help                        Yardım mesajını görüntüler

OPTIONS:
    -j, --json                  Makine tarafından okunabilir JSON biçiminde çıktı verir
    -h, --help                  Yardım mesajını görüntüler
    -V, --version               Sürüm bilgilerini görüntüler
```

</details>

---

## ⚙️ Yapılandırma ve Arka Uç Kurulumu

<details open>
<summary><b>⚙️ Yapılandırma ve Arka Uç Kurulumu</b></summary>
<br>

### İstemci Yapılandırması

Yapılandırma dosyası `%APPDATA%\rdl\config.json` altında tutulur. Hassas veriler, diğer kullanıcıların veya kötü niyetli işlemlerin erişimini engellemek için Windows DPAPI ile şifrelenir.

| Anahtar | Açıklama |
|---|---|
| `worker` | Dağıtılan Cloudflare Edge Worker temel URL adresi |
| `token` | Kalıcı kimlik doğrulama anahtarı (DPAPI ile şifrelenmiştir) |
| `folder` | OneDrive üzerindeki varsayılan hedef klasör adı |

### Arka Uç Otomasyonunun Kurulumu

Bulut işlem hattı GitHub Actions iş akışları üzerinden yürütülür (bkz. [`download.yml`](./.github/workflows/download.yml) ve [`download-hls.yml`](./.github/workflows/download-hls.yml)):

1. Yerel bilgisayarınızda OneDrive için `rclone config` kurulumunu tamamlayın ve oluşturulan `rclone.conf` dosyasının tüm içeriğini `RCLONE_CONF` adıyla depo gizli anahtarlarına ekleyin.
2. `repo` ve `workflow` izinlerine sahip bir GitHub Personal Access Token (PAT) oluşturun ve bunu `GH_PAT` adıyla depoya kaydedin (yenilenen OAuth anahtarlarını kaydetmek için kullanılır).
3. Depoda `workflow_dispatch` olaylarını tetikleyecek Cloudflare Edge Worker kodunu dağıtın ve istemcinizi `rdl config set worker <Worker-URL>` komutuyla bağlayın.

</details>

---

## ❓ Sıkça Sorulan Sorular (FAQ)

<details open>
<summary><b>❓ Sıkça Sorulan Sorular (FAQ)</b></summary>
<br>

**S: Neden yerel bilgisayarda `wget`, `curl` veya `aria2c` kullanmak yerine bu araç tercih edilmeli?**
C: Yerel araçlar onlarca veya yüzlerce gigabaytlık verinin ev internetinizden geçmesini ve bilgisayarınızın diskine yazılmasını gerektirir. `remote-dl`, 100 GB'lık bir dosya aktarımını yerel diskinizden 0 bayt ve ev internetinizden 0 bayt tüketerek tamamlamanızı sağlar.

**S: Birden fazla kullanıcı aynı arka uç örneğini paylaşabilir mi?**
C: Hayır. Araç özellikle tek kullanıcılı kişisel kullanım için tasarlanmıştır. Çok kullanıcılı mimari kota yönetimi, kullanıcı yalıtımı ve kötüye kullanım denetimi gibi karmaşıklıklar getirir ve projenin hafiflik felsefesiyle çelişir.

**S: Kesilen indirmelerin devam ettirilmesi destekleniyor mu?**
C: Evet. OneDrive'a yükleme işlemi parçalı akış olarak yürütülür ve yükleme tarafında otomatik devam etmeyi tam olarak destekler. Kaynak URL'den indirme işlemi her görev için tek seferde yürütülür; kaynakta kopma olursa görevi yeniden kuyruğa eklemeniz yeterlidir.

</details>

---

## 🤝 Katkıda Bulunma ve Teşekkürler

<details open>
<summary><b>🤝 Katkıda Bulunma ve Teşekkürler</b></summary>
<br>

Bir Pull Request (PR) göndermeden önce aşağıdaki üç kalite kontrol adımının yerel olarak başarıyla geçtiğinden emin olun:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

### Teşekkür Edilen Temel Kütüphaneler

- [`clap`](https://crates.io/crates/clap) — derive makroları ile yüksek performanslı CLI argüman ayrıştırma
- [`reqwest`](https://crates.io/crates/reqwest) + [`rustls`](https://crates.io/crates/rustls) — modern TLS odaklı güvenli HTTP istemcisi
- [`windows`](https://crates.io/crates/windows) — resmi Microsoft Win32 API bağlayıcıları (DPAPI)
- [`directories`](https://crates.io/crates/directories) — işletim sistemi standartlarına uygun yapılandırma dizini tespiti
- [`rclone`](https://rclone.org) — bulut ortamlarında güçlü ve güvenilir parçalı yükleme motoru

</details>

---

## 📄 Lisans ve Yasal Uyarı

<details open>
<summary><b>📄 Lisans ve Yasal Uyarı</b></summary>
<br>

Bu proje **MIT Lisansı** kapsamında sunulmaktadır (bkz. [LICENSE](../../LICENSE)).

### Yasal Uyarı (Disclaimer)

Bu araç, yalnızca operatörün yasal mülkiyetinde olan veya açıkça indirme iznine sahip olduğu içeriklerin kişisel arşivleme amacıyla indirilmesi için geliştirilmiştir. Kaynak web sitelerinin hizmet şartlarına ve yürürlükteki telif hakkı mevzuatına uyma sorumluluğu tamamen operatöre aittir. **Bu aracın izinsiz yeniden dağıtım, genel ayna sunucu, ticari proxy veya üçüncü şahıslarla paylaşım amacıyla kullanılması kesinlikle yasaktır.**

</details>
