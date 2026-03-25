#!/usr/bin/env bash
set -euo pipefail

echo "🚀 Space Conquest — Dev Setup"
echo "=============================="

# Vérifier les prérequis
command -v docker >/dev/null 2>&1 || { echo "❌ Docker requis"; exit 1; }
docker compose version >/dev/null 2>&1 || { echo "❌ Docker Compose requis"; exit 1; }

# Copier les fichiers d'env si absents
if [ ! -f .env.dev ]; then
    cp .env.dev.example .env.dev
    echo "✅ .env.dev créé depuis .env.dev.example"
    echo "⚠️  Vérifiez les valeurs dans .env.dev avant de continuer"
fi

# Démarrer les services de base
echo "🐘 Démarrage de PostgreSQL et Redis..."
docker compose -f docker-compose.dev.yml --env-file .env.dev up -d postgres redis

# Attendre que PostgreSQL soit prêt
echo "⏳ Attente de PostgreSQL..."
until docker compose -f docker-compose.dev.yml --env-file .env.dev exec -T postgres pg_isready -U sc_dev -d spaceconquest_dev 2>/dev/null; do
    sleep 1
done
echo "✅ PostgreSQL prêt"

# Démarrer tous les services
echo "🔧 Démarrage de la stack complète..."
docker compose -f docker-compose.dev.yml --env-file .env.dev up -d

echo ""
echo "✅ Stack de développement lancée !"
echo ""
echo "📌 Accès :"
echo "   Frontend : http://localhost:3000"
echo "   Admin    : http://localhost:3001"
echo "   API      : http://localhost:8080"
echo "   Swagger  : http://localhost:8080/swagger-ui"
echo "   Grafana  : http://localhost:3002  (grafana_dev)"
echo "   Mailhog  : http://localhost:8025"
echo "   Postgres : localhost:5432"
echo "   Redis    : localhost:6379"
echo ""
echo "📝 Commandes utiles :"
echo "   make logs          — Voir tous les logs"
echo "   make backend-logs  — Logs du backend"
echo "   make db-shell      — Shell PostgreSQL"
echo "   make migrate       — Lancer les migrations"
echo "   make dev-stop      — Arrêter la stack"
