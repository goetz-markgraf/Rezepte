#!/bin/bash
set -e

# Update Script: Deployed und startet die App auf dem Raspberry Pi neu
#
# Verwendung:
#   pi/update_pi.sh

PI_HOST="markgrafen-pi"
PI_USER="${USER}"
PI_TARGET_DIR="/home/${PI_USER}/rezepte"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${YELLOW}=== Rezepte Update ===${NC}"
echo ""

# 1. Quellcode kopieren
echo -e "${BLUE}[1/3] Kopiere Quellcode...${NC}"
pi/deploy_to_pi.sh

# 2. Auf dem Pi: kompilieren und neustarten
echo ""
echo -e "${BLUE}[2/3] Kompiliere auf dem Pi...${NC}"
ssh "${PI_USER}@${PI_HOST}" "bash ${PI_TARGET_DIR}/pi/build.sh"

echo ""
echo -e "${GREEN}=== Update abgeschlossen ===${NC}"
