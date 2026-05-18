#!/bin/bash
set -e

# Deploy Script: Kopiert das Projekt auf den Raspberry Pi
#
# Verwendung:
#   pi/deploy_to_pi.sh          # Kopiert Quellcode (ohne Datenbank)
#   pi/deploy_to_pi.sh --init   # Erstmaliges Setup: kopiert auch die Datenbank

PI_HOST="markgrafen-pi"
PI_USER="${USER}"
PI_TARGET_DIR="/home/${PI_USER}/rezepte"
NGINX_CONF_DIR="/home/${PI_USER}/nginx-config"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

INIT_MODE=false
if [ "$1" == "--init" ]; then
    INIT_MODE=true
fi

echo -e "${YELLOW}=== Deploy auf markgrafen-pi ===${NC}"
echo ""

# Zielverzeichnis anlegen
ssh "${PI_USER}@${PI_HOST}" "mkdir -p ${PI_TARGET_DIR}/data"

# Quellcode kopieren (ohne Build-Artefakte, Testdaten, etc.)
echo -e "${BLUE}Kopiere Quellcode...${NC}"
rsync -avz --delete \
    --exclude='target/' \
    --exclude='node_modules/' \
    --exclude='.git/' \
    --exclude='data/' \
    --exclude='backup/' \
    --exclude='test-results/' \
    --exclude='playwright-report/' \
    --exclude='.DS_Store' \
    --exclude='.direnv/' \
    --exclude='.agentbox/' \
    --exclude='.ralph/' \
    --exclude='.claude/' \
    --exclude='.opencode/' \
    ./ "${PI_USER}@${PI_HOST}:${PI_TARGET_DIR}/"

# Beim ersten Mal: Datenbank kopieren
if [ "$INIT_MODE" == "true" ]; then
    echo ""
    echo -e "${BLUE}Erstmaliges Setup: Kopiere Datenbank...${NC}"
    scp "data/recipes.db" "${PI_USER}@${PI_HOST}:${PI_TARGET_DIR}/data/"
    
    # Nginx-Konfiguration kopieren
    echo -e "${BLUE}Kopiere Nginx-Konfiguration...${NC}"
    ssh "${PI_USER}@${PI_HOST}" "mkdir -p ${NGINX_CONF_DIR}"
    scp "../pi/nginx/combined.conf" "${PI_USER}@${PI_HOST}:${NGINX_CONF_DIR}/"
    
    # Nginx-Konfiguration aktivieren
    echo -e "${BLUE}Aktiviere Nginx-Konfiguration...${NC}"
    ssh "${PI_USER}@${PI_HOST}" "sudo cp ${NGINX_CONF_DIR}/combined.conf /etc/nginx/sites-available/combined && sudo ln -sf /etc/nginx/sites-available/combined /etc/nginx/sites-enabled/combined && sudo rm -f /etc/nginx/sites-enabled/default && sudo nginx -t && sudo systemctl reload nginx"
    
    echo -e "${GREEN}Datenbank und Nginx-Konfiguration kopiert.${NC}"
fi

echo ""
echo -e "${GREEN}=== Deploy abgeschlossen ===${NC}"
echo ""
echo "Naechste Schritte auf dem Pi:"
echo "  ssh ${PI_USER}@${PI_HOST}"
echo "  cd ${PI_TARGET_DIR}"
echo "  cargo build --release"
