# SPACE CONQUEST — ADDENDUM v1.2
> Extension du Master Document v1.1  
> Nouvelles sections : Interface Admin · Succès & Missions · Sénat Galactique

---

## TABLE DES MATIÈRES

- [21. Interface d'Administration](#21-interface-dadministration)
  - [21.1 Vue d'ensemble & Accès](#211-vue-densemble--accès)
  - [21.2 Architecture Admin Backend](#212-architecture-admin-backend)
  - [21.3 Gestion des Utilisateurs & Empires](#213-gestion-des-utilisateurs--empires)
  - [21.4 Éditeur GameData Live](#214-éditeur-gamedata-live)
  - [21.5 Surveillance des Univers & Événements](#215-surveillance-des-univers--événements)
  - [21.6 Outils de Modération](#216-outils-de-modération)
  - [21.7 Statistiques & Analytics](#217-statistiques--analytics)
  - [21.8 Gestion du Sénat (Admin side)](#218-gestion-du-sénat-admin-side)
  - [21.9 Routes API Admin](#219-routes-api-admin)
  - [21.10 Frontend Admin — Structure](#2110-frontend-admin--structure)
  - [21.11 Schémas SQL Admin](#2111-schémas-sql-admin)
- [22. Système de Succès & Missions](#22-système-de-succès--missions)
  - [22.1 Principes & Architecture](#221-principes--architecture)
  - [22.2 Schéma JSON Succès](#222-schéma-json-succès)
  - [22.3 Schéma JSON Mission](#223-schéma-json-mission)
  - [22.4 Conditions (Trigger Engine)](#224-conditions-trigger-engine)
  - [22.5 Récompenses](#225-récompenses)
  - [22.6 Catalogue de Succès](#226-catalogue-de-succès)
  - [22.7 Catalogue de Missions](#227-catalogue-de-missions)
  - [22.8 Schémas SQL](#228-schémas-sql)
  - [22.9 Backend — Modules Rust](#229-backend--modules-rust)
- [23. Sénat Galactique (Système de Vote)](#23-sénat-galactique-système-de-vote)
  - [23.1 Concept & Mécaniques](#231-concept--mécaniques)
  - [23.2 Types de Propositions](#232-types-de-propositions)
  - [23.3 Schéma JSON Proposition](#233-schéma-json-proposition)
  - [23.4 Règles de Vote](#234-règles-de-vote)
  - [23.5 Effets Actifs (Buffs/Debuffs)](#235-effets-actifs-buffsdebuffs)
  - [23.6 Catalogue de Propositions Prédéfinies](#236-catalogue-de-propositions-prédéfinies)
  - [23.7 Schémas SQL](#237-schémas-sql)
  - [23.8 Backend — Modules Rust](#238-backend--modules-rust)
  - [23.9 Routes API Sénat](#239-routes-api-sénat)

---

# 21. Interface d'Administration

## 21.1 Vue d'ensemble & Accès

Le panel admin est une **application Next.js séparée** (`/admin`) — jamais bundlée avec le frontend joueur. Elle tourne sur un sous-domaine dédié (`admin.space-conquest.gg`) accessible uniquement via **Tailscale** (réseau privé). Aucun accès public.

### Niveaux d'Accès (Rôles Admin)

| Rôle | Description | Permissions |
|------|-------------|-------------|
| `SUPERADMIN` | Fondateur / DevOps | Tout, y compris gestion des autres admins |
| `ADMIN` | Administrateur principal | Gestion utilisateurs, univers, game data, sénat |
| `MODERATOR` | Modérateur communautaire | Ban/warn utilisateurs, lecture logs, gestion rapports |
| `GAME_MASTER` | GM événements | Création événements PvE manuels, cadeaux de ressources, gestion sénat |
| `ANALYST` | Lecture seule | Statistiques, dashboards, logs — zéro action |

Les rôles sont **cumulatifs** : un `SUPERADMIN` hérite de toutes les permissions des rôles inférieurs.

### Authentification Admin

- Compte admin **distinct** du compte joueur (table `admin_users` séparée)
- **MFA obligatoire** (TOTP) pour tous les rôles `ADMIN` et supérieurs
- Session admin : JWT 4h non-renewable (re-login forcé)
- Audit log **immuable** : toute action admin est loggée avec `admin_id`, timestamp, IP, payload avant/après

---

## 21.2 Architecture Admin Backend

### Structure des Modules Admin

```
backend/src/
├── admin/
│   ├── mod.rs
│   ├── middleware/
│   │   ├── auth_admin.rs          # JWT admin + vérif rôle
│   │   └── audit_log.rs           # Middleware d'audit automatique
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── users.rs               # CRUD utilisateurs & empires
│   │   ├── universe.rs            # Gestion univers, tick, reset
│   │   ├── gamedata.rs            # Éditeur live game data
│   │   ├── events.rs              # Gestion event_queue manuelle
│   │   ├── moderation.rs          # Ban, warn, chat logs
│   │   ├── stats.rs               # Statistiques & analytics
│   │   ├── senate.rs              # Gestion propositions sénat
│   │   ├── achievements.rs        # Gestion succès & missions
│   │   └── broadcast.rs           # Messages système / maintenances
│   └── services/
│       ├── audit_service.rs       # Écriture audit log
│       ├── stats_service.rs       # Agrégation métriques
│       └── gamedata_service.rs    # Hot-reload game data
```

### Middleware d'Audit Automatique

```rust
// admin/middleware/audit_log.rs
// Wrappé autour de toutes les routes admin mutantes (POST, PUT, DELETE, PATCH)
pub async fn audit_middleware<B>(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let method     = request.method().clone();
    let path       = request.uri().path().to_string();
    let body_bytes = /* buffer body */;

    let response   = next.run(request).await;

    // Toujours logger, même en cas d'erreur
    audit_service::log(&state.db, AuditEntry {
        admin_id:    admin.id,
        admin_role:  admin.role,
        method:      method.to_string(),
        path,
        request_body: body_bytes,
        status_code: response.status().as_u16(),
        ip:          admin.ip,
        created_at:  Utc::now(),
    }).await;

    response
}
```

---

## 21.3 Gestion des Utilisateurs & Empires

### Fonctionnalités

| Action | Rôle Minimum | Description |
|--------|:------------:|-------------|
| Lister / Rechercher utilisateurs | `ANALYST` | Filtres : username, email, universe, ban status, date |
| Voir profil complet (empires, planètes, logs) | `MODERATOR` | Vue 360° d'un joueur |
| Modifier `display_name` | `MODERATOR` | Nettoyage noms inappropriés |
| Ban temporaire / permanent | `MODERATOR` | Avec raison obligatoire |
| Unban | `ADMIN` | Log de la décision |
| Ajouter / retirer Dark Matter | `GAME_MASTER` | Compensation, événements |
| Modifier ressources d'une planète | `GAME_MASTER` | Compensation technique |
| Téléporter une flotte | `ADMIN` | Résolution de bugs de vol bloqué |
| Supprimer un empire | `ADMIN` | Soft-delete avec archive |
| Supprimer un compte | `SUPERADMIN` | Hard-delete RGPD (cascade) |
| Gérer les rôles admin | `SUPERADMIN` | Attribution / révocation |

### Exemple de Vue Profil Joueur (Admin)

```
┌─────────────────────────────────────────────────────────┐
│  PROFIL : nicolas#7421  [MODERATOR VIEW]                │
├───────────────────────┬─────────────────────────────────┤
│  username: nicolas    │  Univers : Andromeda-3           │
│  display: "fs0c13ty" │  Empire : "Terran Hegemony"      │
│  email: ***@***       │  Points : 1,240,500 (Rang #3)    │
│  Inscrit : 2025-01-12 │  Dark Matter : 42,500            │
│  Dernière co. : 2h    │  Planètes : 7                    │
├───────────────────────┴─────────────────────────────────┤
│  ACTIONS RAPIDES                                         │
│  [Warn] [Ban 7j] [Ajouter DM] [Voir Logs] [Flottes]    │
├─────────────────────────────────────────────────────────┤
│  HISTORIQUE SANCTIONS   │  RAPPORTS COMBAT (derniers 10) │
│  Aucune sanction        │  15 victoires / 3 défaites     │
└─────────────────────────────────────────────────────────┘
```

---

## 21.4 Éditeur GameData Live

L'éditeur permet de modifier les fichiers JSON de configuration **sans redémarrer le serveur**. Le backend expose un `GameDataRegistry` partagé via `Arc<RwLock<...>>` — une écriture sur le registry propage immédiatement les nouvelles valeurs à tous les workers.

### Fonctionnalités

| Fonction | Description |
|----------|-------------|
| **Lister** toutes les entités | Vue tabulaire filtrée par catégorie (ships, buildings, techs, defenses, npcs) |
| **Éditer** inline (JSON editor) | Éditeur JSON Monaco avec validation de schéma en temps réel |
| **Diff avant/après** | Vue diff côte-à-côte avant confirmation |
| **Hot-reload** | Application immédiate sans redémarrage (via `Arc<RwLock>`) |
| **Versionning** | Chaque modification crée une version → possibilité de rollback |
| **Validation** | Le backend rejette tout JSON qui ne respecte pas le schéma Rust |
| **Import/Export** | Import d'un ZIP de configs complet, export en archive |
| **Prévisualisation** | Simulation des coûts à chaque niveau avant publication |

### Hot-Reload Architecture

```rust
// gamedata/registry.rs
pub struct GameDataRegistry {
    pub ships:      HashMap<String, ShipConfig>,
    pub buildings:  HashMap<String, BuildingConfig>,
    pub techs:      HashMap<String, TechConfig>,
    pub defenses:   HashMap<String, DefenseConfig>,
    pub achievements: HashMap<String, AchievementConfig>,
    pub missions:   HashMap<String, MissionConfig>,
    pub senate_props: HashMap<String, SenatePropTemplate>,
}

// Partagé via AppState
pub type SharedRegistry = Arc<RwLock<GameDataRegistry>>;

// admin/services/gamedata_service.rs
pub async fn update_entity(
    registry: &SharedRegistry,
    db: &PgPool,
    category: &str,
    id: &str,
    new_json: Value,
) -> Result<(), AppError> {
    // 1. Valider le JSON contre le schéma Rust
    let validated = validate_against_schema(category, &new_json)?;
    
    // 2. Persister en DB (table gamedata_overrides)
    persist_override(db, category, id, &new_json).await?;
    
    // 3. Écrire dans le registre partagé (propagation immédiate)
    let mut registry = registry.write().await;
    registry.apply_update(category, id, validated)?;
    
    Ok(())
}
```

### Table de Versionning

```sql
CREATE TABLE gamedata_versions (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    admin_id    UUID REFERENCES admin_users(id),
    category    TEXT NOT NULL,              -- ships | buildings | techs | ...
    entity_id   TEXT NOT NULL,
    prev_json   JSONB,
    new_json    JSONB NOT NULL,
    note        TEXT,                       -- Raison de la modification
    created_at  TIMESTAMPTZ DEFAULT NOW()
);
```

---

## 21.5 Surveillance des Univers & Événements

### Dashboard Univers en Temps Réel

```
┌──────────────────────────────────────────────────────────────┐
│  UNIVERS : Andromeda-3  [LIVE]                               │
├──────────────┬──────────────┬──────────────┬────────────────┤
│  Joueurs     │  Connexions  │  Event Queue │  Worker Status │
│  2,847       │  412 online  │  1,204 items │  ✅ Running    │
│  actifs      │  (+12% vs h) │  lag: 0.2s  │  Lock: ✅ held │
├──────────────┴──────────────┴──────────────┴────────────────┤
│  MÉTRIQUES (30 dernières minutes)                            │
│  API latency p95: 42ms  │  DB queries/s: 1,204             │
│  WS connections: 412    │  Events processed/min: 8,400      │
├─────────────────────────────────────────────────────────────┤
│  EVENT QUEUE (live)                                          │
│  ● FLEET_ARRIVAL  fleet:abc123  → 00:04:12  [COMBAT]        │
│  ● BUILD_COMPLETE planet:xyz456 → 00:18:45  [NORMAL]        │
│  ● NPC_ATTACK     planet:def789 → 01:02:11  [PRIORITY]      │
└─────────────────────────────────────────────────────────────┘
```

### Actions sur l'Univers

| Action | Rôle | Description |
|--------|:----:|-------------|
| Pause univers | `ADMIN` | Suspend le worker, aucun event traité |
| Résume univers | `ADMIN` | Reprend le worker |
| Purge event queue | `SUPERADMIN` | Vide la queue (maintenance critique) |
| Annuler un event spécifique | `ADMIN` | Supprimer un event par ID |
| Injecter un event manuel | `GAME_MASTER` | Déclencher une attaque PvE, une expédition... |
| Reset d'un univers | `SUPERADMIN` | Archive + réinitialise toutes les données |
| Maintenance programmée | `ADMIN` | Annonce + fermeture progressive |
| Broadcast message système | `ADMIN` | Message global visible tous les joueurs |

---

## 21.6 Outils de Modération

### File de Rapports Joueurs

```
┌──────────────────────────────────────────────────────────────┐
│  RAPPORTS OUVERTS (12)                                        │
├───────┬──────────────┬──────────────┬───────────┬───────────┤
│  ID   │  Rapporteur  │  Cible       │  Type     │  Priorité │
├───────┼──────────────┼──────────────┼───────────┼───────────┤
│  #441 │  player_a    │  player_b    │  CHEATING │  🔴 HIGH  │
│  #440 │  player_c    │  player_d    │  INSULT   │  🟡 MED   │
│  #439 │  player_e    │  player_f    │  MULTIBOT │  🔴 HIGH  │
└───────┴──────────────┴──────────────┴───────────┴───────────┘
```

### Sanctions Disponibles

| Sanction | Durée | Effets |
|----------|-------|--------|
| `WARN` | Permanent (log) | Avertissement visible dans le profil |
| `CHAT_MUTE` | 1h / 24h / 7j | Désactive la messagerie alliance & diplomatie |
| `BAN_TEMP` | 1j / 7j / 30j | Connexion bloquée, flotte en pause |
| `BAN_PERM` | Permanent | Connexion bloquée, archivage empire |
| `RESOURCE_WIPE` | Unique | Remise à zéro des ressources (triche économique) |
| `POINTS_RESET` | Unique | Remise à zéro des points classement |

Toute sanction nécessite une **raison obligatoire** et notifie le joueur par email.

---

## 21.7 Statistiques & Analytics

### Vue Globale (Dashboard ANALYST)

```
Métriques disponibles — tous filtrables par univers & période :

ÉCONOMIE
├── Production moyenne métal/cristal/deutérium par joueur/h
├── Volume total échangé sur le marché P2P
├── Ressources pillées totales / pillages par jour
└── Distribution des ressources par percentile (top 1% vs médiane)

COMBAT
├── Nombre de combats PvP / PvE par jour
├── Taux de victoire attaquant/défenseur
├── Vaisseaux produits vs détruits (heatmap temporelle)
├── Top 10 des flottes les plus destructrices
└── Champs de débris créés / récoltés

PROGRESSION
├── Distribution des niveaux de bâtiments (histogramme)
├── Technologies les plus recherchées
├── Temps moyen pour atteindre chaque milestone
└── Courbe de rétention : D1 / D7 / D30

ENGAGEMENT
├── Connexions par heure (courbe 24h)
├── Sessions actives simultanées (peak)
├── Actions par session (clics, dispatches)
└── Taux d'abandon à chaque étape (funnel)

SÉNAT
├── Taux de participation aux votes
├── Propositions acceptées vs rejetées
├── Effets actifs et leur impact mesuré
└── Distribution des votes par tier de joueur

SUCCÈS
├── Succès les plus débloqués vs les plus rares
├── Taux de complétion des missions actives
└── Dark Matter distribué via succès/missions
```

### Exports

- Export CSV/Excel pour toutes les vues
- Rapports automatiques hebdomadaires par email (admins abonnés)
- Accès direct à Grafana pour les métriques temps réel (Prometheus)

---

## 21.8 Gestion du Sénat (Admin side)

Les admins peuvent :

| Action | Rôle | Description |
|--------|:----:|-------------|
| Créer une proposition admin | `GAME_MASTER` | Hors template — proposition custom |
| Forcer la clôture d'un vote | `ADMIN` | Clôture anticipée (urgence) |
| Annuler un effet actif | `ADMIN` | Rollback d'un buff/debuff |
| Modifier les paramètres du sénat | `ADMIN` | Durée de vote, quorum, fréquence |
| Voir l'historique complet | `ANALYST` | Tous les votes, résultats, effets appliqués |
| Créer des templates de propositions | `ADMIN` | Ajout dans `game_data/senate_proposals/` |

---

## 21.9 Routes API Admin

Toutes les routes admin sont préfixées `/admin/api/` et protégées par le middleware `AdminAuth`.

### Utilisateurs

| Méthode | Route | Rôle Min. | Description |
|---------|-------|:---------:|-------------|
| `GET` | `/admin/api/users` | `ANALYST` | Liste avec filtres avancés |
| `GET` | `/admin/api/users/:id` | `MODERATOR` | Profil complet 360° |
| `PATCH` | `/admin/api/users/:id/display-name` | `MODERATOR` | Modifier nom affiché |
| `POST` | `/admin/api/users/:id/ban` | `MODERATOR` | Bannir |
| `DELETE` | `/admin/api/users/:id/ban` | `ADMIN` | Lever le ban |
| `POST` | `/admin/api/users/:id/dark-matter` | `GAME_MASTER` | Ajouter/retirer DM |
| `POST` | `/admin/api/users/:id/warn` | `MODERATOR` | Avertissement formel |
| `DELETE` | `/admin/api/users/:id` | `SUPERADMIN` | Suppression RGPD |

### GameData

| Méthode | Route | Rôle Min. | Description |
|---------|-------|:---------:|-------------|
| `GET` | `/admin/api/gamedata` | `ANALYST` | Toutes les entités par catégorie |
| `GET` | `/admin/api/gamedata/:category/:id` | `ANALYST` | Entité spécifique + historique versions |
| `PUT` | `/admin/api/gamedata/:category/:id` | `ADMIN` | Mettre à jour (hot-reload) |
| `POST` | `/admin/api/gamedata/:category` | `ADMIN` | Créer une nouvelle entité |
| `DELETE` | `/admin/api/gamedata/:category/:id` | `SUPERADMIN` | Supprimer une entité |
| `POST` | `/admin/api/gamedata/:category/:id/rollback/:version` | `ADMIN` | Rollback vers version précédente |
| `GET` | `/admin/api/gamedata/validate` | `ADMIN` | Valider tous les JSON |

### Univers & Modération

| Méthode | Route | Rôle Min. | Description |
|---------|-------|:---------:|-------------|
| `GET` | `/admin/api/universes` | `ANALYST` | Liste des univers + métriques |
| `GET` | `/admin/api/universes/:id/live` | `ANALYST` | Dashboard temps réel (SSE) |
| `POST` | `/admin/api/universes/:id/pause` | `ADMIN` | Pause le worker |
| `POST` | `/admin/api/universes/:id/resume` | `ADMIN` | Reprend le worker |
| `POST` | `/admin/api/universes/:id/broadcast` | `ADMIN` | Message global |
| `GET` | `/admin/api/events` | `ANALYST` | Inspection event_queue |
| `DELETE` | `/admin/api/events/:id` | `ADMIN` | Supprimer un event |
| `POST` | `/admin/api/events/inject` | `GAME_MASTER` | Injecter un event manuel |
| `GET` | `/admin/api/reports` | `MODERATOR` | File de rapports joueurs |
| `POST` | `/admin/api/reports/:id/resolve` | `MODERATOR` | Clore un rapport |

### Statistiques

| Méthode | Route | Rôle Min. | Description |
|---------|-------|:---------:|-------------|
| `GET` | `/admin/api/stats/overview` | `ANALYST` | KPIs globaux |
| `GET` | `/admin/api/stats/economy` | `ANALYST` | Métriques économiques |
| `GET` | `/admin/api/stats/combat` | `ANALYST` | Métriques combats |
| `GET` | `/admin/api/stats/engagement` | `ANALYST` | Rétention & engagement |
| `GET` | `/admin/api/stats/senate` | `ANALYST` | Stats sénat |
| `GET` | `/admin/api/audit-log` | `ADMIN` | Historique actions admin |
| `GET` | `/admin/api/stats/export/:report` | `ANALYST` | Export CSV |

---

## 21.10 Frontend Admin — Structure

L'interface admin est une **SPA Next.js dédiée**, isolée du frontend joueur.

```
admin/
├── app/
│   ├── layout.tsx                     # Shell admin (nav sombre + Tailwind)
│   ├── login/page.tsx                 # Login admin + TOTP
│   ├── dashboard/page.tsx             # Vue d'ensemble KPIs
│   ├── users/
│   │   ├── page.tsx                   # Liste & recherche utilisateurs
│   │   └── [id]/page.tsx              # Profil complet
│   ├── universes/
│   │   ├── page.tsx                   # Liste des univers
│   │   └── [id]/
│   │       ├── page.tsx               # Dashboard univers (SSE live)
│   │       └── events/page.tsx        # Inspection event_queue
│   ├── gamedata/
│   │   ├── page.tsx                   # Vue tabulaire toutes entités
│   │   └── [category]/[id]/page.tsx   # Éditeur Monaco + diff
│   ├── moderation/
│   │   ├── reports/page.tsx           # File de rapports
│   │   └── sanctions/page.tsx         # Historique sanctions
│   ├── stats/
│   │   ├── overview/page.tsx
│   │   ├── economy/page.tsx
│   │   ├── combat/page.tsx
│   │   └── engagement/page.tsx
│   ├── senate/
│   │   ├── page.tsx                   # Propositions actives/historique
│   │   └── create/page.tsx            # Création proposition admin
│   └── audit/page.tsx                 # Audit log immuable
├── components/
│   ├── DataTable.tsx                  # Table paginée avec filtres
│   ├── MonacoEditor.tsx               # Éditeur JSON + validation schéma
│   ├── DiffViewer.tsx                 # Diff JSON avant/après
│   ├── LiveMetrics.tsx                # Charts temps réel (SSE)
│   ├── SanctionModal.tsx              # Modal de sanction
│   └── AuditLogViewer.tsx
└── lib/
    ├── admin-api.ts                   # Client API admin typé
    └── permissions.ts                 # Helpers vérif rôles côté UI
```

### Composants Clés

**DataTable** : table universelle avec tri multi-colonnes, filtres dynamiques, export CSV intégré, pagination serveur.

**MonacoEditor** : intégration de l'éditeur VS Code dans le navigateur. Schéma JSON chargé dynamiquement depuis `/admin/api/gamedata/schema/:category` pour la validation inline.

**LiveMetrics** : graphiques Recharts alimentés par Server-Sent Events (SSE) depuis `/admin/api/universes/:id/live`. Latence < 1s.

---

## 21.11 Schémas SQL Admin

```sql
-- Comptes administrateurs (table séparée des users joueurs)
CREATE TABLE admin_users (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email         TEXT UNIQUE NOT NULL,
    username      TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role          TEXT NOT NULL DEFAULT 'ANALYST',
    totp_secret   TEXT,
    totp_enabled  BOOLEAN DEFAULT FALSE,
    is_active     BOOLEAN DEFAULT TRUE,
    last_login_at TIMESTAMPTZ,
    created_by    UUID REFERENCES admin_users(id),
    created_at    TIMESTAMPTZ DEFAULT NOW()
);

-- Audit log immuable (INSERT ONLY via trigger — UPDATE/DELETE interdits)
CREATE TABLE admin_audit_log (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    admin_id     UUID REFERENCES admin_users(id),
    admin_role   TEXT NOT NULL,
    method       TEXT NOT NULL,
    path         TEXT NOT NULL,
    request_body JSONB,
    status_code  SMALLINT,
    target_type  TEXT,             -- user | empire | planet | gamedata | event...
    target_id    TEXT,
    ip           INET NOT NULL,
    created_at   TIMESTAMPTZ DEFAULT NOW()
);

-- Rendre le log immuable
CREATE RULE no_update_audit AS ON UPDATE TO admin_audit_log DO INSTEAD NOTHING;
CREATE RULE no_delete_audit AS ON DELETE TO admin_audit_log DO INSTEAD NOTHING;

-- Versionning des game data
CREATE TABLE gamedata_versions (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    admin_id    UUID REFERENCES admin_users(id),
    category    TEXT NOT NULL,
    entity_id   TEXT NOT NULL,
    version     INT NOT NULL,
    prev_json   JSONB,
    new_json    JSONB NOT NULL,
    note        TEXT,
    created_at  TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_gamedata_versions_entity ON gamedata_versions(category, entity_id, version DESC);

-- Overrides live (persiste les modifications hot-reload)
CREATE TABLE gamedata_overrides (
    category    TEXT NOT NULL,
    entity_id   TEXT NOT NULL,
    json_data   JSONB NOT NULL,
    updated_at  TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (category, entity_id)
);

-- Sanctions joueurs
CREATE TABLE player_sanctions (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID REFERENCES users(id),
    admin_id    UUID REFERENCES admin_users(id),
    type        TEXT NOT NULL,        -- WARN | CHAT_MUTE | BAN_TEMP | BAN_PERM | RESOURCE_WIPE
    reason      TEXT NOT NULL,
    expires_at  TIMESTAMPTZ,          -- NULL = permanent
    is_active   BOOLEAN DEFAULT TRUE,
    revoked_by  UUID REFERENCES admin_users(id),
    revoked_at  TIMESTAMPTZ,
    created_at  TIMESTAMPTZ DEFAULT NOW()
);

-- Rapports joueurs
CREATE TABLE player_reports (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reporter_id  UUID REFERENCES users(id),
    target_id    UUID REFERENCES users(id),
    type         TEXT NOT NULL,       -- CHEATING | INSULT | MULTIBOT | SPAM | OTHER
    description  TEXT,
    evidence     JSONB,               -- Screenshots, rapport de combat, etc.
    status       TEXT DEFAULT 'OPEN', -- OPEN | IN_REVIEW | RESOLVED | DISMISSED
    resolved_by  UUID REFERENCES admin_users(id),
    resolution   TEXT,
    created_at   TIMESTAMPTZ DEFAULT NOW()
);
```

---

# 22. Système de Succès & Missions

## 22.1 Principes & Architecture

Les succès et missions sont entièrement **data-driven** : définis en JSON dans `game_data/achievements/` et `game_data/missions/`. Le backend les charge dans le `GameDataRegistry` au démarrage.

**Différences clés :**

| | Succès (Achievement) | Mission |
|--|---------------------|---------|
| **Durée** | Permanent (jamais expiré) | Temporaire (fenêtre de temps) |
| **Déclenchement** | Automatique (conditions) | Acceptation manuelle ou automatique |
| **Visibilité** | Public (profil joueur) | Privé (tableau de bord joueur) |
| **Progression** | Simple ou multi-étapes | Étapes avec narration |
| **Récompenses** | Dark Matter, titre, cosmétique | Ressources, DM, vaisseaux |
| **Répétabilité** | Non (une seule fois) | Certaines missions répétables |

### Trigger Engine

Le système de déclenchement est un **moteur de conditions** évalué à chaque événement pertinent. Les conditions sont évaluées de façon asynchrone, en batch, après chaque action significative du joueur.

```rust
// services/achievement_service.rs
pub async fn evaluate_triggers(
    pool: &PgPool,
    empire_id: Uuid,
    trigger_event: TriggerEvent,  // Ex: FleetArrived, BuildingCompleted...
    registry: &SharedRegistry,
) -> Result<Vec<UnlockedAchievement>, AppError> {
    let registry = registry.read().await;
    let empire_state = get_empire_state(pool, empire_id).await?;

    let mut unlocked = vec![];

    for (id, ach) in &registry.achievements {
        // Ignorer si déjà débloqué
        if is_already_unlocked(pool, empire_id, id).await? { continue; }

        // Vérifier si l'événement est pertinent pour ce succès
        if !ach.triggers.contains(&trigger_event.kind()) { continue; }

        // Évaluer toutes les conditions
        if evaluate_conditions(&empire_state, &ach.conditions) {
            unlock_achievement(pool, empire_id, id, &ach).await?;
            unlocked.push(ach.clone().into());
        }
    }

    Ok(unlocked)
}
```

---

## 22.2 Schéma JSON Succès

```json
// game_data/achievements/first_blood.json
{
  "id": "ach_first_blood",
  "name": { "fr": "Premier Sang", "en": "First Blood" },
  "description": { "fr": "Remportez votre premier combat PvP.", "en": "Win your first PvP battle." },
  "icon": "icon_ach_sword",
  "category": "COMBAT",
  "rarity": "COMMON",
  "points": 10,
  "hidden": false,
  "triggers": ["COMBAT_RESOLVED"],
  "conditions": [
    {
      "type": "STAT_GTE",
      "stat": "pvp_victories",
      "value": 1
    }
  ],
  "rewards": [
    { "type": "DARK_MATTER", "amount": 500 },
    { "type": "TITLE",       "title_id": "title_warrior" }
  ],
  "stages": null
}
```

### Succès Multi-Étapes

```json
// game_data/achievements/fleet_admiral.json
{
  "id": "ach_fleet_admiral",
  "name": { "fr": "Amiral de Flotte", "en": "Fleet Admiral" },
  "category": "FLEET",
  "rarity": "LEGENDARY",
  "points": 100,
  "hidden": false,
  "triggers": ["FLEET_DISPATCHED", "COMBAT_RESOLVED"],
  "conditions": null,
  "stages": [
    {
      "stage": 1,
      "label": { "fr": "Capitaine", "en": "Captain" },
      "conditions": [{ "type": "STAT_GTE", "stat": "total_fleet_dispatches", "value": 10 }],
      "rewards": [{ "type": "DARK_MATTER", "amount": 1000 }]
    },
    {
      "stage": 2,
      "label": { "fr": "Commodore", "en": "Commodore" },
      "conditions": [{ "type": "STAT_GTE", "stat": "total_fleet_dispatches", "value": 100 }],
      "rewards": [{ "type": "DARK_MATTER", "amount": 5000 }]
    },
    {
      "stage": 3,
      "label": { "fr": "Amiral", "en": "Admiral" },
      "conditions": [{ "type": "STAT_GTE", "stat": "total_fleet_dispatches", "value": 1000 }],
      "rewards": [
        { "type": "DARK_MATTER", "amount": 25000 },
        { "type": "TITLE",       "title_id": "title_fleet_admiral" },
        { "type": "COSMETIC",    "cosmetic_id": "skin_admiral_flagship" }
      ]
    }
  ]
}
```

---

## 22.3 Schéma JSON Mission

```json
// game_data/missions/daily_miner.json
{
  "id": "mission_daily_miner",
  "name":        { "fr": "Extracteur du Jour", "en": "Daily Miner" },
  "description": { "fr": "Produisez 500,000 unités de métal en 24h.", "en": "Produce 500,000 metal in 24h." },
  "icon":        "icon_mission_mine",
  "category":    "ECONOMY",
  "type":        "DAILY",
  "repeatable":  true,
  "auto_accept": true,
  "duration_hours": 24,
  "requirements": [],
  "objectives": [
    {
      "id": "obj_produce_metal",
      "type": "PRODUCE_RESOURCE",
      "resource": "metal",
      "amount": 500000,
      "label": { "fr": "Produire 500 000 métal", "en": "Produce 500,000 metal" }
    }
  ],
  "rewards": [
    { "type": "RESOURCE", "resource": "crystal",      "amount": 50000 },
    { "type": "RESOURCE", "resource": "deuterium",    "amount": 10000 },
    { "type": "DARK_MATTER", "amount": 200 }
  ]
}
```

### Mission Scénarisée (Mandate)

```json
// game_data/missions/ancient_ruins.json
{
  "id": "mission_ancient_ruins",
  "name":        { "fr": "Ruines Anciennes", "en": "Ancient Ruins" },
  "description": { "fr": "Des signatures énergétiques anormales ont été détectées dans la ceinture d'Orion-X.", "en": "..." },
  "icon":        "icon_ruins",
  "category":    "EXPEDITION",
  "type":        "STORY",
  "repeatable":  false,
  "auto_accept": false,
  "duration_hours": 168,
  "requirements": [
    { "type": "tech", "id": "tech_astrophysics", "level": 3 }
  ],
  "objectives": [
    {
      "id": "obj_send_expedition",
      "type": "DISPATCH_FLEET",
      "mission": "EXPEDITION",
      "ships_required": [{ "ship_id": "ship_pathfinder", "min_quantity": 1 }],
      "label": { "fr": "Envoyer une flotte en expédition dans la ceinture Orion-X" }
    },
    {
      "id": "obj_survive_combat",
      "type": "WIN_COMBAT",
      "against": "NPC",
      "count": 1,
      "label": { "fr": "Neutraliser les gardiens de la ruine" },
      "depends_on": "obj_send_expedition"
    }
  ],
  "rewards": [
    { "type": "RESOURCE",    "resource": "metal",    "amount": 2000000 },
    { "type": "RESOURCE",    "resource": "crystal",  "amount": 1000000 },
    { "type": "DARK_MATTER", "amount": 5000 },
    { "type": "FLEET_UNIT",  "ship_id": "ship_pathfinder", "quantity": 2 },
    { "type": "ACHIEVEMENT_PROGRESS", "achievement_id": "ach_explorer", "stage_unlock": 1 }
  ]
}
```

---

## 22.4 Conditions (Trigger Engine)

| `type` | Description | Paramètres |
|--------|-------------|-----------|
| `STAT_GTE` | Statistique empire ≥ valeur | `stat`, `value` |
| `STAT_LTE` | Statistique empire ≤ valeur | `stat`, `value` |
| `BUILDING_LEVEL_GTE` | Bâtiment à un niveau ≥ N | `building_id`, `level` |
| `TECH_LEVEL_GTE` | Technologie à un niveau ≥ N | `tech_id`, `level` |
| `FLEET_SIZE_GTE` | Flotte d'au moins N vaisseaux | `count` |
| `PLANET_COUNT_GTE` | Nombre de planètes ≥ N | `count` |
| `RESOURCE_GTE` | Ressources stockées ≥ valeur | `resource`, `amount` |
| `RANKING_LTE` | Classement ≤ N (top N) | `rank` |
| `FLEET_DISPATCHED` | Mission de flotte spécifique | `mission_type` (optionnel) |
| `COMBAT_OUTCOME` | Résultat de combat | `outcome`, `against` (PVP/NPC) |
| `RESEARCH_COMPLETE` | Technologie recherchée | `tech_id` |
| `MARKET_TRADE` | Transaction marché complétée | `volume` (optionnel) |
| `SENATE_VOTE` | A voté au sénat | - |
| `ALL_OF` | Toutes les sous-conditions | `conditions: [...]` |
| `ANY_OF` | Au moins une sous-condition | `conditions: [...]` |

### Stats Trackées (table `empire_stats`)

```
pvp_victories, pvp_defeats, pve_victories
total_fleet_dispatches, total_attacks_launched
resources_mined_metal, resources_mined_crystal, resources_mined_deuterium
resources_pillaged_total, resources_lost_to_pillage
ships_produced_total, ships_destroyed_total, ships_lost_total
planets_colonized, planets_abandoned
researches_completed, buildings_upgraded
market_trades_completed, market_volume_total
senate_votes_cast
expedition_returns_success, expedition_returns_failure
dark_matter_earned_total
```

---

## 22.5 Récompenses

| `type` | Paramètres | Description |
|--------|-----------|-------------|
| `DARK_MATTER` | `amount` | Crédite directement |
| `RESOURCE` | `resource`, `amount` | Ajoute à la planète principale |
| `FLEET_UNIT` | `ship_id`, `quantity` | Spawne des vaisseaux sur la planète principale |
| `TITLE` | `title_id` | Titre cosmétique affiché sous le display_name |
| `COSMETIC` | `cosmetic_id` | Skin de vaisseau, avatar, effet de planète |
| `BUILDING_SPEEDUP` | `percent`, `duration_hours` | Accélère les constructions temporairement |
| `PRODUCTION_BONUS` | `resource`, `percent`, `duration_hours` | Boost de production temporaire |
| `ACHIEVEMENT_PROGRESS` | `achievement_id`, `stage_unlock` | Progression forcée vers une étape |

---

## 22.6 Catalogue de Succès

### Catégorie : Économie

| ID | Nom FR | Rareté | Condition | Récompense |
|----|--------|:------:|-----------|-----------|
| `ach_first_mine` | Première Extraction | Common | Mine métal Niv.5 | 250 DM |
| `ach_rich_empire` | Empire Prospère | Rare | 10M métal stocké | 2 500 DM |
| `ach_energy_lord` | Seigneur de l'Énergie | Rare | Énergie ≥ 10 000 | 2 500 DM |
| `ach_full_storage` | Hangar Comble | Common | Tous hangars à 100% | 500 DM |
| `ach_market_mogul` | Magnat du Marché | Epic | 100 trades P2P | 10 000 DM + titre |

### Catégorie : Combat

| ID | Nom FR | Rareté | Condition | Récompense |
|----|--------|:------:|-----------|-----------|
| `ach_first_blood` | Premier Sang | Common | 1 victoire PvP | 500 DM |
| `ach_warlord` | Seigneur de Guerre | Rare | 50 victoires PvP | 5 000 DM |
| `ach_grand_admiral` | Grand Amiral | Epic | 500 victoires PvP | 25 000 DM + titre |
| `ach_deathstar` | Terreur Galactique | Legendary | Construire une Étoile Noire | 50 000 DM + skin |
| `ach_pirate_hunter` | Chasseur de Pirates | Rare | 20 victoires PvE | 3 000 DM |
| `ach_no_losses` | Combat Parfait | Epic | Victoire sans pertes | 5 000 DM |

### Catégorie : Exploration

| ID | Nom FR | Rareté | Condition | Récompense |
|----|--------|:------:|-----------|-----------|
| `ach_colonist` | Colonisateur | Common | 1ère colonie | 1 000 DM |
| `ach_expander` | Bâtisseur d'Empire | Rare | 5 planètes | 5 000 DM |
| `ach_galactic_empire` | Empire Galactique | Legendary | 10 planètes | 50 000 DM + titre |
| `ach_first_expedition` | Explorateur | Common | 1ère expédition | 500 DM |
| `ach_debris_hunter` | Chasseur de Débris | Common | 10 recyclages | 1 000 DM |

### Catégorie : Recherche

| ID | Nom FR | Rareté | Condition | Récompense |
|----|--------|:------:|-----------|-----------|
| `ach_first_research` | Scientifique | Common | 1ère recherche | 250 DM |
| `ach_plasma_tech` | Maître du Plasma | Epic | Plasma Niv.1 | 10 000 DM |
| `ach_graviton` | Manipulateur de Gravité | Legendary | Tech Graviton Niv.1 | 50 000 DM + titre |
| `ach_all_techs` | Omniscient | Legendary | Toutes les techs Niv.1 | 100 000 DM + skin |

### Catégorie : Social

| ID | Nom FR | Rareté | Condition | Récompense |
|----|--------|:------:|-----------|-----------|
| `ach_diplomat` | Diplomate | Common | 1er pacte signé | 500 DM |
| `ach_senator` | Sénateur | Common | 10 votes sénat | 1 000 DM |
| `ach_alliance_founder` | Fondateur | Rare | Créer une alliance | 3 000 DM + titre |
| `ach_top10` | Élite Galactique | Epic | Top 10 classement | 25 000 DM + titre |
| `ach_top1` | Maître de l'Univers | Legendary | 1er classement | 100 000 DM + skin unique |

---

## 22.7 Catalogue de Missions

### Missions Quotidiennes (Daily — auto-accept)

| ID | Nom FR | Objectif | Récompense |
|----|--------|----------|-----------|
| `mission_daily_miner` | Extracteur du Jour | Produire 500K métal | 50K cristal + 200 DM |
| `mission_daily_trader` | Marchand du Jour | Effectuer 3 trades marché | 100K métal + 200 DM |
| `mission_daily_scout` | Éclaireur du Jour | Espionner 5 planètes | 50K deutérium + 200 DM |
| `mission_daily_builder` | Constructeur du Jour | Compléter 2 constructions | 100K cristal + 200 DM |
| `mission_daily_fighter` | Combattant du Jour | Gagner 1 combat | 100K métal + 500 DM |
| `mission_daily_vote` | Citoyen du Sénat | Voter sur 1 proposition | 50K métal + 100 DM |

### Missions Hebdomadaires (Weekly)

| ID | Nom FR | Objectif | Récompense |
|----|--------|----------|-----------|
| `mission_weekly_conquest` | Conquête de la Semaine | Gagner 10 combats PvP | 2M métal + 1M cristal + 2 000 DM |
| `mission_weekly_expansion` | Expansion | Coloniser ou améliorer 20 bâtiments | 1M métal + 500K cristal + 1 500 DM |
| `mission_weekly_research` | Percée Scientifique | Compléter 3 recherches | 500K cristal + 200K deutérium + 2 000 DM |

### Missions Scénarisées (Story — uniques)

| ID | Nom FR | Prérequis | Récompense |
|----|--------|-----------|-----------|
| `mission_ancient_ruins` | Ruines Anciennes | Astrophysique Niv.3 | 2M métal + 5 000 DM + 2 Éclaireurs |
| `mission_pirate_king` | Roi des Pirates | 20 victoires PvE | 5M métal + 10 000 DM + titre |
| `mission_first_alliance` | Naissance d'une Alliance | Aucun | 1M toutes ressources + 3 000 DM |
| `mission_graviton_hunt` | Chasse au Graviton | Lab Niv.12 | 10M métal + 5M cristal + 25 000 DM |

---

## 22.8 Schémas SQL

```sql
-- Progression des succès par empire
CREATE TABLE empire_achievements (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    empire_id      UUID REFERENCES empires(id) ON DELETE CASCADE,
    achievement_id TEXT NOT NULL,
    current_stage  SMALLINT DEFAULT 0,
    unlocked_at    TIMESTAMPTZ,
    is_complete    BOOLEAN DEFAULT FALSE,
    UNIQUE(empire_id, achievement_id)
);

-- Missions actives & historique
CREATE TABLE empire_missions (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    empire_id       UUID REFERENCES empires(id) ON DELETE CASCADE,
    mission_id      TEXT NOT NULL,
    status          TEXT DEFAULT 'ACTIVE',  -- ACTIVE | COMPLETED | FAILED | EXPIRED
    accepted_at     TIMESTAMPTZ DEFAULT NOW(),
    expires_at      TIMESTAMPTZ,
    completed_at    TIMESTAMPTZ,
    objectives_progress JSONB DEFAULT '{}' -- { "obj_id": { "current": 0, "target": 10 } }
);

CREATE INDEX idx_missions_empire_active ON empire_missions(empire_id) WHERE status = 'ACTIVE';

-- Statistiques empire (incrémentées à chaque événement)
CREATE TABLE empire_stats (
    empire_id                    UUID PRIMARY KEY REFERENCES empires(id),
    pvp_victories                BIGINT DEFAULT 0,
    pvp_defeats                  BIGINT DEFAULT 0,
    pve_victories                BIGINT DEFAULT 0,
    total_fleet_dispatches       BIGINT DEFAULT 0,
    total_attacks_launched       BIGINT DEFAULT 0,
    resources_mined_metal        NUMERIC(30,2) DEFAULT 0,
    resources_mined_crystal      NUMERIC(30,2) DEFAULT 0,
    resources_mined_deuterium    NUMERIC(30,2) DEFAULT 0,
    resources_pillaged_total     NUMERIC(30,2) DEFAULT 0,
    resources_lost_to_pillage    NUMERIC(30,2) DEFAULT 0,
    ships_produced_total         BIGINT DEFAULT 0,
    ships_destroyed_total        BIGINT DEFAULT 0,
    ships_lost_total             BIGINT DEFAULT 0,
    planets_colonized            SMALLINT DEFAULT 0,
    researches_completed         SMALLINT DEFAULT 0,
    buildings_upgraded           BIGINT DEFAULT 0,
    market_trades_completed      BIGINT DEFAULT 0,
    market_volume_total          NUMERIC(30,2) DEFAULT 0,
    senate_votes_cast            BIGINT DEFAULT 0,
    expedition_returns_success   BIGINT DEFAULT 0,
    dark_matter_earned_total     BIGINT DEFAULT 0,
    updated_at                   TIMESTAMPTZ DEFAULT NOW()
);

-- Titres & cosmétiques débloqués
CREATE TABLE empire_titles (
    empire_id   UUID REFERENCES empires(id) ON DELETE CASCADE,
    title_id    TEXT NOT NULL,
    unlocked_at TIMESTAMPTZ DEFAULT NOW(),
    is_active   BOOLEAN DEFAULT FALSE,  -- Titre actuellement affiché
    PRIMARY KEY (empire_id, title_id)
);

CREATE TABLE empire_cosmetics (
    empire_id    UUID REFERENCES empires(id) ON DELETE CASCADE,
    cosmetic_id  TEXT NOT NULL,
    unlocked_at  TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (empire_id, cosmetic_id)
);
```

---

## 22.9 Backend — Modules Rust

```
backend/src/
├── achievements/
│   ├── mod.rs
│   ├── engine.rs                  # evaluate_triggers() — moteur principal
│   ├── conditions.rs              # Évaluation de chaque type de condition
│   ├── rewards.rs                 # Distribution des récompenses
│   └── stats_tracker.rs           # Incrémentation empire_stats
├── missions/
│   ├── mod.rs
│   ├── manager.rs                 # Acceptation, expiration, reset daily/weekly
│   ├── progress_tracker.rs        # Mise à jour objectives_progress
│   └── rewards.rs                 # Distribution récompenses missions
```

### Intégration dans les workers existants

```rust
// Dans workers/combat_resolver.rs, après résolution :
achievements::engine::evaluate_triggers(
    &pool, empire_id,
    TriggerEvent::CombatResolved { outcome, against: CombatTarget::PVP },
    &registry,
).await?;

missions::progress_tracker::update(
    &pool, empire_id,
    ObjectiveEvent::WinCombat { against: CombatTarget::PVP },
).await?;

stats_tracker::increment(&pool, empire_id, StatField::PvpVictories, 1).await?;
```

---

# 23. Sénat Galactique (Système de Vote)

## 23.1 Concept & Mécaniques

Le **Sénat Galactique** est un mécanisme de démocratie participative ancré dans l'univers. Les joueurs votent pour influencer les règles temporaires du jeu — buffs de production, malus de combat, prix du marché, nouvelles mécaniques expérimentales.

### Principes

- **Un cycle de vote par semaine** (configurable par l'admin)
- **Une seule proposition active à la fois** (ou plusieurs si l'admin active le mode multi-vote)
- **Poids du vote** : égalitaire par défaut, ou pondéré par les points empire (configurable)
- **Effets temporaires** : toute décision du Sénat a une durée définie (pas de modification permanente du game balance sans intervention admin)
- **Transparence** : tous les votes sont publics (joueur → choix, pas de vote anonyme)
- **Quorum** : minimum X% des joueurs actifs doivent voter pour que la décision soit valide

### Flux d'un Cycle

```
[Lundi 00:00] Nouvelle proposition publiée
      │
[Lun → Ven]  Phase de vote (5 jours)
      │
[Vendredi 23:59] Clôture du vote
      │
[Samedi 00:00] Résultats annoncés + effet appliqué si quorum atteint
      │
[Sam → Dim]  Délibération (2 jours sans vote)
      │
[Lundi 00:00] Nouveau cycle
```

---

## 23.2 Types de Propositions

### Type A — Propositions à Choix Binaire (Oui/Non)

Le joueur vote pour ou contre. La majorité simple l'emporte.

```
Exemple :
"Activer la Taxe de Guerre Galactique ?"
→ OUI : +15% de deutérium produit par toutes les planètes pendant 7 jours
→ NON : Aucun changement
```

### Type B — Propositions à Choix Multiple

Le joueur choisit parmi 2 à 4 options. L'option avec le plus de votes l'emporte.

```
Exemple :
"Quel bonus de production cette semaine ?"
→ Option A : +20% métal (3 jours)
→ Option B : +20% cristal (3 jours)
→ Option C : +20% deutérium (3 jours)
→ Option D : +10% toutes ressources (2 jours)
```

### Type C — Propositions à Curseur (Vote Pondéré)

Le joueur attribue des points entre plusieurs options (budget de vote = 100 points).

```
Exemple :
"Répartissez le bonus hebdomadaire :"
→ Réduction temps de recherche [0–100]
→ Bonus de production minière   [0–100]
→ Réduction consommation deutérium [0–100]
(Total doit faire 100)
```

### Type D — Propositions Admin Custom

L'admin crée une proposition entièrement personnalisée avec des effets non-standards.

```
Exemple :
"Événement : L'Armada des Ombres approche. Comment répondre ?"
→ Option A : Mobilisation militaire (+30% vitesse de production vaisseaux, 5 jours)
→ Option B : Fortification défensive (+25% boucliers planétaires, 5 jours)
→ Option C : Négociation (bonus de diplomatie, pas d'effet combat)
```

---

## 23.3 Schéma JSON Proposition

```json
// game_data/senate_proposals/weekly_resource_boost.json
{
  "id": "prop_weekly_resource_boost",
  "name":        { "fr": "Bonus de Production Hebdomadaire", "en": "Weekly Production Bonus" },
  "description": { "fr": "Le Sénat vote pour le bonus de production de la semaine.", "en": "..." },
  "flavor_text": { "fr": "\"Les ressources sont le sang de l'empire.\"", "en": "..." },
  "icon":        "icon_senate_mine",
  "type":        "MULTIPLE_CHOICE",
  "vote_weight": "EQUAL",
  "quorum_percent": 15,
  "duration_days": 5,
  "effect_duration_hours": 72,
  "options": [
    {
      "id": "opt_metal",
      "label": { "fr": "+20% Métal (3 jours)", "en": "+20% Metal (3 days)" },
      "effects": [
        { "type": "PRODUCTION_BONUS", "resource": "metal", "percent": 20, "duration_hours": 72 }
      ]
    },
    {
      "id": "opt_crystal",
      "label": { "fr": "+20% Cristal (3 jours)", "en": "+20% Crystal (3 days)" },
      "effects": [
        { "type": "PRODUCTION_BONUS", "resource": "crystal", "percent": 20, "duration_hours": 72 }
      ]
    },
    {
      "id": "opt_deuterium",
      "label": { "fr": "+20% Deutérium (3 jours)", "en": "+20% Deuterium (3 days)" },
      "effects": [
        { "type": "PRODUCTION_BONUS", "resource": "deuterium", "percent": 20, "duration_hours": 72 }
      ]
    },
    {
      "id": "opt_all",
      "label": { "fr": "+10% Toutes Ressources (2 jours)", "en": "+10% All Resources (2 days)" },
      "effects": [
        { "type": "PRODUCTION_BONUS", "resource": "metal",     "percent": 10, "duration_hours": 48 },
        { "type": "PRODUCTION_BONUS", "resource": "crystal",   "percent": 10, "duration_hours": 48 },
        { "type": "PRODUCTION_BONUS", "resource": "deuterium", "percent": 10, "duration_hours": 48 }
      ]
    }
  ]
}
```

---

## 23.4 Règles de Vote

### Éligibilité

| Condition | Valeur |
|-----------|--------|
| Points minimum pour voter | 1 000 pts (configurable) |
| Votes par cycle | 1 vote par proposition |
| Modification du vote | Autorisée jusqu'à la clôture |
| Abstention | Autorisée (ne compte pas pour le quorum) |

### Poids du Vote

| Mode | Description |
|------|-------------|
| `EQUAL` | Chaque joueur = 1 vote. Démocratie directe. |
| `WEIGHTED_POINTS` | Poids = `log10(points + 1)`. Les grands empires ont légèrement plus de poids mais sans dominer. |
| `WEIGHTED_RANK` | Poids inversement proportionnel au rang (top joueurs ont plus de poids). |

### Résolution

1. Si `votes_total / joueurs_actifs < quorum` → Proposition **échouée** (aucun effet)
2. Sinon : Option avec le plus de poids l'emporte
3. En cas d'égalité exacte → Option la plus conservatrice (ou tir au sort si admin configure `tie_break: "random"`)
4. Effets appliqués via un événement `SENATE_EFFECT_START` dans l'event_queue
5. Expiration via `SENATE_EFFECT_END` schedulé automatiquement

---

## 23.5 Effets Actifs (Buffs/Debuffs)

Les effets du sénat sont stockés dans une table `active_universe_effects` et consultés à chaque calcul de production/combat.

| `type` | Paramètres | Application |
|--------|-----------|-------------|
| `PRODUCTION_BONUS` | `resource`, `percent` | Multiplicateur sur les formules de production |
| `PRODUCTION_MALUS` | `resource`, `percent` | Diviseur sur les formules de production |
| `COMBAT_ATTACK_BONUS` | `percent` | Multiplicateur sur les dégâts d'attaque |
| `COMBAT_SHIELD_BONUS` | `percent` | Multiplicateur sur les boucliers |
| `RESEARCH_SPEEDUP` | `percent` | Réduction du temps de recherche |
| `BUILD_SPEEDUP` | `percent` | Réduction du temps de construction |
| `FLIGHT_SPEEDUP` | `percent` | Réduction du temps de vol |
| `MARKET_FEE_REDUCTION` | `percent` | Réduction des commissions marché |
| `DEBRIS_MULTIPLIER` | `percent` | Plus de débris générés en combat |
| `PVP_PROTECTION` | `duration_hours` | Tous les joueurs sous protection PvP |
| `DARK_MATTER_BONUS` | `percent` | Bonus DM sur toutes les récompenses |
| `CUSTOM` | `payload: JSONB` | Effet spécial défini par l'admin (script safe) |

---

## 23.6 Catalogue de Propositions Prédéfinies

### Propositions Économiques

| ID | Nom FR | Type | Effets Possibles |
|----|--------|:----:|-----------------|
| `prop_weekly_resource_boost` | Bonus Production Hebdo | MULTIPLE_CHOICE | +20% métal OU cristal OU deutérium |
| `prop_market_regulation` | Régulation du Marché | BINARY | Réduire la commission de 5% à 2% (7j) |
| `prop_storage_event` | Crise du Stockage | MULTIPLE_CHOICE | +50% capacité stockage OU +25% prod |
| `prop_energy_crisis` | Crise Énergétique | BINARY | -20% prod énergie (malus) pendant 3j... ou payer un tribut pour l'éviter |

### Propositions Militaires

| ID | Nom FR | Type | Effets Possibles |
|----|--------|:----:|-----------------|
| `prop_arms_race` | Course aux Armements | MULTIPLE_CHOICE | -25% temps construction vaisseaux OU +15% attaque OU +15% boucliers |
| `prop_ceasefire` | Cessez-le-Feu Galactique | BINARY | PvP suspendu 48h (sauf guerres déclarées) |
| `prop_debris_law` | Loi sur les Débris | BINARY | +50% débris générés en combat (7j) |
| `prop_rapid_deployment` | Déploiement Rapide | BINARY | +25% vitesse de vol toutes flottes (3j) |

### Propositions Scientifiques

| ID | Nom FR | Type | Effets Possibles |
|----|--------|:----:|-----------------|
| `prop_research_sprint` | Sprint de Recherche | MULTIPLE_CHOICE | -30% temps recherche OU bonus DM par recherche |
| `prop_technology_sharing` | Partage Technologique | BINARY | Membres d'alliance partagent 5% de leur vitesse de recherche |

### Propositions Événementielles (Admin Custom)

| ID | Nom FR | Description |
|----|--------|-------------|
| `prop_pirate_armada` | L'Armada des Pirates | Vague PvE massive dans 48h — vote sur la stratégie de défense |
| `prop_golden_age` | Âge d'Or | Semaine de bonus généraux — vote sur le type |
| `prop_galactic_council` | Conseil Galactique | Vote sur l'ajout d'une nouvelle mécanique (feature vote) |
| `prop_season_end_reward` | Récompenses de Fin de Saison | Vote sur la distribution des prix de fin de saison |

---

## 23.7 Schémas SQL

```sql
-- Propositions du sénat (instances actives & historique)
CREATE TABLE senate_proposals (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    universe_id     UUID NOT NULL,
    template_id     TEXT,                       -- Ref game_data si créé depuis template
    name            JSONB NOT NULL,             -- {fr, en}
    description     JSONB NOT NULL,
    type            TEXT NOT NULL,              -- BINARY | MULTIPLE_CHOICE | WEIGHTED | CUSTOM
    vote_weight     TEXT DEFAULT 'EQUAL',
    quorum_percent  SMALLINT DEFAULT 15,
    options         JSONB NOT NULL,             -- Array d'options avec effets
    status          TEXT DEFAULT 'ACTIVE',      -- ACTIVE | CLOSED | CANCELLED | FAILED
    created_by      UUID,                       -- NULL = système, sinon admin_users.id
    vote_start_at   TIMESTAMPTZ NOT NULL,
    vote_end_at     TIMESTAMPTZ NOT NULL,
    effect_start_at TIMESTAMPTZ,
    effect_end_at   TIMESTAMPTZ,
    winning_option  TEXT,                       -- option id gagnante
    total_votes     INT DEFAULT 0,
    total_eligible  INT DEFAULT 0,
    created_at      TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_senate_universe_active ON senate_proposals(universe_id)
    WHERE status = 'ACTIVE';

-- Votes individuels
CREATE TABLE senate_votes (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    proposal_id UUID REFERENCES senate_proposals(id) ON DELETE CASCADE,
    empire_id   UUID REFERENCES empires(id),
    option_id   TEXT NOT NULL,
    weight      NUMERIC(10,4) DEFAULT 1.0,      -- Poids calculé selon vote_weight
    voted_at    TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(proposal_id, empire_id)
);

CREATE INDEX idx_senate_votes_proposal ON senate_votes(proposal_id);

-- Effets actifs au niveau de l'univers
CREATE TABLE active_universe_effects (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    universe_id  UUID NOT NULL,
    proposal_id  UUID REFERENCES senate_proposals(id),
    effect_type  TEXT NOT NULL,
    parameters   JSONB NOT NULL,
    starts_at    TIMESTAMPTZ NOT NULL,
    expires_at   TIMESTAMPTZ NOT NULL,
    is_active    BOOLEAN DEFAULT TRUE,
    applied_by   TEXT DEFAULT 'SENATE',         -- SENATE | ADMIN
    created_at   TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_effects_universe_active ON active_universe_effects(universe_id)
    WHERE is_active = TRUE;

-- Paramètres sénat par univers (configurables par admin)
CREATE TABLE senate_config (
    universe_id       UUID PRIMARY KEY,
    cycle_days        SMALLINT DEFAULT 7,
    vote_duration_days SMALLINT DEFAULT 5,
    quorum_default    SMALLINT DEFAULT 15,
    min_points_to_vote BIGINT DEFAULT 1000,
    vote_weight_mode  TEXT DEFAULT 'EQUAL',
    multi_vote        BOOLEAN DEFAULT FALSE,
    is_enabled        BOOLEAN DEFAULT TRUE
);
```

---

## 23.8 Backend — Modules Rust

```
backend/src/
├── senate/
│   ├── mod.rs
│   ├── handlers.rs                # Routes joueurs (/api/senate/*)
│   ├── vote_engine.rs             # Calcul poids, validation éligibilité
│   ├── result_resolver.rs         # Clôture vote + détermination gagnant
│   ├── effect_applier.rs          # Application des effets dans l'univers
│   └── scheduler.rs               # Scheduling cycles via event_queue
```

### Intégration dans les Formules de Production

```rust
// services/resource_service.rs
pub async fn calculate_production(
    pool: &PgPool,
    planet: &Planet,
    buildings: &[Building],
    techs: &[Technology],
    universe_id: Uuid,
) -> ResourceProduction {
    let base = compute_base_production(planet, buildings, techs);

    // Appliquer les effets actifs du sénat
    let effects = get_active_effects(pool, universe_id, EffectType::ProductionBonus).await?;
    let multiplier = effects.iter().fold(1.0, |acc, e| {
        if e.parameters["resource"] == planet_resource || e.parameters["resource"] == "all" {
            acc * (1.0 + e.parameters["percent"].as_f64().unwrap_or(0.0) / 100.0)
        } else {
            acc
        }
    });

    base * multiplier
}
```

### Worker — Résolution des Votes

```rust
// senate/scheduler.rs — événements dans event_queue
// "SENATE_VOTE_CLOSE"  → result_resolver.rs → effet appliqué → "SENATE_EFFECT_START"
// "SENATE_EFFECT_END"  → désactive l'effet dans active_universe_effects
// "SENATE_CYCLE_START" → publie la prochaine proposition
```

---

## 23.9 Routes API Sénat

### Routes Joueur

| Méthode | Route | Auth | Description |
|---------|-------|:----:|-------------|
| `GET` | `/api/senate/active` | ✓ | Proposition(s) en cours de vote |
| `GET` | `/api/senate/history` | ✓ | Historique des votes passés |
| `GET` | `/api/senate/effects` | ✓ | Effets actuellement actifs dans l'univers |
| `GET` | `/api/senate/proposals/:id` | ✓ | Détail d'une proposition + résultats temps réel |
| `POST` | `/api/senate/proposals/:id/vote` | ✓ | Voter (body: `{option_id}`) |
| `PUT` | `/api/senate/proposals/:id/vote` | ✓ | Modifier son vote (avant clôture) |
| `DELETE` | `/api/senate/proposals/:id/vote` | ✓ | Retirer son vote (abstention) |
| `GET` | `/api/senate/my-votes` | ✓ | Historique de mes votes |

### Routes Admin

| Méthode | Route | Rôle | Description |
|---------|-------|:----:|-------------|
| `GET` | `/admin/api/senate/proposals` | `ANALYST` | Toutes les propositions |
| `POST` | `/admin/api/senate/proposals` | `GAME_MASTER` | Créer proposition custom |
| `POST` | `/admin/api/senate/proposals/from-template` | `GAME_MASTER` | Créer depuis template |
| `POST` | `/admin/api/senate/proposals/:id/close` | `ADMIN` | Clôture forcée |
| `DELETE` | `/admin/api/senate/proposals/:id` | `ADMIN` | Annuler une proposition |
| `GET` | `/admin/api/senate/effects` | `ANALYST` | Effets actifs (tous univers) |
| `DELETE` | `/admin/api/senate/effects/:id` | `ADMIN` | Annuler un effet actif |
| `GET` | `/admin/api/senate/config/:universe_id` | `ANALYST` | Config sénat d'un univers |
| `PUT` | `/admin/api/senate/config/:universe_id` | `ADMIN` | Modifier la config |

---

## Annexe C — Arborescence Backend Complète (v1.2)

```
backend/src/
├── main.rs
├── config.rs
├── error.rs
├── db/
│   ├── mod.rs
│   └── pool.rs
├── models/
│   ├── mod.rs, user.rs, empire.rs, planet.rs, building.rs
│   ├── technology.rs, fleet.rs, event.rs, combat.rs
│   ├── market.rs, alliance.rs, debris.rs
│   ├── achievement.rs, mission.rs
│   └── senate.rs
├── handlers/
│   ├── mod.rs, auth.rs, empire.rs, planet.rs, building.rs
│   ├── research.rs, fleet.rs, combat.rs, market.rs
│   ├── diplomacy.rs, galaxy.rs, gamedata.rs
│   ├── achievements.rs, missions.rs
│   └── senate.rs
├── services/
│   ├── mod.rs, auth_service.rs, resource_service.rs
│   ├── build_service.rs, research_service.rs, fleet_service.rs
│   ├── combat_service.rs, espionage_service.rs, market_service.rs
│   ├── alliance_service.rs, ranking_service.rs
│   ├── requirements_service.rs, event_service.rs
│   └── senate_service.rs
├── workers/
│   ├── mod.rs, event_processor.rs, combat_resolver.rs
│   ├── expedition_resolver.rs, resource_tick.rs
│   ├── ranking_updater.rs, npc_spawner.rs, debris_cleaner.rs
│   └── senate_scheduler.rs
├── achievements/
│   ├── mod.rs, engine.rs, conditions.rs
│   ├── rewards.rs
│   └── stats_tracker.rs
├── missions/
│   ├── mod.rs, manager.rs, progress_tracker.rs
│   └── rewards.rs
├── senate/
│   ├── mod.rs, handlers.rs, vote_engine.rs
│   ├── result_resolver.rs, effect_applier.rs
│   └── scheduler.rs
├── admin/
│   ├── mod.rs
│   ├── middleware/
│   │   ├── auth_admin.rs
│   │   └── audit_log.rs
│   ├── handlers/
│   │   ├── mod.rs, users.rs, universe.rs, gamedata.rs
│   │   ├── events.rs, moderation.rs, stats.rs
│   │   ├── senate.rs, achievements.rs
│   │   └── broadcast.rs
│   └── services/
│       ├── audit_service.rs, stats_service.rs
│       └── gamedata_service.rs
├── websocket/
│   ├── mod.rs, hub.rs, events.rs
│   └── broadcaster.rs
├── gamedata/
│   ├── mod.rs, loader.rs, schema.rs
│   └── registry.rs
├── middleware/
│   ├── mod.rs, auth_middleware.rs
│   ├── rate_limit.rs
│   └── request_id.rs
└── routes/
    ├── mod.rs
    └── router.rs
```

---

*— FIN DE L'ADDENDUM v1.2 —*  
*SPACE CONQUEST 4X // ADDENDUM : ADMIN · ACHIEVEMENTS · SÉNAT*
