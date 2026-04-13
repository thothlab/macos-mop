# macOS System Cleaner — Техническое задание (ТЗ)

## 1. Обзор проекта

**Название:** `syscleaner` (рабочее название)
**Тип:** Консольная утилита (CLI) для macOS
**Язык:** Rust
**Целевая платформа:** macOS 12+ (aarch64 + x86_64)
**Распространение:** Homebrew, GitHub Releases, установочный скрипт

### Цель
Разработать быструю, безопасную консольную утилиту для очистки macOS от системного мусора, остатков удалённых программ, кэшей, логов, сборочных артефактов и прочего. Аналог CleanMyMac / Mole, но в виде компактного CLI на Rust.

---

## 2. Функциональные требования

### 2.1. Команда `clean` — Очистка системы

Категории очистки (каждая может быть включена/выключена флагами):

| Категория | Что чистит |
|---|---|
| **user-cache** | `~/Library/Caches/*` — кэши пользовательских приложений |
| **system-logs** | `/var/log/`, `~/Library/Logs/` — системные и пользовательские логи |
| **crash-reports** | `~/Library/Logs/DiagnosticReports/` — отчёты о сбоях |
| **browser-cache** | Chrome, Safari, Firefox, Arc, Brave — кэши браузеров |
| **app-leftovers** | Остатки удалённых приложений в Library (Preferences, Application Support, Containers, Launch Agents) |
| **dev-tools** | Xcode DerivedData, CoreSimulator, npm/yarn/pnpm cache, pip cache, cargo cache, CocoaPods cache |
| **homebrew** | `brew cleanup` — старые версии формул и кэш Homebrew |
| **trash** | Очистка корзины (`~/.Trash/`) |
| **downloads** | Старые файлы из `~/Downloads/` (по дате, с подтверждением) |
| **mail-attachments** | Кэшированные вложения почты |
| **docker** | Docker images/containers/volumes неиспользуемые |

**Флаги:**
- `--dry-run` — показать что будет удалено без удаления
- `--all` — очистить все категории
- `--category <name>` — очистить только указанную категорию
- `--force` — не спрашивать подтверждение
- `--verbose` — подробный вывод

### 2.2. Команда `analyze` — Анализ дискового пространства

- Рекурсивный обход указанной директории (по умолчанию `~`)
- Вывод топ-N самых больших файлов/директорий
- Группировка по типу файлов
- Формат вывода: таблица с размерами и процентными барами
- Флаг `--json` для машиночитаемого вывода
- Флаг `--depth <N>` для ограничения глубины

### 2.3. Команда `uninstall` — Удаление приложений

- Показать список установленных приложений из `/Applications/` и `~/Applications/`
- Для выбранного приложения найти все связанные файлы:
  - `~/Library/Application Support/<app>/`
  - `~/Library/Preferences/com.<bundle>.*`
  - `~/Library/Caches/<app>/`
  - `~/Library/Containers/<bundle>/`
  - `~/Library/Group Containers/<group>/`
  - `~/Library/Saved Application State/<bundle>.savedState/`
  - `~/Library/HTTPStorages/<bundle>/`
  - `~/Library/WebKit/<bundle>/`
  - `~/Library/LaunchAgents/*<bundle>*`
  - `/Library/LaunchDaemons/*<bundle>*`
- Показать все найденные файлы и размеры
- Удалить после подтверждения (или с `--force`)
- `--dry-run` поддержка

### 2.4. Команда `purge` — Очистка сборочных артефактов

- Сканирование указанных директорий (по умолчанию `~/Projects`, `~/Developer`, `~/Documents`)
- Поиск и удаление:
  - `node_modules/`
  - `target/` (Rust)
  - `.build/` (Swift)
  - `build/` (Gradle, CMake)
  - `dist/`
  - `venv/`, `.venv/`, `env/`
  - `__pycache__/`
  - `.gradle/`
  - `Pods/`
- Показ размера каждого найденного артефакта
- Фильтр по возрасту (по умолчанию: проекты старше 7 дней)
- Интерактивный выбор что удалять

### 2.5. Команда `status` — Состояние системы

- Использование диска (общее / свободно / использовано)
- Использование памяти (RAM)
- Информация о CPU
- Топ-5 процессов по потреблению CPU/RAM
- Оценка "здоровья" системы (простая метрика)

### 2.6. Команда `installer` — Поиск установщиков

- Поиск `.dmg`, `.pkg`, `.zip`, `.iso` файлов в:
  - `~/Downloads/`
  - `~/Desktop/`
  - Homebrew cache
- Показ размеров и предложение удалить

---

## 3. Нефункциональные требования

### 3.1. Безопасность
- **Dry-run по умолчанию** при первом запуске
- Защита системных директорий (SIP-protected paths)
- Белый список защищённых приложений/путей (настраиваемый)
- Логирование всех операций удаления в `~/.config/syscleaner/operations.log`
- Перемещение в корзину вместо прямого удаления (опционально, `--trash` флаг)

### 3.2. Производительность
- Быстрое сканирование файловой системы (параллельный обход)
- Целевое время сканирования: < 5 сек для домашней директории

### 3.3. UX
- Цветной вывод (отключаемый через `--no-color` или `NO_COLOR` env)
- Прогресс-бары при длительных операциях
- Человекочитаемые размеры файлов (KB, MB, GB)
- Интерактивный выбор с чекбоксами для списков
- Поддержка `--json` для интеграции со скриптами

### 3.4. Конфигурация
- Файл конфигурации: `~/.config/syscleaner/config.toml`
- Настройки:
  - Пути для сканирования `purge`
  - Белый список путей
  - Категории очистки по умолчанию
  - Возраст файлов для `downloads` и `purge`

---

## 4. Архитектура

```
src/
├── main.rs              # Точка входа, CLI парсинг (clap)
├── cli.rs               # Определение CLI команд и аргументов
├── commands/
│   ├── mod.rs
│   ├── clean.rs         # Логика команды clean
│   ├── analyze.rs       # Логика команды analyze
│   ├── uninstall.rs     # Логика команды uninstall
│   ├── purge.rs         # Логика команды purge
│   ├── status.rs        # Логика команды status
│   └── installer.rs     # Логика команды installer
├── core/
│   ├── mod.rs
│   ├── scanner.rs       # Параллельный обход файловой системы
│   ├── file_ops.rs      # Безопасные файловые операции
│   ├── size.rs          # Подсчёт и форматирование размеров
│   └── protection.rs    # Защита системных путей
├── ui/
│   ├── mod.rs
│   ├── progress.rs      # Прогресс-бары и спиннеры
│   ├── table.rs         # Форматированные таблицы
│   ├── prompt.rs        # Интерактивные подтверждения
│   └── colors.rs        # Цветовые стили
├── config/
│   ├── mod.rs
│   └── settings.rs      # Чтение/запись конфигурации
└── cleaners/
    ├── mod.rs
    ├── user_cache.rs
    ├── system_logs.rs
    ├── browser_cache.rs
    ├── app_leftovers.rs
    ├── dev_tools.rs
    ├── homebrew.rs
    ├── trash.rs
    ├── downloads.rs
    └── docker.rs
```

### Зависимости (Cargo)
- `clap` — CLI парсинг
- `walkdir` — обход файловой системы
- `rayon` — параллелизм
- `indicatif` — прогресс-бары
- `console` — цвета и стили терминала
- `dialoguer` — интерактивные промпты
- `sysinfo` — информация о системе
- `serde` + `toml` — конфигурация
- `serde_json` — JSON вывод
- `dirs` — стандартные пути
- `chrono` — работа с датами
- `bytesize` — форматирование размеров
- `log` + `env_logger` — логирование
- `plist` — чтение Info.plist для определения bundle ID приложений

---

## 5. Распространение

### 5.1. GitHub Releases
- CI/CD через GitHub Actions
- Сборка для `aarch64-apple-darwin` и `x86_64-apple-darwin`
- Universal binary через `lipo`
- Публикация релизов с SHA256 чексуммами

### 5.2. Homebrew
- Формула в tap-репозитории (`homebrew-syscleaner`)
- `brew tap <user>/syscleaner && brew install syscleaner`

### 5.3. Установочный скрипт
- `curl -fsSL https://raw.githubusercontent.com/<user>/syscleaner/main/install.sh | bash`
- Определение архитектуры, загрузка бинарника, установка в `~/.local/bin/`

### 5.4. Установка из исходников
- `cargo install --git https://github.com/<user>/syscleaner`

---

## 6. Пример использования

```bash
# Показать что можно очистить (без удаления)
syscleaner clean --dry-run

# Очистить всё
syscleaner clean --all

# Только кэши и логи
syscleaner clean --category user-cache --category system-logs

# Анализ диска
syscleaner analyze ~/Documents --depth 3

# Удалить приложение
syscleaner uninstall "Slack"

# Очистить сборочные артефакты
syscleaner purge ~/Projects

# Статус системы
syscleaner status

# Найти установщики
syscleaner installer
```
