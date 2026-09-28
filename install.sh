#!/usr/bin/env bash

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

REPO="aether-flux/infiltrator"

### HEADING ASCII

echo -e "${MAGENTA}${BOLD}"
echo " _         ___ _ _                                    "
echo "| |       / __|_) |  _                 _              "
echo "| |____ _| |__ _| |_| |_  ____ _____ _| |_ ___   ____ "
echo "| |  _ (_   __) | (_   _)/ ___|____ (_   _) _ \ / ___)"
echo "| | | | || |  | | | | |_| |   / ___ | | || |_| | |    "
echo "|_|_| |_||_|  |_|\_) \__)_|   \_____|  \__)___/|_|    "
echo -e "${RESET}"
echo ""

# Environment check
echo -e "${BLUE}[1/4] Checking system environment...${RESET}"

if [ "$XDG_SESSION_TYPE" != "wayland" ]; then
  echo -e "${YELLOW}WARN: Current session is '$XDG_SESSION_TYPE'. Infiltrator is optimized for wayland.${RESET}"
  echo -e "${YELLOW}Text expansion and shell output clipboard actions may not work properly."
fi

# Checking dependencies
MISSING_DEPS=()
command -v wtype >/dev/null 2>&1 || MISSING_DEPS+=("wtype")
command -v wl-copy >/dev/null 2>&1 || MISSING_DEPS+=("wl-clipboard")
command -v xclip >/dev/null 2>&1 || MISSING_DEPS+=("xclip")

if [ ${#MISSING_DEPS[@]} -ne 0 ]; then
  echo -e "${YELLOW}WARN: Recommended runtime dependencies missing: ${MISSING_DEPS[*]}${RESET}"
else
  echo -e "${GREEN}All recommended runtime tooling detected (wtype, wl-clipboard)"
fi

# Create target directories
echo -e "${BLUE}[2/4] Setting up installation directories...${RESET}"

BIN_DIR="$HOME/.local/bin"
APP_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons/hicolor/48x48/apps"

mkdir -p "$BIN_DIR" "$APP_DIR" "$ICON_DIR"

# Fetch latest release assets
echo -e "${BLUE}[3/4] Downloading latest release from GitHub...${RESET}"

LATEST_TAG=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
  echo -e "${RED}ERR: Failed to fetch latest release version from GitHub.${RESET}"
  exit 1
fi

echo -e "Latest release tag: ${GREEN}$LATEST_TAG${RESET}"

# Download assets from latest tag
TMP_DIR=$(mktemp -d)
TAR_URL="https://github.com/$REPO/releases/download/$LATEST_TAG/infiltrator-x86_64-unknown-linux-gnu.tar.gz"

curl -sL "$TAR_URL" -o "$TMP_DIR/infiltrator.tar.gz"
tar -xzf "$TMP_DIR/infiltrator.tar.gz" -C "$TMP_DIR" --strip-components=1

cp "$TMP_DIR/infiltrator" "$BIN_DIR/infiltrator"
chmod +x "$BIN_DIR/infiltrator"

if [ -f "$TMP_DIR/infiltrator.desktop" ]; then
  cp "$TMP_DIR/infiltrator.desktop" "$APP_DIR/infiltrator.desktop"
fi

if [ -f "$TMP_DIR/infiltrator.png" ]; then
  cp "$TMP_DIR/infiltrator.png" "$ICON_DIR/infiltrator.png"
fi

rm -rf "$TMP_DIR"

# Refresh icon cache
command -v gtk-update-icon-cache >/dev/null 2>&1 && gtk-update-icon-cache -f -t "$HOME/.local/share/icons/hicolor" 2>/dev/null || true

# Success!
echo -e "\n${GREEN}${BOLD}Infiltrator successfully installed!${RESET}"
echo -e "Binary installed to: ${CYAN}$BIN_DIR/infiltrator${RESET}"

if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo -e "${YELLOW}Note: Make sure '$BIN_DIR' is in your PATH env variable.${RESET}"
fi

# Keybind suggestions
echo -e "\n${BOLD}Keybinding setup options:${RESET}"
echo -e "  ${CYAN}Hyprland (lua):${RESET}   hl.bind(\"SUPER + Space\", hl.dsp.exec_cmd(\"infiltrator\"))"
echo -e "  ${CYAN}Sway/i3:${RESET}          bindsym \$mod+space exec infiltrator"
echo -e "  ${CYAN}sxhkd (bspwm):${RESET}"
echo -e "  super + space"
echo -e "    infiltrator"
echo -e "  ${CYAN}KDE, Gnome, etc:${RESET}  Add custom shortcut for 'infiltrator' in System Settings"
echo ""
