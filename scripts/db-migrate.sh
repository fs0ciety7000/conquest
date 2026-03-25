#!/usr/bin/env bash
set -euo pipefail

ENV_FILE="${1:-.env.dev}"
COMPOSE_FILE="docker-compose.dev.yml"

if [ "$ENV_FILE" = ".env.prod" ]; then
    COMPOSE_FILE="docker-compose.prod.yml"
fi

echo "🗄️  Lancement des migrations sqlx ($ENV_FILE)..."
docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" exec backend sqlx migrate run
echo "✅ Migrations terminées"
