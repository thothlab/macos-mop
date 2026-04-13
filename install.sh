#!/bin/bash
set -euo pipefail

# mop installer
# Usage: curl -fsSL https://raw.githubusercontent.com/USER/mop/main/install.sh | bash

REPO="thothlab/macos-mop"
INSTALL_DIR="${HOME}/.local/bin"
BINARY_NAME="mop"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

info() { echo -e "${CYAN}[info]${NC} $1"; }
success() { echo -e "${GREEN}[ok]${NC} $1"; }
warn() { echo -e "${YELLOW}[warn]${NC} $1"; }
error() { echo -e "${RED}[error]${NC} $1"; exit 1; }

# Check OS
if [[ "$(uname -s)" != "Darwin" ]]; then
    error "mop is only supported on macOS."
fi

# Detect architecture
ARCH=$(uname -m)
case "$ARCH" in
    arm64|aarch64) ARCH_LABEL="arm64" ;;
    x86_64)        ARCH_LABEL="x86_64" ;;
    *)             error "Unsupported architecture: $ARCH" ;;
esac

info "Detected macOS ($ARCH_LABEL)"

# Get latest release version
info "Fetching latest release..."
LATEST=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')

if [[ -z "$LATEST" ]]; then
    error "Could not determine latest release."
fi

info "Latest version: $LATEST"

# Download
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST}/mop-${ARCH_LABEL}.tar.gz"
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

info "Downloading ${BINARY_NAME} ${LATEST} for ${ARCH_LABEL}..."
curl -fsSL "$DOWNLOAD_URL" -o "${TEMP_DIR}/mop.tar.gz" || error "Download failed."

# Extract
cd "$TEMP_DIR"
tar xzf mop.tar.gz

# Install
mkdir -p "$INSTALL_DIR"
cp "mop-${ARCH_LABEL}" "${INSTALL_DIR}/${BINARY_NAME}"
chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

success "Installed ${BINARY_NAME} to ${INSTALL_DIR}/${BINARY_NAME}"

# Check PATH
if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
    warn "${INSTALL_DIR} is not in your PATH."
    echo ""
    echo "  Add this to your shell profile (~/.zshrc or ~/.bashrc):"
    echo ""
    echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
fi

success "Installation complete! Run 'mop --help' to get started."
