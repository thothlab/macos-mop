# mop

[English](README.en.md) | **Русский**

**mop** — быстрая консольная утилита для очистки macOS, написанная на Rust.

Освобождает место на диске, удаляя системный мусор, кэши, логи, остатки удалённых приложений и сборочные артефакты — прямо из терминала, без GUI и подписок.

За несколько секунд mop находит и показывает всё, что можно удалить. Вы видите список и размеры файлов до того, как что-либо будет удалено — случайных потерь быть не может.

---

## Возможности

### `mop clean` — Очистка системы

Сканирует macOS по 10 категориям и удаляет мусор, который накапливается незаметно:

| Категория | Что чистит |
|---|---|
| `user-cache` | Кэши приложений в `~/Library/Caches` |
| `system-logs` | Системные и пользовательские логи в `/var/log` и `~/Library/Logs` |
| `crash-reports` | Отчёты о сбоях в `~/Library/Logs/DiagnosticReports` |
| `browser-cache` | Кэши Chrome, Safari, Firefox, Arc, Brave, Edge |
| `app-leftovers` | Остатки удалённых приложений в Library (Preferences, Application Support, Containers) |
| `dev-tools` | Кэши Xcode DerivedData, CoreSimulator, npm, yarn, pip, cargo, CocoaPods, Gradle, Maven, Go |
| `homebrew` | Старые версии формул и кэш загрузок Homebrew |
| `trash` | Корзина macOS |
| `downloads` | Старые файлы в `~/Downloads` (старше заданного порога) |
| `docker` | Неиспользуемые образы, контейнеры, тома Docker |

```bash
# Посмотреть что будет удалено — без удаления
mop clean --dry-run --all

# Очистить всё
mop clean --all

# Очистить только выбранные категории
mop clean --category user-cache --category browser-cache --category trash

# Подробный вывод
mop clean --all --verbose
```

---

### `mop analyze` — Анализ дискового пространства

Находит, что занимает место на диске. Показывает топ папок и файлов с визуальными барами, группирует по типу файлов.

```bash
# Анализ домашней папки
mop analyze

# Анализ конкретной директории
mop analyze ~/Documents

# Ограничить глубину обхода
mop analyze ~/Projects --depth 3

# Вывод в JSON для скриптов
mop analyze --json
```

---

### `mop uninstall` — Полное удаление приложений

Удаляет не только `.app`-бандл, но и **все связанные файлы**, которые macOS оставляет после стандартного перетаскивания в корзину:

- `~/Library/Application Support/<app>`
- `~/Library/Preferences/com.<bundle>.*`
- `~/Library/Caches/<app>`
- `~/Library/Containers/<bundle>`
- `~/Library/Group Containers`
- `~/Library/Saved Application State`
- `~/Library/WebKit/<bundle>`
- `~/Library/LaunchAgents`

```bash
# Показать что будет удалено
mop uninstall --dry-run "Slack"

# Удалить приложение и все его файлы
mop uninstall "Charles"

# Интерактивный выбор из списка установленных приложений
mop uninstall
```

Перед удалением mop показывает полный список файлов с размерами и запрашивает явное подтверждение заглавной буквой `Y`.

---

### `mop purge` — Очистка сборочных артефактов

Для разработчиков: рекурсивно обходит директории с проектами и находит папки, которые можно безопасно удалить и пересобрать:

- `node_modules/` (Node.js)
- `target/` (Rust)
- `.build/` (Swift)
- `build/`, `dist/` (общие)
- `venv/`, `.venv/`, `__pycache__/` (Python)
- `.gradle/`, `.m2/` (Java)

```bash
# Найти артефакты в папке с проектами
mop purge --dry-run ~/Projects

# Удалить все найденные артефакты
mop purge ~/Projects

# Только старше 30 дней
mop purge ~/Projects --older-than 30
```

---

### `mop status` — Состояние системы

Быстрый обзор текущего состояния системы: диск, память, CPU, топ процессов по потреблению ресурсов.

```bash
mop status

# Вывод в JSON
mop status --json
```

---

### `mop installer` — Поиск установочных файлов

Находит старые `.dmg`, `.pkg` и `.zip` в `~/Downloads` и других стандартных местах — файлы, которые уже не нужны после установки приложений.

```bash
mop installer --dry-run
mop installer
```

---

## Установка

### Homebrew (рекомендуется)

```bash
brew tap thothlab/macos-mop
brew install mop
```

### Скрипт установки

```bash
curl -fsSL https://raw.githubusercontent.com/thothlab/macos-mop/main/install.sh | bash
```

### Из исходников (требуется Rust)

```bash
cargo install --git https://github.com/thothlab/macos-mop
```

### Вручную из GitHub Releases

Скачайте бинарник для своей архитектуры со страницы [Releases](https://github.com/thothlab/macos-mop/releases), сделайте исполняемым и поместите в PATH:

```bash
# Apple Silicon (M1/M2/M3/M4)
curl -L https://github.com/thothlab/macos-mop/releases/latest/download/mop-arm64.tar.gz | tar xz
sudo mv mop-arm64 /usr/local/bin/mop

# Intel
curl -L https://github.com/thothlab/macos-mop/releases/latest/download/mop-x86_64.tar.gz | tar xz
sudo mv mop-x86_64 /usr/local/bin/mop
```

---

## Конфигурация

Файл конфигурации создаётся автоматически при первом запуске: `~/.config/mop/config.toml`

```toml
# Пути для поиска сборочных артефактов командой purge
purge_paths = ["~/Projects", "~/Developer"]

# Пути, которые никогда не трогать
whitelist = []

# Категории по умолчанию для 'mop clean' (без --all)
default_categories = ["user-cache", "system-logs", "crash-reports", "browser-cache", "trash"]

# Возраст файлов для очистки Downloads (дни)
download_age_days = 30

# Возраст артефактов для purge (дни)
purge_age_days = 7
```

---

## Безопасность

mop спроектирован так, чтобы исключить случайную потерю данных:

- **Dry run по умолчанию** — флаг `--dry-run` показывает список файлов без удаления
- **Явное подтверждение** — удаление требует ввода заглавной `Y`; Enter и строчная `y` не принимаются
- **SIP-защита** — системные пути macOS (защищённые SIP) никогда не затрагиваются
- **Белый список** — пути из `whitelist` в конфиге не удаляются никогда
- **Защита системных приложений** — встроенные приложения macOS не могут быть удалены командой `uninstall`
- **Лог операций** — все действия записываются в `~/.config/mop/operations.log`

---

## Требования

- macOS 12 Monterey и новее
- Apple Silicon (arm64) или Intel (x86_64)

---

## Лицензия

MIT
