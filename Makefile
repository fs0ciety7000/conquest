.PHONY: help dev dev-build dev-stop dev-clean migrate migrate-revert seed logs \
        backend-logs frontend-logs worker-logs db-shell redis-shell backend-shell \
        prod-build prod-up prod-down prod-migrate prod-logs backup restore \
        lint-backend test-backend fmt-backend sqlx-prepare

# ─── DÉTECTION PODMAN vs DOCKER ──────────────────────────────────────────────
# Bazzite/Fedora utilisent Podman. Sur Bazzite, `docker` peut pointer vers
# podman via le shim, mais `podman compose` est plus fiable.
COMPOSE_BIN := $(shell \
  if command -v podman >/dev/null 2>&1 && podman info >/dev/null 2>&1; then \
    echo "podman compose"; \
  else \
    echo "docker compose"; \
  fi)

DOCKER_DEV  = $(COMPOSE_BIN) -f docker-compose.dev.yml --env-file .env.dev
DOCKER_PROD = $(COMPOSE_BIN) -f docker-compose.prod.yml --env-file .env.prod

help: ## Affiche l'aide
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-22s\033[0m %s\n", $$1, $$2}'

# ─── SETUP INITIAL ────────────────────────────────────────────────────────────
setup: ## Premier lancement : copie .env, build, migrations
	@[ -f .env.dev ] || cp .env.dev.example .env.dev
	$(DOCKER_DEV) up -d postgres redis
	@echo "Attente que postgres soit prêt (max 60s)..."
	@for i in $$(seq 1 60); do \
		if $(DOCKER_DEV) exec postgres pg_isready -q 2>/dev/null; then \
			echo "Postgres prêt."; break; \
		fi; \
		if [ "$$i" = "60" ]; then \
			echo "ERREUR: Postgres n'a pas démarré. Logs:"; \
			$(DOCKER_DEV) logs --tail 30 postgres; \
			exit 1; \
		fi; \
		sleep 1; \
	done
	$(DOCKER_DEV) run --rm -e DATABASE_URL=$${DATABASE_URL} backend sqlx migrate run
	$(DOCKER_DEV) up -d

# ─── DEV ──────────────────────────────────────────────────────────────────────
dev: ## Lance la stack de développement complète
	$(DOCKER_DEV) up -d

dev-build: ## Rebuild les images dev et relance
	$(DOCKER_DEV) up -d --build

dev-stop: ## Arrête la stack de développement
	$(DOCKER_DEV) down

dev-clean: ## Arrête et supprime les volumes dev (reset complet)
	$(DOCKER_DEV) down -v

# ─── MIGRATIONS ───────────────────────────────────────────────────────────────
migrate: ## Lance les migrations sqlx (dev)
	$(DOCKER_DEV) exec backend sqlx migrate run

migrate-revert: ## Annule la dernière migration (dev)
	$(DOCKER_DEV) exec backend sqlx migrate revert

sqlx-prepare: ## Régénère le cache .sqlx offline (à commiter)
	cd backend && DATABASE_URL=$$(grep DATABASE_URL ../.env.dev | cut -d= -f2-) cargo sqlx prepare

seed: ## Insère les données de test (dev)
	$(DOCKER_DEV) exec postgres psql -U sc_dev -d spaceconquest_dev -f /docker-entrypoint-initdb.d/seed.sql

# ─── LOGS ─────────────────────────────────────────────────────────────────────
logs: ## Affiche tous les logs en temps réel
	$(DOCKER_DEV) logs -f

backend-logs: ## Logs du backend Rust uniquement
	$(DOCKER_DEV) logs -f backend

worker-logs: ## Logs du worker uniquement
	$(DOCKER_DEV) logs -f worker

frontend-logs: ## Logs du frontend uniquement
	$(DOCKER_DEV) logs -f frontend

# ─── SHELLS ───────────────────────────────────────────────────────────────────
db-shell: ## Ouvre un shell psql dans PostgreSQL (dev)
	$(DOCKER_DEV) exec postgres psql -U sc_dev -d spaceconquest_dev

redis-shell: ## Ouvre redis-cli (dev)
	$(DOCKER_DEV) exec redis valkey-cli

backend-shell: ## Shell bash dans le conteneur backend
	$(DOCKER_DEV) exec backend bash

# ─── PROD ─────────────────────────────────────────────────────────────────────
prod-build: ## Build les images de production
	$(DOCKER_PROD) build

prod-up: ## Démarre la stack de production
	$(DOCKER_PROD) up -d

prod-down: ## Arrête la stack de production
	$(DOCKER_PROD) down

prod-migrate: ## Lance les migrations en production
	$(DOCKER_PROD) run --rm backend sqlx migrate run

prod-logs: ## Logs de production
	$(DOCKER_PROD) logs -f

# ─── BACKUP ───────────────────────────────────────────────────────────────────
backup: ## Backup de la base de données prod
	./scripts/db-backup.sh

restore: ## Restaure un backup (BACKUP_FILE=path requis)
	./scripts/db-restore.sh $(BACKUP_FILE)

# ─── QUALITÉ ──────────────────────────────────────────────────────────────────
lint-backend: ## Clippy sur le backend Rust
	$(DOCKER_DEV) exec backend cargo clippy -- -D warnings

test-backend: ## Tests unitaires du backend
	$(DOCKER_DEV) exec backend cargo test

fmt-backend: ## Formate le code Rust
	$(DOCKER_DEV) exec backend cargo fmt
