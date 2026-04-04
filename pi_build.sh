#!/bin/bash
set -e

# Dieses Script laeuft auf dem Raspberry Pi.
# Es wird von update_pi.sh aufgerufen.
#
# - Stoppt den systemd Service (falls vorhanden)
# - Kompiliert die Anwendung
# - Richtet den systemd Service ein (beim ersten Mal)
# - Startet den Service neu

PI_USER="${USER}"
APP_DIR="/home/${PI_USER}/rezepte"
SERVICE_NAME="rezepte"
SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"
BINARY="${APP_DIR}/target/release/rezepte"
DATABASE_URL="sqlite:${APP_DIR}/data/recipes.db"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Service stoppen (falls er laeuft)
if systemctl is-active --quiet "${SERVICE_NAME}" 2>/dev/null; then
    echo -e "${BLUE}Stoppe Service...${NC}"
    sudo systemctl stop "${SERVICE_NAME}"
fi

# Kompilieren
echo -e "${BLUE}Kompiliere (das dauert einen Moment)...${NC}"
cd "${APP_DIR}"
source "$HOME/.cargo/env" 2>/dev/null || true
cargo build --release

# systemd Service einrichten (nur beim ersten Mal)
if [ ! -f "${SERVICE_FILE}" ]; then
    echo -e "${BLUE}Richte systemd Service ein...${NC}"
    sudo tee "${SERVICE_FILE}" > /dev/null << EOF
[Unit]
Description=Rezepte App
After=network.target

[Service]
ExecStart=${BINARY}
WorkingDirectory=${APP_DIR}
Restart=on-failure
Environment=DATABASE_URL=${DATABASE_URL}

[Install]
WantedBy=multi-user.target
EOF
    sudo systemctl daemon-reload
    sudo systemctl enable "${SERVICE_NAME}"
fi

# Service starten
echo -e "${BLUE}Starte Service...${NC}"
sudo systemctl start "${SERVICE_NAME}"

echo -e "${GREEN}App laeuft. Status:${NC}"
systemctl status "${SERVICE_NAME}" --no-pager -l
