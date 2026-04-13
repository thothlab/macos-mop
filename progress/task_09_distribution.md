# Задача 9: Распространение — ВЫПОЛНЕНО

## Что сделано
- GitHub Actions CI/CD (ci.yml + release.yml)
  - CI: fmt, clippy, build, test на каждый PR/push
  - Release: сборка для arm64 + x86_64, universal binary, GitHub Release
- install.sh — установочный скрипт (curl | bash)
- Formula/syscleaner.rb — Homebrew formula
- README.md — документация с примерами
- LICENSE — MIT
- .gitignore

## Способы установки
1. `brew tap USER/syscleaner && brew install syscleaner`
2. `curl -fsSL .../install.sh | bash`
3. `cargo install --git ...`
4. Скачать бинарник из GitHub Releases
