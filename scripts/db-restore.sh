#!/usr/bin/env bash
set -euo pipefail

BACKUP_FILE="${1:-}"

if [ -z "$BACKUP_FILE" ]; then
    echo "❌ Usage: $0 <backup_file.sql.gz>"
    exit 1
fi

if [ ! -f "$BACKUP_FILE" ]; then
    echo "❌ Fichier introuvable : $BACKUP_FILE"
    exit 1
fi

echo "⚠️  ATTENTION: Cette opération va écraser la base de données de production !"
read -r -p "Tapez 'yes' pour confirmer : " confirm
if [ "$confirm" != "yes" ]; then
    echo "Annulé."
    exit 0
fi

echo "🔄 Restauration depuis $BACKUP_FILE..."
gunzip -c "$BACKUP_FILE" | docker compose -f docker-compose.prod.yml --env-file .env.prod exec -T postgres \
    psql -U sc_prod spaceconquest_prod

echo "✅ Restauration terminée"
