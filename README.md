# mop

[English](README.en.md) | **Русский**

Быстрая консольная утилита для очистки macOS, написанная на Rust.

Удаляет системный мусор, кэши, логи, остатки приложений, сборочные артефакты и многое другое — прямо из терминала.

## Возможности

- **clean** — Удаление системных кэшей, логов, данных браузеров, остатков приложений, кэша Homebrew, корзины и др.
- **analyze** — Анализ дискового пространства с визуальными барами и разбивкой по типам файлов
- **uninstall** — Полное удаление приложений вместе со всеми связанными файлами
- **purge** — Поиск и удаление сборочных артефактов (node_modules, target, .build, venv и т.д.)
- **status** — Обзор состояния системы: диск, память, CPU, процессы
- **installer** — Поиск и удаление старых установочных файлов (.dmg, .pkg, .zip)

## Установка

### Homebrew

```bash
brew tap thothlab/macos-mop
brew install mop
```

### Скрипт установки

```bash
curl -fsSL https://raw.githubusercontent.com/thothlab/macos-mop/main/install.sh | bash
```

### Из исходников

```bash
cargo install --git https://github.com/thothlab/macos-mop
```

### Из GitHub Releases

Скачайте последний бинарник со страницы [Releases](https://github.com/thothlab/macos-mop/releases) и поместите в PATH.

## Использование

```bash
# Показать что можно очистить (без удаления)
mop clean --dry-run --all

# Очистить всё
mop clean --all

# Очистить конкретные категории
mop clean --category user-cache --category browser-cache

# Анализ дискового пространства
mop analyze ~/Documents --depth 3

# Полное удаление приложения
mop uninstall "Slack"

# Удалить сборочные артефакты из проектов
mop purge ~/Projects

# Состояние системы
mop status

# Найти установочные файлы
mop installer
```

## Категории очистки

| Категория | Описание |
|---|---|
| `user-cache` | Кэши приложений в ~/Library/Caches |
| `system-logs` | Системные и пользовательские логи |
| `crash-reports` | Отчёты о сбоях и диагностические данные |
| `browser-cache` | Кэши Chrome, Safari, Firefox, Arc, Brave, Edge |
| `app-leftovers` | Остатки удалённых приложений |
| `dev-tools` | Кэши Xcode, npm, yarn, pip, cargo, CocoaPods, Gradle, Maven, Go |
| `homebrew` | Кэш загрузок Homebrew и старые версии формул |
| `trash` | Корзина macOS |
| `downloads` | Старые файлы в ~/Downloads |
| `docker` | Данные Docker Desktop и кэш buildx |

## Конфигурация

Файл конфигурации: `~/.config/mop/config.toml`

```toml
# Пути для поиска сборочных артефактов
purge_paths = ["~/Projects", "~/Developer"]

# Пути, которые никогда не нужно чистить
whitelist = []

# Категории по умолчанию для команды 'clean'
default_categories = ["user-cache", "system-logs", "crash-reports", "browser-cache", "trash"]

# Порог возраста для очистки загрузок (дни)
download_age_days = 30

# Порог возраста для команды purge (дни)
purge_age_days = 7
```

## Безопасность

- Режим **dry run** (`--dry-run`) для просмотра изменений перед удалением
- Защищённые системные пути (SIP) никогда не затрагиваются
- Системные приложения защищены от удаления
- Белый список путей, которые нужно сохранить
- Все операции логируются в `~/.config/mop/operations.log`
- Подтверждение удаления требует ввода заглавной `Y` — Enter и строчная `y` не принимаются

## Лицензия

MIT
