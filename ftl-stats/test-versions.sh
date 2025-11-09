#!/bin/bash
# Script per testare ftl-stats con diverse versioni di FTL
# Utile quando hai più versioni di FTL in esecuzione contemporaneamente

set -e

# Colori
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

usage() {
    echo "Usage: $0 <version_name> <shm_path> [example]"
    echo ""
    echo "Arguments:"
    echo "  version_name  Nome descrittivo della versione (es: v5, v6, latest)"
    echo "  shm_path      Path dei file shared memory per questa versione"
    echo "  example       Esempio da eseguire (default: comprehensive)"
    echo ""
    echo "Examples:"
    echo "  $0 v5 /mnt/shm-v5 comprehensive"
    echo "  $0 v6 /mnt/shm-v6 summary"
    echo "  $0 latest /dev/shm basic"
    echo ""
    echo "Tip: Crea symlink ai file shm per organizzarli:"
    echo "  mkdir -p /mnt/shm-v5 /mnt/shm-v6"
    echo "  ln -s /dev/shm/FTL-<PID_V5>-* /mnt/shm-v5/"
    echo "  ln -s /dev/shm/FTL-<PID_V6>-* /mnt/shm-v6/"
    exit 1
}

if [ $# -lt 2 ]; then
    usage
fi

VERSION_NAME="$1"
SHM_PATH="$2"
EXAMPLE="${3:-comprehensive}"

echo -e "${YELLOW}━━━ Testing FTL Stats with $VERSION_NAME ━━━${NC}"
echo ""

# Verifica che il path esista
if [ ! -d "$SHM_PATH" ]; then
    echo -e "${RED}❌ Error: Directory $SHM_PATH not found${NC}"
    exit 1
fi

# Trova i file FTL-* in quel path
FTL_FILES=$(ls "$SHM_PATH"/FTL-*-settings 2>/dev/null | head -1)

if [ -z "$FTL_FILES" ]; then
    echo -e "${RED}❌ Error: No FTL shared memory files found in $SHM_PATH${NC}"
    echo "Expected files like: FTL-<PID>-settings"
    exit 1
fi

# Estrai il PID dal nome file
PID=$(basename "$FTL_FILES" | sed 's/FTL-\([0-9]*\)-.*/\1/')

if [ -z "$PID" ]; then
    echo -e "${RED}❌ Error: Could not extract PID from $FTL_FILES${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Found FTL PID: $PID${NC}"
echo -e "${GREEN}✓ Shared memory path: $SHM_PATH${NC}"
echo -e "${GREEN}✓ Running example: $EXAMPLE${NC}"
echo ""

# Conta i file
FILE_COUNT=$(ls "$SHM_PATH"/FTL-$PID-* 2>/dev/null | wc -l)
echo "📋 Found $FILE_COUNT shared memory segments"
ls -lh "$SHM_PATH"/FTL-$PID-* | awk '{print "   ", $9, "-", $5}'
echo ""

# Esegui l'esempio con le variabili d'ambiente
echo -e "${YELLOW}🚀 Running example...${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

FTL_PID=$PID FTL_SHM_PATH=$SHM_PATH cargo run --example "$EXAMPLE"
