#!/usr/bin/env bash
set -euo pipefail

echo "🌱 Insertion des données de test..."

docker compose -f docker-compose.dev.yml --env-file .env.dev exec -T postgres psql \
    -U sc_dev -d spaceconquest_dev <<'SQL'
-- Seed: univers de dev
INSERT INTO universes (id, name, speed_multiplier, fleet_speed, economy_speed, research_speed, is_active)
VALUES ('00000000-0000-0000-0000-000000000001', 'Dev Universe', 2, 2, 2, 2, true)
ON CONFLICT (id) DO NOTHING;
SQL

echo "✅ Données de test insérées"
