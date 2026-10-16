<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <strong>🇷🇺 Русский</strong> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# remote-dl

[![Build](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/remote-dl/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

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

### Структура проекта

```
remote-dl/
├── Cargo.toml             Манифест пакета, оптимизированный по размеру release
├── rust-toolchain.toml    Закрепление toolchain (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml     Цель сборки + статическая MSVC CRT
├── README.md / LICENSE / CHANGELOG.md
├── src/
│   ├── main.rs            Точка входа CLI, диспетчер clap, цветной вывод
│   ├── cli.rs             Определения clap: get / list / status / config / auth / watch
│   ├── api.rs             Клиент reqwest::blocking (обёртка над API воркера)
│   ├── config.rs          Чтение/запись config.json + защита токена через DPAPI
│   └── error.rs           Тип ошибки уровня крейта
├── tests/integration.rs   CLI-тесты на основе assert_cmd
└── .github/workflows/
    ├── build.yml          CI release-сборки rdl.exe
    ├── test.yml           CI fmt / clippy / test
    ├── download.yml       Бэкенд-автоматизация: URL → OneDrive потоково
    └── download-hls.yml   Вариант для HLS / стриминга (yt-dlp + rclone rcat)
```

Подробные инструкции по установке, быстрому старту, CLI-справочник и FAQ см.
в разделе [English](#english).

### Лицензия

MIT. См. [LICENSE](./LICENSE).
