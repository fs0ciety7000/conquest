# SPACE CONQUEST — INFRASTRUCTURE & DÉPLOIEMENT
> Addendum v1.3 — Environnements Dev & Prod  
> Docker Compose séparés · Variables d'environnement · Coolify · Scripts utilitaires

---

## TABLE DES MATIÈRES

- [Structure des fichiers](#structure-des-fichiers)
- [Variables d'environnement](#variables-denvironnement)
- [Docker Compose — Développement](#docker-compose--développement)
- [Docker Compose — Production (Coolify)](#docker-compose--production-coolify)
- [Dockerfiles](#dockerfiles)
- [Configuration Coolify](#configuration-coolify)
- [Scripts utilitaires](#scripts-utilitaires)
- [Makefile](#makefile)

---

## Structure des Fichiers

```
space-conquest/
├── .env.dev                        # Variables dev (jamais committé)
├── .env.dev.example                # Template dev (committé)
├── .env.prod                       # Variables prod (jamais committé)
├── .env.prod.example               # Template prod (committé)
├── .gitignore
│
├── docker-compose.dev.yml          # Stack complète en local
├── docker-compose.prod.yml         # Stack prod (Coolify)
│
├── backend/
│   ├── Dockerfile.dev              # Image dev (hot-reload avec cargo-watch)
│   ├── Dockerfile.prod             # Image prod (multi-stage, binaire optimisé)
│   └── ...
│
├── frontend/
│   ├── Dockerfile.dev              # Image dev (Next.js dev server)
│   ├── Dockerfile.prod             # Image prod (Next.js standalone)
│   └── ...
│
├── admin/
│   ├── Dockerfile.dev
│   ├── Dockerfile.prod
│   └── ...
│
├── nginx/
│   └── dev.conf                    # Reverse proxy local (optionnel)
│
├── scripts/
│   ├── dev-setup.sh                # Premier lancement dev
│   ├── prod-deploy.sh              # Déploiement prod manuel
│   ├── db-migrate.sh               # Lancer les migrations
│   ├── db-seed.sh                  # Seed données de test
│   ├── db-backup.sh                # Backup prod
│   └── db-restore.sh               # Restore backup
│
└── Makefile                        # Commandes raccourcies
```

---

## Variables d'Environnement

### `.env.dev.example`

```bash
# ─── ENVIRONNEMENT ────────────────────────────────────────────
ENV=development

# ─── BASE DE DONNÉES ──────────────────────────────────────────
DB_HOST=postgres
DB_PORT=5432
DB_NAME=spaceconquest_dev
DB_USER=sc_dev
DB_PASSWORD=devpassword123
DATABASE_URL=postgres://sc_dev:devpassword123@postgres:5432/spaceconquest_dev

# ─── REDIS ────────────────────────────────────────────────────
REDIS_HOST=redis
REDIS_PORT=6379
REDIS_URL=redis://redis:6379

# ─── AUTH ─────────────────────────────────────────────────────
# Générer avec : openssl rand -hex 32
JWT_SECRET=dev_jwt_secret_change_me_in_prod_32chars_min
JWT_EXPIRY_MINUTES=60
REFRESH_TOKEN_EXPIRY_DAYS=30

# ─── BACKEND ──────────────────────────────────────────────────
BACKEND_HOST=0.0.0.0
BACKEND_PORT=8080
RUST_LOG=debug,sqlx=debug,tower_http=debug
GAME_DATA_PATH=/app/game_data

# Universe de dev (UUID fixe pour reproductibilité)
DEFAULT_UNIVERSE_ID=00000000-0000-0000-0000-000000000001

# ─── FRONTEND ─────────────────────────────────────────────────
NEXT_PUBLIC_API_URL=http://localhost:8080
NEXT_PUBLIC_WS_URL=ws://localhost:8080/ws
NEXT_PUBLIC_ENV=development

# ─── ADMIN PANEL ──────────────────────────────────────────────
ADMIN_PORT=3001
NEXT_PUBLIC_ADMIN_API_URL=http://localhost:8080

# Premier compte superadmin (créé au premier boot si absent)
ADMIN_BOOTSTRAP_EMAIL=admin@localhost
ADMIN_BOOTSTRAP_PASSWORD=AdminDev123!
ADMIN_BOOTSTRAP_USERNAME=superadmin

# ─── MONITORING (optionnel en dev) ───────────────────────────
GRAFANA_PASSWORD=grafana_dev
PROMETHEUS_RETENTION=7d

# ─── CORS ─────────────────────────────────────────────────────
CORS_ORIGINS=http://localhost:3000,http://localhost:3001
```

---

### `.env.prod.example`

```bash
# ─── ENVIRONNEMENT ────────────────────────────────────────────
ENV=production

# ─── BASE DE DONNÉES ──────────────────────────────────────────
DB_HOST=postgres
DB_PORT=5432
DB_NAME=spaceconquest_prod
DB_USER=sc_prod
# Générer avec : openssl rand -base64 32
DB_PASSWORD=CHANGE_ME_STRONG_PASSWORD
DATABASE_URL=postgres://sc_prod:CHANGE_ME_STRONG_PASSWORD@postgres:5432/spaceconquest_prod

# PgBouncer (connection pooling en prod)
PGBOUNCER_URL=postgres://sc_prod:CHANGE_ME_STRONG_PASSWORD@pgbouncer:5432/spaceconquest_prod

# ─── REDIS ────────────────────────────────────────────────────
REDIS_HOST=redis
REDIS_PORT=6379
# Mot de passe Redis en prod
REDIS_PASSWORD=CHANGE_ME_REDIS_PASSWORD
REDIS_URL=redis://:CHANGE_ME_REDIS_PASSWORD@redis:6379

# ─── AUTH ─────────────────────────────────────────────────────
# Générer avec : openssl rand -hex 64
JWT_SECRET=CHANGE_ME_VERY_LONG_RANDOM_SECRET_FOR_PRODUCTION
JWT_EXPIRY_MINUTES=15
REFRESH_TOKEN_EXPIRY_DAYS=30

# ─── BACKEND ──────────────────────────────────────────────────
BACKEND_HOST=0.0.0.0
BACKEND_PORT=8080
RUST_LOG=info,sqlx=warn,tower_http=info
GAME_DATA_PATH=/app/game_data

DEFAULT_UNIVERSE_ID=CHANGE_ME_GENERATE_UUID

# ─── DOMAINES ─────────────────────────────────────────────────
DOMAIN=space-conquest.gg
API_DOMAIN=api.space-conquest.gg
ADMIN_DOMAIN=admin.space-conquest.gg
ACME_EMAIL=admin@space-conquest.gg

# ─── FRONTEND ─────────────────────────────────────────────────
NEXT_PUBLIC_API_URL=https://api.space-conquest.gg
NEXT_PUBLIC_WS_URL=wss://api.space-conquest.gg/ws
NEXT_PUBLIC_ENV=production

# ─── ADMIN PANEL ──────────────────────────────────────────────
# Accès via Tailscale uniquement (pas exposé publiquement)
ADMIN_TAILSCALE_IP=100.x.x.x
NEXT_PUBLIC_ADMIN_API_URL=https://api.space-conquest.gg

ADMIN_BOOTSTRAP_EMAIL=CHANGE_ME_ADMIN_EMAIL
ADMIN_BOOTSTRAP_PASSWORD=CHANGE_ME_STRONG_ADMIN_PASSWORD
ADMIN_BOOTSTRAP_USERNAME=superadmin

# ─── MONITORING ───────────────────────────────────────────────
GRAFANA_PASSWORD=CHANGE_ME_GRAFANA_PASSWORD
PROMETHEUS_RETENTION=30d
LOKI_URL=http://loki:3100

# ─── CORS ─────────────────────────────────────────────────────
CORS_ORIGINS=https://space-conquest.gg,https://admin.space-conquest.gg

# ─── SÉCURITÉ ─────────────────────────────────────────────────
# Rate limiting global (req/s par IP avant blocage)
RATE_LIMIT_GLOBAL=200
RATE_LIMIT_AUTH=10

# ─── BACKUPS ──────────────────────────────────────────────────
# Rclone remote pour backup S3/R2
BACKUP_REMOTE=r2:space-conquest-backups
BACKUP_RETENTION_DAYS=30
```

---

## Docker Compose — Développement

### `docker-compose.dev.yml`

```yaml
name: spaceconquest-dev

services:

  # ─── BASE DE DONNÉES ────────────────────────────────────────
  postgres:
    image: postgres:16-alpine
    restart: unless-stopped
    env_file: .env.dev
    environment:
      POSTGRES_DB:       ${DB_NAME}
      POSTGRES_USER:     ${DB_USER}
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    ports:
      - "5432:5432"          # Exposé localement pour accès DBeaver/psql
    volumes:
      - pgdata_dev:/var/lib/postgresql/data
      - ./scripts/init-db.sql:/docker-entrypoint-initdb.d/init.sql:ro
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${DB_USER} -d ${DB_NAME}"]
      interval: 5s
      timeout: 5s
      retries: 10

  # ─── REDIS ──────────────────────────────────────────────────
  redis:
    image: valkey/valkey:7-alpine
    restart: unless-stopped
    ports:
      - "6379:6379"          # Exposé localement pour debug Redis CLI
    command: valkey-server --save 30 1 --maxmemory 256mb --maxmemory-policy allkeys-lru
    volumes:
      - redisdata_dev:/data
    healthcheck:
      test: ["CMD", "valkey-cli", "ping"]
      interval: 5s
      retries: 5

  # ─── BACKEND (hot-reload) ───────────────────────────────────
  backend:
    build:
      context: ./backend
      dockerfile: Dockerfile.dev
    restart: unless-stopped
    env_file: .env.dev
    environment:
      DATABASE_URL: ${DATABASE_URL}
      REDIS_URL:    ${REDIS_URL}
    ports:
      - "8080:8080"          # API REST + WebSocket
      - "9229:9229"          # Debug port (optionnel)
    volumes:
      - ./backend/src:/app/src                  # Hot-reload sources
      - ./game_data:/app/game_data              # Game data live-reload
      - cargo_cache:/usr/local/cargo/registry   # Cache Cargo
      - target_cache:/app/target                # Cache build
    depends_on:
      postgres: { condition: service_healthy }
      redis:    { condition: service_healthy }
    command: cargo watch -x 'run -- --mode=api'

  # ─── WORKER (hot-reload) ────────────────────────────────────
  worker:
    build:
      context: ./backend
      dockerfile: Dockerfile.dev
    restart: unless-stopped
    env_file: .env.dev
    environment:
      DATABASE_URL: ${DATABASE_URL}
      REDIS_URL:    ${REDIS_URL}
    volumes:
      - ./backend/src:/app/src
      - ./game_data:/app/game_data
      - cargo_cache:/usr/local/cargo/registry
      - target_cache:/app/target
    depends_on:
      postgres: { condition: service_healthy }
      redis:    { condition: service_healthy }
      backend:  { condition: service_started }
    command: cargo watch -x 'run -- --mode=worker'

  # ─── FRONTEND (Next.js dev server) ──────────────────────────
  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile.dev
    restart: unless-stopped
    env_file: .env.dev
    ports:
      - "3000:3000"
    volumes:
      - ./frontend/src:/app/src
      - ./frontend/app:/app/app
      - ./frontend/components:/app/components
      - ./frontend/public:/app/public
      - /app/node_modules                       # Ne pas monter node_modules host
      - /app/.next                              # Cache Next.js dans le conteneur
    environment:
      NEXT_PUBLIC_API_URL: ${NEXT_PUBLIC_API_URL}
      NEXT_PUBLIC_WS_URL:  ${NEXT_PUBLIC_WS_URL}
      WATCHPACK_POLLING: "true"                 # Hot-reload dans Docker sur Windows/Mac
    depends_on:
      - backend

  # ─── ADMIN PANEL (Next.js dev server) ───────────────────────
  admin:
    build:
      context: ./admin
      dockerfile: Dockerfile.dev
    restart: unless-stopped
    env_file: .env.dev
    ports:
      - "3001:3001"
    volumes:
      - ./admin/src:/app/src
      - ./admin/app:/app/app
      - ./admin/components:/app/components
      - /app/node_modules
    environment:
      PORT: "3001"
      NEXT_PUBLIC_ADMIN_API_URL: ${NEXT_PUBLIC_ADMIN_API_URL}
    depends_on:
      - backend

  # ─── MONITORING ─────────────────────────────────────────────
  prometheus:
    image: prom/prometheus:latest
    restart: unless-stopped
    ports:
      - "9090:9090"
    volumes:
      - ./monitoring/prometheus.dev.yml:/etc/prometheus/prometheus.yml:ro
      - prometheus_data_dev:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.retention.time=${PROMETHEUS_RETENTION:-7d}'

  grafana:
    image: grafana/grafana:latest
    restart: unless-stopped
    ports:
      - "3002:3000"
    volumes:
      - grafana_data_dev:/var/lib/grafana
      - ./monitoring/grafana/provisioning:/etc/grafana/provisioning:ro
      - ./monitoring/grafana/dashboards:/var/lib/grafana/dashboards:ro
    environment:
      GF_SECURITY_ADMIN_PASSWORD: ${GRAFANA_PASSWORD:-grafana_dev}
      GF_USERS_ALLOW_SIGN_UP: "false"
    depends_on:
      - prometheus

  # ─── MAILHOG (catch-all emails en dev) ──────────────────────
  mailhog:
    image: mailhog/mailhog:latest
    restart: unless-stopped
    ports:
      - "1025:1025"          # SMTP
      - "8025:8025"          # Interface web

volumes:
  pgdata_dev:
  redisdata_dev:
  cargo_cache:
  target_cache:
  prometheus_data_dev:
  grafana_data_dev:

networks:
  default:
    name: sc_dev_network
```

---

## Docker Compose — Production (Coolify)

### `docker-compose.prod.yml`

```yaml
name: spaceconquest-prod

services:

  # ─── BASE DE DONNÉES ────────────────────────────────────────
  postgres:
    image: postgres:16-alpine
    restart: always
    env_file: .env.prod
    environment:
      POSTGRES_DB:       ${DB_NAME}
      POSTGRES_USER:     ${DB_USER}
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - pgdata_prod:/var/lib/postgresql/data
      - ./scripts/init-db.sql:/docker-entrypoint-initdb.d/init.sql:ro
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${DB_USER} -d ${DB_NAME}"]
      interval: 10s
      timeout: 5s
      retries: 10
    # Pas de port exposé publiquement
    networks:
      - internal

  # ─── PGBOUNCER (connection pooling) ─────────────────────────
  pgbouncer:
    image: bitnami/pgbouncer:latest
    restart: always
    environment:
      POSTGRESQL_HOST:     postgres
      POSTGRESQL_PORT:     5432
      POSTGRESQL_DATABASE: ${DB_NAME}
      POSTGRESQL_USERNAME: ${DB_USER}
      POSTGRESQL_PASSWORD: ${DB_PASSWORD}
      PGBOUNCER_POOL_MODE: transaction
      PGBOUNCER_MAX_CLIENT_CONN: 1000
      PGBOUNCER_DEFAULT_POOL_SIZE: 50
    depends_on:
      postgres: { condition: service_healthy }
    networks:
      - internal

  # ─── REDIS ──────────────────────────────────────────────────
  redis:
    image: valkey/valkey:7-alpine
    restart: always
    command: >
      valkey-server
      --requirepass ${REDIS_PASSWORD}
      --save 60 1
      --maxmemory 512mb
      --maxmemory-policy allkeys-lru
      --loglevel warning
    volumes:
      - redisdata_prod:/data
    healthcheck:
      test: ["CMD", "valkey-cli", "-a", "${REDIS_PASSWORD}", "ping"]
      interval: 10s
      retries: 5
    networks:
      - internal

  # ─── BACKEND ────────────────────────────────────────────────
  backend:
    build:
      context: ./backend
      dockerfile: Dockerfile.prod
    restart: always
    env_file: .env.prod
    environment:
      DATABASE_URL: ${PGBOUNCER_URL}
      REDIS_URL:    ${REDIS_URL}
      ENV:          production
    volumes:
      - ./game_data:/app/game_data:ro
    depends_on:
      postgres:  { condition: service_healthy }
      pgbouncer: { condition: service_started }
      redis:     { condition: service_healthy }
    networks:
      - internal
      - public
    labels:
      - "traefik.enable=true"
      - "traefik.docker.network=sc_prod_public"
      # API REST + WebSocket
      - "traefik.http.routers.sc-api.rule=Host(`${API_DOMAIN}`)"
      - "traefik.http.routers.sc-api.entrypoints=websecure"
      - "traefik.http.routers.sc-api.tls.certresolver=letsencrypt"
      - "traefik.http.services.sc-api.loadbalancer.server.port=8080"
      # Sticky sessions pour WebSocket (si plusieurs instances backend)
      - "traefik.http.services.sc-api.loadbalancer.sticky.cookie=true"
    deploy:
      replicas: 1          # Passer à 2+ quand la charge le justifie
      resources:
        limits:
          cpus: '2'
          memory: 1G

  # ─── WORKER ─────────────────────────────────────────────────
  worker:
    build:
      context: ./backend
      dockerfile: Dockerfile.prod
    restart: always
    env_file: .env.prod
    environment:
      DATABASE_URL: ${DATABASE_URL}  # Worker accède directement à Postgres (pas PgBouncer)
      REDIS_URL:    ${REDIS_URL}
      ENV:          production
    command: ["/app/space-conquest", "--mode=worker"]
    volumes:
      - ./game_data:/app/game_data:ro
    depends_on:
      postgres: { condition: service_healthy }
      redis:    { condition: service_healthy }
    networks:
      - internal
    deploy:
      replicas: 1          # TOUJOURS 1 seul worker par univers
      resources:
        limits:
          cpus: '1'
          memory: 512M

  # ─── FRONTEND ───────────────────────────────────────────────
  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile.prod
      args:
        NEXT_PUBLIC_API_URL: ${NEXT_PUBLIC_API_URL}
        NEXT_PUBLIC_WS_URL:  ${NEXT_PUBLIC_WS_URL}
        NEXT_PUBLIC_ENV:     production
    restart: always
    env_file: .env.prod
    networks:
      - internal
      - public
    labels:
      - "traefik.enable=true"
      - "traefik.docker.network=sc_prod_public"
      - "traefik.http.routers.sc-web.rule=Host(`${DOMAIN}`)"
      - "traefik.http.routers.sc-web.entrypoints=websecure"
      - "traefik.http.routers.sc-web.tls.certresolver=letsencrypt"
      - "traefik.http.services.sc-web.loadbalancer.server.port=3000"
      # Redirect HTTP → HTTPS
      - "traefik.http.routers.sc-web-http.rule=Host(`${DOMAIN}`)"
      - "traefik.http.routers.sc-web-http.entrypoints=web"
      - "traefik.http.routers.sc-web-http.middlewares=redirect-https"
    depends_on:
      - backend

  # ─── ADMIN PANEL (accessible via Tailscale uniquement) ──────
  admin:
    build:
      context: ./admin
      dockerfile: Dockerfile.prod
      args:
        NEXT_PUBLIC_ADMIN_API_URL: ${NEXT_PUBLIC_ADMIN_API_URL}
        NEXT_PUBLIC_ENV:           production
    restart: always
    env_file: .env.prod
    networks:
      - internal
      - public
    labels:
      - "traefik.enable=true"
      - "traefik.docker.network=sc_prod_public"
      - "traefik.http.routers.sc-admin.rule=Host(`${ADMIN_DOMAIN}`)"
      - "traefik.http.routers.sc-admin.entrypoints=websecure"
      - "traefik.http.routers.sc-admin.tls.certresolver=letsencrypt"
      - "traefik.http.services.sc-admin.loadbalancer.server.port=3000"
      # Restriction IP Tailscale uniquement
      - "traefik.http.routers.sc-admin.middlewares=tailscale-only"
      - "traefik.http.middlewares.tailscale-only.ipallowlist.sourcerange=${ADMIN_TAILSCALE_IP}/32,100.64.0.0/10"
    depends_on:
      - backend

  # ─── TRAEFIK ────────────────────────────────────────────────
  traefik:
    image: traefik:v3
    restart: always
    command:
      - "--log.level=INFO"
      - "--api.dashboard=false"
      - "--providers.docker=true"
      - "--providers.docker.exposedbydefault=false"
      - "--providers.docker.network=sc_prod_public"
      - "--entrypoints.web.address=:80"
      - "--entrypoints.websecure.address=:443"
      - "--entrypoints.web.http.redirections.entrypoint.to=websecure"
      - "--entrypoints.web.http.redirections.entrypoint.scheme=https"
      - "--certificatesresolvers.letsencrypt.acme.httpchallenge=true"
      - "--certificatesresolvers.letsencrypt.acme.httpchallenge.entrypoint=web"
      - "--certificatesresolvers.letsencrypt.acme.email=${ACME_EMAIL}"
      - "--certificatesresolvers.letsencrypt.acme.storage=/letsencrypt/acme.json"
      # Headers de sécurité globaux
      - "--entrypoints.websecure.http.middlewares=security-headers"
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - traefik_certs:/letsencrypt
    networks:
      - public
    labels:
      # Middleware redirect HTTP→HTTPS
      - "traefik.http.middlewares.redirect-https.redirectscheme.scheme=https"
      - "traefik.http.middlewares.redirect-https.redirectscheme.permanent=true"
      # Headers de sécurité
      - "traefik.http.middlewares.security-headers.headers.stsSeconds=31536000"
      - "traefik.http.middlewares.security-headers.headers.stsIncludeSubdomains=true"
      - "traefik.http.middlewares.security-headers.headers.contentTypeNosniff=true"
      - "traefik.http.middlewares.security-headers.headers.frameDeny=true"

  # ─── MONITORING ─────────────────────────────────────────────
  prometheus:
    image: prom/prometheus:latest
    restart: always
    volumes:
      - ./monitoring/prometheus.prod.yml:/etc/prometheus/prometheus.yml:ro
      - prometheus_data_prod:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.retention.time=${PROMETHEUS_RETENTION:-30d}'
      - '--web.enable-lifecycle'
    networks:
      - internal

  grafana:
    image: grafana/grafana:latest
    restart: always
    volumes:
      - grafana_data_prod:/var/lib/grafana
      - ./monitoring/grafana/provisioning:/etc/grafana/provisioning:ro
      - ./monitoring/grafana/dashboards:/var/lib/grafana/dashboards:ro
    environment:
      GF_SECURITY_ADMIN_PASSWORD: ${GRAFANA_PASSWORD}
      GF_USERS_ALLOW_SIGN_UP: "false"
      GF_SERVER_ROOT_URL: "https://grafana.${DOMAIN}"
    networks:
      - internal
      - public
    labels:
      - "traefik.enable=true"
      - "traefik.docker.network=sc_prod_public"
      - "traefik.http.routers.sc-grafana.rule=Host(`grafana.${DOMAIN}`)"
      - "traefik.http.routers.sc-grafana.entrypoints=websecure"
      - "traefik.http.routers.sc-grafana.tls.certresolver=letsencrypt"
      - "traefik.http.routers.sc-grafana.middlewares=tailscale-only"
    depends_on:
      - prometheus

  loki:
    image: grafana/loki:latest
    restart: always
    command: -config.file=/etc/loki/local-config.yaml
    volumes:
      - ./monitoring/loki.prod.yml:/etc/loki/local-config.yaml:ro
      - loki_data_prod:/loki
    networks:
      - internal

  # ─── BACKUP AUTOMATIQUE ─────────────────────────────────────
  backup:
    image: alpine:3.19
    restart: always
    env_file: .env.prod
    environment:
      DB_HOST:     postgres
      DB_NAME:     ${DB_NAME}
      DB_USER:     ${DB_USER}
      DB_PASSWORD: ${DB_PASSWORD}
    volumes:
      - ./scripts/db-backup.sh:/backup.sh:ro
      - backup_data:/backups
      - ./rclone.conf:/root/.config/rclone/rclone.conf:ro
    depends_on:
      postgres: { condition: service_healthy }
    # Backup quotidien à 3h du matin
    command: >
      sh -c "while true; do
        sleep $$(( $(date -d 'tomorrow 03:00' +%s) - $(date +%s) ));
        sh /backup.sh;
        sleep 86400;
      done"
    networks:
      - internal

volumes:
  pgdata_prod:
  redisdata_prod:
  prometheus_data_prod:
  grafana_data_prod:
  loki_data_prod:
  traefik_certs:
  backup_data:

networks:
  internal:
    name: sc_prod_internal
    internal: true           # Aucun accès internet direct pour les services internes
  public:
    name: sc_prod_public
```

---

## Dockerfiles

### `backend/Dockerfile.dev`

```dockerfile
FROM rust:1.82-slim-bookworm

# Dépendances système
RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev libpq-dev \
    curl git \
    && rm -rf /var/lib/apt/lists/*

# cargo-watch pour hot-reload
RUN cargo install cargo-watch

# sqlx-cli pour les migrations
RUN cargo install sqlx-cli --no-default-features --features postgres

WORKDIR /app

# Copier les fichiers de dépendances en premier (cache layer)
COPY Cargo.toml Cargo.lock ./

# Pré-compiler les dépendances (couche cachée)
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build 2>/dev/null || true
RUN rm -rf src

# Le code source est monté en volume (hot-reload)
# CMD est défini dans docker-compose.dev.yml

EXPOSE 8080
```

---

### `backend/Dockerfile.prod`

```dockerfile
# ── Stage 1 : Chef (compilateur) ──────────────────────────────
FROM rust:1.82-slim-bookworm AS chef
RUN cargo install cargo-chef
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# ── Stage 2 : Planner (analyse des dépendances) ───────────────
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ── Stage 3 : Builder (compilation) ───────────────────────────
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Compiler les dépendances (cachées si recipe.json n'a pas changé)
RUN cargo chef cook --release --recipe-path recipe.json

# Compiler l'application
COPY . .
RUN cargo build --release

# ── Stage 4 : Runtime (image finale minimale) ─────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    libssl3 libpq5 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Utilisateur non-root
RUN useradd -r -s /bin/false -u 1001 appuser

WORKDIR /app

COPY --from=builder /app/target/release/space-conquest /app/space-conquest
COPY --chown=appuser:appuser migrations/ /app/migrations/

# game_data monté en volume
RUN mkdir -p /app/game_data && chown appuser:appuser /app/game_data

USER appuser

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=10s --start-period=15s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

CMD ["/app/space-conquest", "--mode=api"]
```

---

### `frontend/Dockerfile.dev`

```dockerfile
FROM node:22-alpine

RUN apk add --no-cache libc6-compat

WORKDIR /app

# Installer pnpm
RUN corepack enable && corepack prepare pnpm@latest --activate

# Copier les dépendances
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

# Le code source est monté en volume
EXPOSE 3000

CMD ["pnpm", "dev"]
```

---

### `frontend/Dockerfile.prod`

```dockerfile
# ── Stage 1 : Dependencies ─────────────────────────────────────
FROM node:22-alpine AS deps
RUN apk add --no-cache libc6-compat
WORKDIR /app
RUN corepack enable && corepack prepare pnpm@latest --activate
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile --prod

# ── Stage 2 : Builder ─────────────────────────────────────────
FROM node:22-alpine AS builder
WORKDIR /app
RUN corepack enable && corepack prepare pnpm@latest --activate
COPY --from=deps /app/node_modules ./node_modules
COPY . .

# Variables d'environnement build-time
ARG NEXT_PUBLIC_API_URL
ARG NEXT_PUBLIC_WS_URL
ARG NEXT_PUBLIC_ENV

ENV NEXT_PUBLIC_API_URL=$NEXT_PUBLIC_API_URL
ENV NEXT_PUBLIC_WS_URL=$NEXT_PUBLIC_WS_URL
ENV NEXT_PUBLIC_ENV=$NEXT_PUBLIC_ENV
ENV NEXT_TELEMETRY_DISABLED=1

RUN pnpm build

# ── Stage 3 : Runner ──────────────────────────────────────────
FROM node:22-alpine AS runner
WORKDIR /app

RUN addgroup --system --gid 1001 nodejs \
    && adduser  --system --uid 1001 nextjs

ENV NODE_ENV=production
ENV NEXT_TELEMETRY_DISABLED=1

# Next.js standalone output
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static
COPY --from=builder --chown=nextjs:nodejs /app/public ./public

USER nextjs

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=10s --retries=3 \
    CMD wget -qO- http://localhost:3000/ || exit 1

CMD ["node", "server.js"]
```

> ⚠️ Ajouter `output: 'standalone'` dans `next.config.ts` pour activer le mode standalone.

---

## Configuration Coolify

### Stratégie de Déploiement

Coolify gère le déploiement via le fichier `docker-compose.prod.yml`. Configuration dans l'interface Coolify :

```
Project     : Space Conquest
Environment : Production

Source      : GitHub repo (main branch)
Compose file: docker-compose.prod.yml
Env file    : .env.prod (uploadé manuellement via Coolify Secrets)

Build & Deploy:
  - Auto-deploy on push to main : ✅
  - Health check before swap    : ✅
  - Zero-downtime deploy        : ✅ (rolling update)
```

### Secrets dans Coolify

Dans l'interface Coolify → Project → Environment → Secrets, ajouter **chaque variable** de `.env.prod` individuellement. Ne jamais committer `.env.prod`.

### Variables Build-time vs Runtime

```
Build-time (ARG dans Dockerfile, passées au moment du docker build) :
  NEXT_PUBLIC_API_URL
  NEXT_PUBLIC_WS_URL
  NEXT_PUBLIC_ENV

Runtime (disponibles au démarrage du conteneur, via env_file) :
  DATABASE_URL, REDIS_URL, JWT_SECRET, etc.
```

### Réseau Coolify

Coolify crée automatiquement un réseau `coolify` partagé. Pour que Traefik Coolify (déjà présent sur le VPS) détecte les conteneurs :

```yaml
# Ajouter à chaque service exposé dans docker-compose.prod.yml :
networks:
  - coolify          # Réseau Traefik de Coolify
  - internal         # Réseau interne du projet

# Déclarer le réseau externe Coolify en bas du compose :
networks:
  coolify:
    external: true
  internal:
    name: sc_prod_internal
    internal: true
```

### Labels Traefik pour Coolify

Remplacer les labels Traefik manuels par la convention Coolify si Traefik est géré par Coolify :

```yaml
labels:
  - "traefik.enable=true"
  - "traefik.http.routers.sc-api.rule=Host(`api.space-conquest.gg`)"
  - "traefik.http.routers.sc-api.entrypoints=https"
  - "traefik.http.routers.sc-api.tls=true"
  - "traefik.http.routers.sc-api.tls.certresolver=letsencrypt"
  - "traefik.http.services.sc-api.loadbalancer.server.port=8080"
```

---

## Scripts Utilitaires

### `scripts/dev-setup.sh`

```bash
#!/usr/bin/env bash
set -euo pipefail

echo "🚀 Space Conquest — Setup Environnement Dev"
echo "============================================"

# Vérifier les prérequis
command -v docker    >/dev/null || { echo "❌ Docker requis"; exit 1; }
command -v docker compose >/dev/null || { echo "❌ Docker Compose requis"; exit 1; }

# Copier les fichiers .env si absents
if [ ! -f .env.dev ]; then
    cp .env.dev.example .env.dev
    echo "✅ .env.dev créé depuis .env.dev.example"
    echo "⚠️  Modifiez .env.dev si nécessaire, puis relancez ce script"
fi

# Démarrer les services infrastructure
echo ""
echo "📦 Démarrage PostgreSQL & Redis..."
docker compose -f docker-compose.dev.yml --env-file .env.dev up -d postgres redis

# Attendre que Postgres soit prêt
echo "⏳ Attente de PostgreSQL..."
until docker compose -f docker-compose.dev.yml exec postgres pg_isready -U sc_dev -d spaceconquest_dev 2>/dev/null; do
    sleep 1
done
echo "✅ PostgreSQL prêt"

# Lancer les migrations
echo ""
echo "🗃️  Application des migrations..."
docker compose -f docker-compose.dev.yml --env-file .env.dev run --rm backend \
    sqlx migrate run

# Seeder la base de données de dev
echo ""
echo "🌱 Seeding de la base de données..."
docker compose -f docker-compose.dev.yml --env-file .env.dev run --rm backend \
    cargo run --bin seed -- --env dev

# Installer les dépendances frontend
echo ""
echo "📦 Installation des dépendances frontend..."
docker compose -f docker-compose.dev.yml --env-file .env.dev run --rm frontend \
    pnpm install

docker compose -f docker-compose.dev.yml --env-file .env.dev run --rm admin \
    pnpm install

echo ""
echo "✅ Setup terminé !"
echo ""
echo "Pour démarrer l'environnement complet :"
echo "  make dev"
echo ""
echo "Services disponibles :"
echo "  Frontend  : http://localhost:3000"
echo "  Admin     : http://localhost:3001"
echo "  API       : http://localhost:8080"
echo "  Swagger   : http://localhost:8080/swagger-ui"
echo "  Grafana   : http://localhost:3002"
echo "  Mailhog   : http://localhost:8025"
echo "  PgAdmin   : psql -h localhost -U sc_dev -d spaceconquest_dev"
```

---

### `scripts/db-migrate.sh`

```bash
#!/usr/bin/env bash
set -euo pipefail

ENV="${1:-dev}"  # Usage: ./db-migrate.sh [dev|prod]

if [ "$ENV" = "prod" ]; then
    COMPOSE_FILE="docker-compose.prod.yml"
    ENV_FILE=".env.prod"
    echo "⚠️  Migration PRODUCTION — Êtes-vous sûr ? (Ctrl+C pour annuler)"
    sleep 3
else
    COMPOSE_FILE="docker-compose.dev.yml"
    ENV_FILE=".env.dev"
fi

echo "🗃️  Application des migrations ($ENV)..."
docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" run --rm backend \
    sqlx migrate run

echo "✅ Migrations appliquées"
```

---

### `scripts/db-seed.sh`

```bash
#!/usr/bin/env bash
set -euo pipefail

echo "🌱 Seeding base de données dev..."
echo "  → Création de l'univers de dev"
echo "  → Création de 3 comptes de test"
echo "  → Bootstrap du compte superadmin"

docker compose -f docker-compose.dev.yml --env-file .env.dev run --rm backend \
    cargo run --bin seed -- \
        --universe-id "00000000-0000-0000-0000-000000000001" \
        --universe-name "Dev Universe Alpha" \
        --test-players 3 \
        --admin-email "admin@localhost" \
        --admin-password "AdminDev123!"

echo ""
echo "✅ Seed terminé"
echo ""
echo "Comptes de test créés :"
echo "  player1@test.com / Password123!"
echo "  player2@test.com / Password123!"
echo "  player3@test.com / Password123!"
echo "  admin@localhost  / AdminDev123!  [SUPERADMIN]"
```

---

### `scripts/db-backup.sh`

```bash
#!/usr/bin/env bash
set -euo pipefail

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="/backups/spaceconquest_${TIMESTAMP}.sql.gz"

echo "💾 Backup PostgreSQL — $TIMESTAMP"

# Dump compressé
PGPASSWORD="$DB_PASSWORD" pg_dump \
    -h "$DB_HOST" \
    -U "$DB_USER" \
    -d "$DB_NAME" \
    --no-owner \
    --no-acl \
    | gzip > "$BACKUP_FILE"

echo "✅ Backup créé : $BACKUP_FILE ($(du -sh "$BACKUP_FILE" | cut -f1))"

# Upload vers R2/S3 via rclone
if command -v rclone >/dev/null 2>&1 && [ -n "${BACKUP_REMOTE:-}" ]; then
    echo "☁️  Upload vers $BACKUP_REMOTE..."
    rclone copy "$BACKUP_FILE" "$BACKUP_REMOTE/$(date +%Y/%m/)/"
    echo "✅ Upload terminé"
fi

# Nettoyage des anciens backups locaux (garder 7 jours)
find /backups -name "*.sql.gz" -mtime +7 -delete
echo "🧹 Anciens backups nettoyés (>7j)"

# Lister les backups existants
echo ""
echo "📁 Backups disponibles :"
ls -lh /backups/*.sql.gz 2>/dev/null || echo "  (aucun)"
```

---

### `scripts/db-restore.sh`

```bash
#!/usr/bin/env bash
set -euo pipefail

BACKUP_FILE="${1:-}"

if [ -z "$BACKUP_FILE" ]; then
    echo "Usage: ./db-restore.sh <backup_file.sql.gz>"
    echo ""
    echo "Backups disponibles :"
    ls -lh /backups/*.sql.gz 2>/dev/null || echo "  (aucun)"
    exit 1
fi

echo "⚠️  RESTAURATION depuis : $BACKUP_FILE"
echo "    Base cible : $DB_NAME@$DB_HOST"
echo "    Cette opération va ÉCRASER toutes les données existantes !"
echo ""
read -p "Confirmer ? (yes/no) " CONFIRM
[ "$CONFIRM" = "yes" ] || { echo "Annulé"; exit 0; }

echo "🔄 Restauration en cours..."

# Drop & recreate
PGPASSWORD="$DB_PASSWORD" psql -h "$DB_HOST" -U "$DB_USER" -c "DROP DATABASE IF EXISTS ${DB_NAME};"
PGPASSWORD="$DB_PASSWORD" psql -h "$DB_HOST" -U "$DB_USER" -c "CREATE DATABASE ${DB_NAME};"

# Restore
gunzip -c "$BACKUP_FILE" | PGPASSWORD="$DB_PASSWORD" psql \
    -h "$DB_HOST" \
    -U "$DB_USER" \
    -d "$DB_NAME"

echo "✅ Restauration terminée"
```

---

## Makefile

```makefile
# ═══════════════════════════════════════════════════════════════
#  SPACE CONQUEST — Makefile
# ═══════════════════════════════════════════════════════════════

.PHONY: help dev dev-build dev-down dev-logs prod-build \
        migrate migrate-prod seed logs-backend logs-frontend \
        backup restore test lint clean

# Détection de l'OS pour les commandes cross-platform
UNAME := $(shell uname)

# Couleurs terminal
CYAN  := \033[36m
GREEN := \033[32m
YELLOW := \033[33m
RED   := \033[31m
RESET := \033[0m

# ─── AIDE ───────────────────────────────────────────────────────
help: ## Afficher cette aide
	@echo ""
	@echo "$(CYAN)SPACE CONQUEST — Commandes disponibles$(RESET)"
	@echo "════════════════════════════════════════"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-20s$(RESET) %s\n", $$1, $$2}'
	@echo ""

# ─── DÉVELOPPEMENT ──────────────────────────────────────────────
setup: ## Premier lancement : copie .env, migrations, seed
	@chmod +x scripts/*.sh
	@./scripts/dev-setup.sh

dev: ## Démarrer l'environnement dev complet
	@echo "$(CYAN)🚀 Démarrage Space Conquest Dev...$(RESET)"
	docker compose -f docker-compose.dev.yml --env-file .env.dev up

dev-d: ## Démarrer en arrière-plan
	docker compose -f docker-compose.dev.yml --env-file .env.dev up -d
	@echo "$(GREEN)✅ Services démarrés en arrière-plan$(RESET)"
	@echo "  Frontend  : http://localhost:3000"
	@echo "  Admin     : http://localhost:3001"
	@echo "  API       : http://localhost:8080"
	@echo "  Swagger   : http://localhost:8080/swagger-ui"
	@echo "  Grafana   : http://localhost:3002  (admin/grafana_dev)"
	@echo "  Mailhog   : http://localhost:8025"

dev-build: ## Rebuild les images dev
	docker compose -f docker-compose.dev.yml --env-file .env.dev build

dev-down: ## Arrêter l'environnement dev
	docker compose -f docker-compose.dev.yml --env-file .env.dev down

dev-clean: ## Arrêter et supprimer les volumes dev
	@echo "$(RED)⚠️  Suppression des données dev...$(RESET)"
	docker compose -f docker-compose.dev.yml --env-file .env.dev down -v

# ─── SERVICES INDIVIDUELS ───────────────────────────────────────
up-infra: ## Démarrer uniquement postgres + redis
	docker compose -f docker-compose.dev.yml --env-file .env.dev up -d postgres redis

up-backend: ## Démarrer backend + infra
	docker compose -f docker-compose.dev.yml --env-file .env.dev up -d postgres redis backend worker

up-frontend: ## Démarrer frontend seulement
	docker compose -f docker-compose.dev.yml --env-file .env.dev up -d frontend

restart-backend: ## Redémarrer le backend
	docker compose -f docker-compose.dev.yml --env-file .env.dev restart backend worker

# ─── BASE DE DONNÉES ────────────────────────────────────────────
migrate: ## Appliquer les migrations (dev)
	@./scripts/db-migrate.sh dev

migrate-prod: ## Appliquer les migrations (prod)
	@./scripts/db-migrate.sh prod

migrate-new: ## Créer une nouvelle migration
	@read -p "Nom de la migration (ex: add_notifications): " NAME; \
	docker compose -f docker-compose.dev.yml --env-file .env.dev run --rm backend \
		sqlx migrate add $$NAME

seed: ## Seeder la base dev
	@./scripts/db-seed.sh

db-shell: ## Accéder au shell PostgreSQL dev
	docker compose -f docker-compose.dev.yml --env-file .env.dev exec postgres \
		psql -U sc_dev -d spaceconquest_dev

redis-shell: ## Accéder au shell Redis dev
	docker compose -f docker-compose.dev.yml --env-file .env.dev exec redis \
		valkey-cli

# ─── LOGS ───────────────────────────────────────────────────────
logs: ## Logs de tous les services
	docker compose -f docker-compose.dev.yml --env-file .env.dev logs -f

logs-backend: ## Logs backend uniquement
	docker compose -f docker-compose.dev.yml --env-file .env.dev logs -f backend

logs-worker: ## Logs worker uniquement
	docker compose -f docker-compose.dev.yml --env-file .env.dev logs -f worker

logs-frontend: ## Logs frontend uniquement
	docker compose -f docker-compose.dev.yml --env-file .env.dev logs -f frontend

# ─── PRODUCTION ─────────────────────────────────────────────────
prod-build: ## Builder les images prod
	docker compose -f docker-compose.prod.yml --env-file .env.prod build

prod-up: ## Démarrer en prod (Coolify gère normalement ceci)
	docker compose -f docker-compose.prod.yml --env-file .env.prod up -d

prod-down: ## Arrêter la prod
	docker compose -f docker-compose.prod.yml --env-file .env.prod down

prod-logs: ## Logs prod
	docker compose -f docker-compose.prod.yml --env-file .env.prod logs -f

backup: ## Backup manuel de la base prod
	@./scripts/db-backup.sh

restore: ## Restaurer un backup
	@./scripts/db-restore.sh

# ─── QUALITÉ DE CODE ────────────────────────────────────────────
test: ## Lancer les tests Rust
	docker compose -f docker-compose.dev.yml --env-file .env.dev run --rm backend \
		cargo test

test-watch: ## Tests en mode watch
	docker compose -f docker-compose.dev.yml --env-file .env.dev run --rm backend \
		cargo watch -x test

lint: ## Linter Rust + frontend
	docker compose -f docker-compose.dev.yml --env-file .env.dev run --rm backend \
		cargo clippy -- -D warnings
	docker compose -f docker-compose.dev.yml --env-file .env.dev run --rm frontend \
		pnpm lint

format: ## Formatter le code
	docker compose -f docker-compose.dev.yml --env-file .env.dev run --rm backend \
		cargo fmt
	docker compose -f docker-compose.dev.yml --env-file .env.dev run --rm frontend \
		pnpm format

# ─── GAME DATA ──────────────────────────────────────────────────
validate-gamedata: ## Valider tous les JSON game_data
	docker compose -f docker-compose.dev.yml --env-file .env.dev run --rm backend \
		cargo run --bin validate-gamedata

# ─── UTILITAIRES ────────────────────────────────────────────────
ps: ## Statut des conteneurs dev
	docker compose -f docker-compose.dev.yml --env-file .env.dev ps

clean: ## Nettoyer les images Docker non utilisées
	docker image prune -f
	docker volume prune -f

generate-secrets: ## Générer des secrets sécurisés pour .env.prod
	@echo "$(CYAN)Secrets générés pour .env.prod :$(RESET)"
	@echo "JWT_SECRET=$(shell openssl rand -hex 64)"
	@echo "DB_PASSWORD=$(shell openssl rand -base64 32 | tr -d '/+=' | head -c 32)"
	@echo "REDIS_PASSWORD=$(shell openssl rand -base64 24 | tr -d '/+=')"
	@echo "GRAFANA_PASSWORD=$(shell openssl rand -base64 16 | tr -d '/+=')"
	@echo "DEFAULT_UNIVERSE_ID=$(shell cat /proc/sys/kernel/random/uuid 2>/dev/null || uuidgen)"
```

---

## `.gitignore`

```gitignore
# Environnements
.env.dev
.env.prod
.env.local
.env*.local

# Garder les exemples
!.env.dev.example
!.env.prod.example

# Rust
backend/target/
backend/.sqlx/

# Node
frontend/node_modules/
frontend/.next/
admin/node_modules/
admin/.next/

# Certificats & secrets
*.pem
*.key
rclone.conf
acme.json

# Backups locaux
scripts/backups/
*.sql.gz

# Logs
*.log
logs/

# IDE
.idea/
.vscode/
*.swp
.DS_Store
```

---

## Résumé des Ports (Dev)

| Service | Port | URL |
|---------|-----:|-----|
| Frontend | 3000 | http://localhost:3000 |
| Admin Panel | 3001 | http://localhost:3001 |
| Backend API / WS | 8080 | http://localhost:8080 |
| Swagger UI | 8080 | http://localhost:8080/swagger-ui |
| Grafana | 3002 | http://localhost:3002 |
| Prometheus | 9090 | http://localhost:9090 |
| Mailhog SMTP | 1025 | — |
| Mailhog UI | 8025 | http://localhost:8025 |
| PostgreSQL | 5432 | localhost:5432 (DBeaver/psql) |
| Redis | 6379 | localhost:6379 |

---

*— FIN DE L'ADDENDUM v1.3 —*  
*SPACE CONQUEST 4X // INFRASTRUCTURE & DÉPLOIEMENT*
