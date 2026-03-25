#!/usr/bin/env bash
set -euo pipefail

TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
BACKUP_DIR="./backups"
BACKUP_FILE="$BACKUP_DIR/spaceconquest_prod_$TIMESTAMP.sql.gz"

mkdir -p "$BACKUP_DIR"

echo "💾 Backup de la base de données prod..."
docker compose -f docker-compose.prod.yml --env-file .env.prod exec -T postgres \
    pg_dump -U sc_prod spaceconquest_prod | gzip > "$BACKUP_FILE"

echo "✅ Backup créé : $BACKUP_FILE"
ls -lh "$BACKUP_FILE"
