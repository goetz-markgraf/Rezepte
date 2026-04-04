#!/bin/bash
set -e

# Sichert die Datenbank vom Raspberry Pi auf den lokalen Rechner
#
# Verwendung:
#   pi/backup_db.sh

PI_HOST="markgrafen-pi"
PI_USER="${USER}"
PI_DB="/home/${PI_USER}/rezepte/data/recipes.db"
BACKUP_DIR="./backup"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

TIMESTAMP=$(date +"%Y-%m-%d_%H-%M")
BACKUP_FILE="${BACKUP_DIR}/recipes_${TIMESTAMP}.db"

mkdir -p "${BACKUP_DIR}"

echo -e "${YELLOW}Sichere Datenbank von markgrafen-pi...${NC}"
scp "${PI_USER}@${PI_HOST}:${PI_DB}" "${BACKUP_FILE}"

echo -e "${GREEN}Gesichert: ${BACKUP_FILE}${NC}"
