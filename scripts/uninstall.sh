#!/usr/bin/env bash
set -e

GREEN="\033[0;32m"
YELLOW="\033[1;33m"
CYAN="\033[0;36m"
RED="\033[0;31m"
BOLD="\033[1m"
RESET="\033[0m"
CHECK="${GREEN}✅${RESET}"
FAIL="${RED}❌${RESET}"
INFO="${CYAN}➜${RESET}"

echo -e "${BOLD}${CYAN}"
echo "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓"
echo "┃           voxlan Uninstaller         ┃"
echo "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛"
echo -e "${RESET}"

INSTALL_PATH="/usr/local/bin/voxlan"
REPO_DIR="$HOME/voxlan"

echo -e "${INFO} Removing voxlan binary..."

if [ ! -f "$INSTALL_PATH" ]; then
    echo -e "${FAIL} voxlan binary not found at ${INSTALL_PATH}"
else
    sudo rm -f "$INSTALL_PATH"
    echo -e "${CHECK} Removed voxlan binary from ${INSTALL_PATH}"
fi

echo -e "${INFO} Removing voxlan repository..."

if [ ! -d "$REPO_DIR" ]; then
    echo -e "${YELLOW}Warning: voxlan repository not found at ${REPO_DIR}${RESET}"
else
    rm -rf "$REPO_DIR"
    echo -e "${CHECK} Removed voxlan repository at ${REPO_DIR}"
fi

echo -e "\n${BOLD}${GREEN}voxlan is completely removed from your system. ${CHECK}${RESET}"
