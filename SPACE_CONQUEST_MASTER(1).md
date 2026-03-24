# SPACE CONQUEST — MASTER DESIGN & ARCHITECTURE DOCUMENT
> Version 1.1 — Mars 2026  
> Statut : Document de référence vivant. Toute modification de mécanique de jeu ou d'architecture doit être reflétée ici.

---

## TABLE DES MATIÈRES

### PARTIE I — GAME DESIGN DOCUMENT
1. [Vision & Positionnement](#1-vision--positionnement)
2. [Gestion Planétaire & Infrastructures](#2-gestion-planétaire--infrastructures)
3. [Arbre des Technologies](#3-arbre-des-technologies)
4. [Dispatcher de Flotte & Machine à États](#4-dispatcher-de-flotte--machine-à-états)
5. [Système de Combat](#5-système-de-combat)
6. [Espionnage & Diplomatie](#6-espionnage--diplomatie)
7. [Économie & Marchés](#7-économie--marchés)
8. [File d'Événements & Workers](#8-file-dévénements--workers)
9. [Équilibrage & Formules](#9-équilibrage--formules)
10. [Contenu Data-Driven — Schémas JSON](#10-contenu-data-driven--schémas-json)

### PARTIE II — ARCHITECTURE TECHNIQUE
11. [Vue d'ensemble & Stack](#11-vue-densemble--stack)
12. [Backend Rust — Structure Modulaire](#12-backend-rust--structure-modulaire)
13. [Base de Données — Schémas SQL](#13-base-de-données--schémas-sql)
14. [Cache & Real-Time (Redis / WebSocket)](#14-cache--real-time-redis--websocket)
15. [API REST & Swagger](#15-api-rest--swagger)
16. [Frontend Next.js](#16-frontend-nextjs)
17. [Design System](#17-design-system)
18. [Infrastructure & Déploiement](#18-infrastructure--déploiement)
19. [Sécurité & Anti-Cheat](#19-sécurité--anti-cheat)
20. [Roadmap & Phases](#20-roadmap--phases)

---

# PARTIE I — GAME DESIGN DOCUMENT

---

## 1. Vision & Positionnement

### 1.1 Concept

Space Conquest est un jeu de stratégie **4X** (eXplore, eXpand, eXploit, eXterminate) massivement multijoueur en navigateur, inspiré d'OGame tout en apportant des mécaniques modernes : arbre de technologies visuel et interactif, missions PvE (Mandates), diplomatie inter-empires, et une économie entièrement **data-driven**.

Le joueur dirige un Empire galactique depuis sa Planète Mère. Il colonise de nouveaux mondes, bâtit des flottes, recherche des technologies, et entre en compétition avec d'autres joueurs dans un univers persistant tournant **24h/24**.

### 1.2 Piliers de Design

| Pilier | Description |
|--------|-------------|
| **Asynchrone-first** | Le jeu tourne en continu. Les actions ont des durées de traitement. Aucune action n'est instantanée. |
| **Data-driven total** | Tout contenu (bâtiments, vaisseaux, technologies) est défini en JSON/YAML. Ajouter un vaisseau = ajouter un fichier de config, zéro recompilation. |
| **Progression longue** | La progression s'étale sur des semaines/mois avec des paliers de puissance clairs. |
| **Équité non-P2W** | La monnaie premium accélère marginalement mais n'achète pas de puissance directe. |
| **Lisibilité permanente** | L'UI affiche en permanence l'état de l'empire, alertes et events en cours. |

### 1.3 Paramètres Globaux

| Attribut | Valeur |
|----------|--------|
| Plateforme | Web (navigateur desktop & mobile) |
| Public cible | Fans de 4X (OGame, Ikariam, Forge of Empires) |
| Âge cible | 16 — 40 ans |
| Modèle économique | Free-to-Play + Premium optionnel (cosmétiques + QoL) |
| Structure univers | Serveurs par univers (~3 000 joueurs/univers) |

---

## 2. Gestion Planétaire & Infrastructures

### 2.1 Planète Mère & Colonies

Chaque joueur commence avec une **Planète Mère** générée dans un secteur aléatoire de la galaxie. La planète possède un nombre de **cases (slots)** déterminé à la génération (entre 120 et 200 selon sa taille). Chaque niveau d'un bâtiment occupe 1 case, ce qui impose des **choix stratégiques permanents**.

La colonisation de nouvelles planètes est possible via un **Vaisseau de Colonisation**. Chaque colonie hérite du système de bâtiments et de file d'attente, mais partage l'arbre de technologies de l'Empire.

### 2.2 Production de Ressources

| ID | Nom | Produit | Formule Production | Formule Énergie Consommée |
|----|-----|---------|-------------------|--------------------------|
| `bldg_metal_mine` | Mine de Métal | Métal | `30 × lvl × 1.1^lvl` | `10 × lvl × 1.1^lvl` |
| `bldg_crystal_mine` | Mine de Cristal | Cristal | `20 × lvl × 1.1^lvl` | `10 × lvl × 1.1^lvl` |
| `bldg_deuterium_synth` | Synthétiseur Deutérium | Deutérium | `10 × lvl × 1.1^lvl × (1.44 - 0.004 × T°)` | `30 × lvl × 1.1^lvl` |

> La température `T°` est une propriété fixe de la planète tirée à la génération (entre -60°C et +120°C). Elle affecte uniquement le rendement du Synthétiseur.

### 2.3 Production d'Énergie

| ID | Nom | Formule Production | Notes |
|----|-----|-------------------|-------|
| `bldg_solar_plant` | Centrale Solaire | `20 × lvl × 1.1^lvl` | Pas de coût en Deutérium |
| `bldg_fusion_reactor` | Centrale à Fusion | `30 × lvl × 1.05^lvl` | Consomme Deutérium en continu |
| `item_solar_satellite` | Satellite Solaire | Production fixe par unité | Destructible en combat |

#### Règle d'Énergie
L'énergie **n'est pas stockée**. Elle est calculée en temps réel :
- `Énergie Produite ≥ Énergie Consommée` → Production mines à **100%**
- `Énergie Produite < Énergie Consommée` → Taux de production = `Prod / Conso` (toutes les mines tournent au ralenti proportionnellement)

Un déficit énergétique est une **urgence stratégique** — il ralentit immédiatement la croissance économique.

### 2.4 Stockage

| ID | Nom | Capacité par Niveau | Comportement si Plein |
|----|-----|--------------------|-----------------------|
| `bldg_metal_storage` | Hangar Métal | `5 000 × 2.5^lvl` | Production stoppée |
| `bldg_crystal_storage` | Hangar Cristal | `5 000 × 2.5^lvl` | Production stoppée |
| `bldg_deuterium_tank` | Réservoir Deutérium | `5 000 × 2.5^lvl` | Production stoppée |

### 2.5 Installations Militaires & Scientifiques

| ID | Nom | Effet |
|----|-----|-------|
| `bldg_robot_factory` | Usine de Robots | Divise le temps de construction par `(1 + niveau)` |
| `bldg_shipyard` | Chantier Spatial | Débloque & accélère la construction de vaisseaux/défenses |
| `bldg_research_lab` | Laboratoire de Recherche | Contribue au Réseau Intergalactique, débloque les recherches |
| `bldg_nanite_factory` | Usine de Nanites | Réduction exponentielle des temps de construction |
| `bldg_terraformer` | Terraformeur | +5 cases disponibles par niveau |
| `bldg_ion_cannon` | Canon Ionique Orbital | Défense planétaire fixe, non déplaçable |
| `bldg_missile_silo` | Silo à Missiles | Stocke et lance missiles interplanétaires |
| `bldg_sensor_phalanx` | Phalange de Détection | Espionner les vols de flotte dans un rayon défini |

---

## 3. Arbre des Technologies

### 3.1 Principes

- L'arbre est **centralisé à l'échelle de l'Empire**. Une recherche s'applique à toutes les planètes.
- Une seule recherche peut être en cours à la fois (sauf déblocage via Réseau Intergalactique).
- Les prérequis sont **vérifiés récursivement** côté backend — jamais côté client.
- Les coûts sont exponentiels : `coût_N = base_cost × multiplicateur^(N-1)`.

### 3.2 Technologies Fondamentales

| ID | Nom | Prérequis | Effet Principal |
|----|-----|-----------|----------------|
| `tech_energy` | Technologie Énergie | Lab Niv.1 | +10% prod énergie/lvl, débloque Fusion |
| `tech_laser` | Technologie Laser | Lab Niv.1, Énergie Niv.2 | Débloque armes laser, +5% dégâts/lvl |
| `tech_ion` | Technologie Ionique | Lab Niv.4, Laser Niv.5 | Débloque moteurs ioniques & canons ions |
| `tech_plasma` | Technologie Plasma | Lab Niv.4, Laser Niv.10, Ion Niv.5 | Arme de fin de partie |
| `tech_espionage` | Espionnage | Lab Niv.3 | +1 niveau de détail rapport/lvl |
| `tech_computer` | Ordinateurs | Lab Niv.1 | +1 slot de flotte simultanée/lvl |
| `tech_astrophysics` | Astrophysique | Lab Niv.4, Énergie Niv.8, Espionnage Niv.4 | Colonisation +1 planète par 2 lvl |
| `tech_intergalactic_net` | Réseau Intergalactique | Lab Niv.10, Ordinateurs Niv.8 | Cumule niveaux labo de toutes les planètes |
| `tech_graviton` | Technologie Graviton | Lab Niv.12, toutes fondamentales Niv.5 | Débloque l'Étoile Noire |

### 3.3 Technologies de Propulsion

| ID | Nom | Prérequis | Effet |
|----|-----|-----------|-------|
| `tech_combustion` | Moteur à Combustion | Lab Niv.1, Énergie Niv.1 | Vaisseaux légers +10% vitesse/lvl |
| `tech_impulse` | Moteur Impulsion | Lab Niv.2, Énergie Niv.2, Combustion Niv.6 | Croiseurs & frégates +20% vitesse/lvl |
| `tech_hyperspace_drive` | Hyperespace | Lab Niv.7, Énergie Niv.5, Boucliers Niv.5, Ion Niv.5 | Capitaux & cuirassés, vitesse maximale |

### 3.4 Technologies de Combat

| ID | Nom | Prérequis | Effet |
|----|-----|-----------|-------|
| `tech_weapons` | Technologie Armes | Lab Niv.4 | +10% dégâts de toute la flotte/lvl |
| `tech_shields` | Technologie Boucliers | Lab Niv.6, Énergie Niv.3 | +10% absorption boucliers/lvl |
| `tech_armor` | Protection Vaisseaux | Lab Niv.2 | +10% points de coque/lvl |
| `tech_targeting` | Système de Ciblage | Lab Niv.5, Ordinateurs Niv.4 | +5% précision, -1 round minimum |

### 3.5 Formule Temps de Recherche

```
temps_secondes = (base_cost_metal + base_cost_crystal) / (1 + labs_total) × 3600
```

Avec **Réseau Intergalactique** activé :
```
labs_total = Σ(niveau_laboratoire de toutes les planètes)
```

---

## 4. Dispatcher de Flotte & Machine à États

### 4.1 Types de Mission

| Mission | Description | Retour auto | Pillage |
|---------|-------------|:-----------:|:-------:|
| `TRANSPORT` | Déplace ressources entre planètes du même empire | ✗ | ✗ |
| `DEPLOY` | Transfère flotte + ressources, reste sur destination | ✗ | ✗ |
| `ATTACK` | Combat PvP ou PvE, retour avec butin | ✓ | ✓ |
| `SPY` | Envoi de sondes, rapport créé, retour automatique | ✓ | ✗ |
| `RECYCLE` | Collecte d'un champ de débris orbital | ✓ | ✗ |
| `COLONIZE` | Envoie vaisseau colonisateur vers position vide | ✗ | ✗ |
| `EXPEDITION` | Explore une zone inconnue, résultat aléatoire | ✓ | Possible |
| `ACS_ATTACK` | Attaque coordonnée multi-joueurs (Alliance) | ✓ | ✓ |
| `ACS_DEFEND` | Renfort défensif coordonné | ✓ | ✗ |

### 4.2 Machine à États d'une Flotte

```
         [Ordre de départ]
               │
           OUTBOUND ──────────────────────────────┐
               │                                  │ (Rappel manuel)
               ▼                                  ▼
          ARRIVED?          IN_COMBAT ────► RETURNING ────► IDLE
         /        \                              ▲
    (DEPLOY)   (ATTACK)                          │
        │          │                        (fin de mission)
      IDLE    IN_COMBAT
```

| État | Description | Transitions Possibles |
|------|-------------|----------------------|
| `IDLE` | En orbite, disponible | → `OUTBOUND` (sur ordre) |
| `OUTBOUND` | En transit vers destination | → `IN_COMBAT`, `ARRIVED` |
| `ARRIVED` | Sur destination (DEPLOY) | → `IDLE` |
| `IN_COMBAT` | Résolution de combat en cours | → `RETURNING` |
| `RETURNING` | En route de retour | → `IDLE` |
| `RECALLED` | Rappel manuel en cours | → `RETURNING` |
| `DESTROYED` | Détruite en combat | État final |

### 4.3 Formules de Calcul du Vol

#### Distance 2D dans la Galaxie (coordonnées G:S:P)

```
distance = |sys_orig - sys_dest| × 1000 + |pos_orig - pos_dest| × 5 + 5
```

Inter-galaxies : `distance = |gal_orig - gal_dest| × 20000 + 5`

#### Durée du Vol

```
speed_factor = 35000 / (speed_percentage / 10 - 5)
flight_time  = 10 + (3500 / speed_factor) × √(10 × distance / vitesse_min_flotte)
```

> La vitesse minimale de la flotte détermine le temps total — un seul vaisseau lent pénalise toute la flotte.

#### Consommation Deutérium

```
deuterium_cost = max(1, round(distance × taille_flotte × (speed_pct / 10)² × 0.0001))
```

### 4.4 Vérifications Avant Départ (Pipeline)

1. Vérifier que toutes les unités sélectionnées existent et appartiennent au joueur
2. Calculer le coût en Deutérium → bloquer si insuffisant
3. Vérifier la destination (occupée pour `ATTACK`, vide pour `COLONIZE`)
4. Vérifier la cohérence de la mission (impossible d'attaquer sa propre planète)
5. Vérifier le nombre de flottes simultanées autorisées (`tech_computer`)
6. Écrire l'événement dans `event_queue` avec `execution_time` calculé
7. Mettre les vaisseaux en état `OUTBOUND` dans la table `units`

---

## 5. Système de Combat

### 5.1 Résolution

Le combat est résolu **entièrement côté serveur** à l'instant T d'arrivée, de façon **synchrone et déterministe** (seed loggable pour replay). Il n'est pas interactif.

- Maximum **6 rounds** de combat
- Chaque round : boucliers rechargés à 100% en début de round (sauf si détruits)
- Si les deux côtés sont debout à R6 → le défenseur gagne, l'attaquant bat en retraite
- Un rapport de combat complet est persisté et accessible aux deux parties

### 5.2 Calcul des Dégâts par Round

```
valeur_attaque_effective  = valeur_attaque × (1 + tech_weapons × 0.10)
valeur_bouclier_effective = valeur_bouclier × (1 + tech_shields × 0.10)
degats_nets               = max(1, attaque_effective - bouclier_effectif)
coque_restante            = coque_actuelle - max(0, degats_nets)
```

Si `coque_restante ≤ 0` → unité détruite. Les dégâts sont distribués sur une cible **aléatoire** parmi les cibles disponibles, modifié par le Rapid Fire.

### 5.3 Rapid Fire (Système Pierre-Papier-Ciseaux)

Chaque vaisseau définit une table `rapid_fire` dans son JSON. Une `chance` de `1.0` = toujours re-tirer gratuitement. Cela crée la dynamique de **composition optimale de flotte** contre des défenses spécifiques.

```json
// Exemple : Croiseur contre défenses légères
"rapid_fire": {
  "ship_light_fighter": { "chance": 0.833 },
  "turret_rocket":      { "chance": 1.0 },
  "turret_laser_light": { "chance": 1.0 }
}
```

### 5.4 Catalogue des Vaisseaux

| ID | Nom | Classe | Coque | Bouclier | Attaque | Cargo | Vitesse Base |
|----|-----|--------|------:|--------:|-------:|------:|------------:|
| `ship_probe` | Sonde Espion | Civil | 100 | 10 | 0 | 5 | 100M |
| `ship_solar_satellite` | Satellite Solaire | Civil | 200 | 1 | 1 | 0 | 0 |
| `ship_colony` | Vaisseau Colonisateur | Civil | 3 000 | 100 | 50 | 7 500 | 2 500 |
| `ship_recycler` | Recycleur | Civil | 1 600 | 10 | 1 | 20 000 | 2 000 |
| `ship_cargo_small` | Cargo Léger | Civil | 400 | 10 | 5 | 5 000 | 5 000 |
| `ship_cargo_large` | Cargo Lourd | Civil | 12 000 | 25 | 5 | 25 000 | 7 500 |
| `ship_light_fighter` | Chasseur Léger | Combat | 400 | 10 | 50 | 50 | 12 500 |
| `ship_heavy_fighter` | Chasseur Lourd | Combat | 1 000 | 25 | 150 | 100 | 10 000 |
| `ship_cruiser` | Croiseur | Combat | 2 700 | 50 | 400 | 800 | 15 000 |
| `ship_battleship` | Cuirassé | Combat | 6 000 | 200 | 1 000 | 1 500 | 10 000 |
| `ship_battlecruiser` | Croiseur de Bataille | Combat | 7 000 | 400 | 700 | 750 | 10 000 |
| `ship_bomber` | Bombardier | Combat | 7 500 | 500 | 1 000 | 500 | 4 000 |
| `ship_destroyer` | Destructeur | Combat | 11 000 | 500 | 2 000 | 2 000 | 5 000 |
| `ship_deathstar` | Étoile Noire | Capital | 900 000 | 50 000 | 200 000 | 1 000 000 | 100 |
| `ship_reaper` | Faucheur | Combat | 14 000 | 700 | 2 800 | 10 000 | 7 000 |
| `ship_pathfinder` | Éclaireur | Scout | 2 300 | 100 | 200 | 10 000 | 22 000 |

### 5.5 Catalogue des Défenses Planétaires

| ID | Nom | Coque | Bouclier | Attaque |
|----|-----|------:|--------:|-------:|
| `turret_rocket` | Lance-Missiles | 200 | 20 | 80 |
| `turret_laser_light` | Laser Léger | 200 | 25 | 100 |
| `turret_laser_heavy` | Laser Lourd | 800 | 100 | 250 |
| `turret_ion` | Canon Ionique | 800 | 500 | 150 |
| `turret_gauss` | Canon Gauss | 3 500 | 200 | 1 100 |
| `turret_plasma` | Tourelle Plasma | 3 000 | 300 | 3 000 |
| `defense_small_shield` | Dôme de Protection Petit | 20 000 | 2 000 | 1 |
| `defense_large_shield` | Dôme de Protection Grand | 70 000 | 10 000 | 1 |

### 5.6 Pillage & Débris

- **Pillage** = `min(cargo_disponible, 50% des ressources stockées)`
- Les ressources dans le **Hangar de Sécurité** (`bldg_secure_hangar`) sont à l'abri du pillage
- **Débris** = `30%` des coûts (métal + cristal) des vaisseaux détruits → champ orbital récoltable
- Les débris persistent **24h** avant désintégration si non récoltés

---

## 6. Espionnage & Diplomatie

### 6.1 Système d'Espionnage

```
delta = tech_spy_attaquant - tech_spy_defenseur
```

| Delta | Informations Révélées |
|-------|-----------------------|
| ≥ 0 | Quantité de ressources |
| ≥ 2 | + Composition de la flotte |
| ≥ 4 | + Défenses planétaires |
| ≥ 6 | + Bâtiments et niveaux |
| ≥ 8 | + Technologies de l'empire |

**Risque de destruction des sondes :**
```
risque = (flotte_defense × 200 + tech_spy_def²) / (sondes_envoyées × 200)
```

**Phalange de Détection** : permet d'espionner les vols de flotte entrants/sortants d'un système entier, sans envoyer de sondes.

### 6.2 Diplomatie — Alliances

- Création avec nom, tag, description et capacité maximale
- Rangs internes : Recrue → Membre → Officier → Co-dirigeant → Dirigeant
- Combat coordonné **ACS** entre membres
- Partage de ressources et messages internes
- Forum d'alliance (messages threadés)

### 6.3 Pactes Diplomatiques

| Type | Description |
|------|-------------|
| Pacte de Non-Agression (NAP) | Engagement mutuel de non-attaque |
| Alliance Commerciale | Taux de marché préférentiel entre les deux empires |
| Alliance Militaire | Permet ACS entre non-membres de la même alliance |
| Déclaration de Guerre | Multiplicateur de points en combat |

---

## 7. Économie & Marchés

### 7.1 Marché NPC (Troc)

Échanges de ressources avec taux variables selon l'offre/demande globale de l'univers :

| Échange | Taux Base NPC | Fluctuation |
|---------|:------------:|:-----------:|
| Métal → Cristal | 3:1 | ±20% |
| Cristal → Deutérium | 2:1 | ±20% |
| Métal → Deutérium | 6:1 | ±20% |

> Les taux fluctuent en fonction du volume total échangé dans l'univers au cours des dernières 24h.

### 7.2 Marché Joueur (P2P)

- Poster une **offre de vente** : quantité, ressource, prix minimum en Crédits
- Poster une **offre d'achat** : quantité souhaitée, budget maximum
- **Matching automatique** quand offre achat ≥ offre vente
- **Commission** : 5% prélevés sur chaque transaction

### 7.3 Monnaie Premium — Matière Noire (Dark Matter)

La Matière Noire est **strictement non-P2W**. Elle offre uniquement des avantages de confort :

| Dépense | Coût | Catégorie |
|---------|-----:|-----------|
| Officier : Commandant (mines +10%) | 25 000 DM / 30j | QoL |
| Officier : Amiral (flotte +10% vitesse) | 25 000 DM / 30j | QoL |
| Officier : Ingénieur (énergie +10%) | 25 000 DM / 30j | QoL |
| Officier : Géologue (stockage +10%) | 25 000 DM / 30j | QoL |
| Officier : Technocrate (labs +10%) | 25 000 DM / 30j | QoL |
| File de construction +1 slot | 2 500 DM permanent | Confort |
| Rappel de flotte instantané | 1 DM / sec restant | QoL |
| Pack cosmétique (skin vaisseau) | 10 000 DM | Cosmétique |
| Renommer une planète | 2 500 DM | Cosmétique |

**Sources sans achat** : expéditions PvE (drops rares), classement de fin de saison (top 100), achievements.

---

## 8. File d'Événements & Workers

### 8.1 Principes

Le jeu est **entièrement event-driven** côté serveur. Aucun tick global : chaque action crée un événement dans une queue avec un `execution_time`. Un worker Rust poll cette queue en continu.

> ⚠️ Le worker tourne en instance **UNIQUE** par univers. Un lock Redis distribué (`SETNX`) garantit l'unicité. En cas de crash, le lock expire automatiquement après 10 secondes.

### 8.2 Types d'Événements

| `event_type` | Payload | Handler |
|-------------|---------|---------|
| `BUILD_COMPLETE` | `{planet_id, building_id, level}` | Incrémente le niveau, libère la file |
| `RESEARCH_COMPLETE` | `{empire_id, tech_id, level}` | Incrémente la techno empire |
| `FLEET_ARRIVAL` | `{fleet_id, mission_type}` | Dispatcher : combat, pillage, colonisation... |
| `FLEET_RETURN` | `{fleet_id}` | IDLE les vaisseaux, ajoute les ressources |
| `RESOURCE_CAP_CHECK` | `{planet_id}` | Stoppe la prod si hangar plein |
| `NPC_ATTACK` | `{target_planet_id, npc_config_id}` | Génère une attaque PvE aléatoire |
| `EXPEDITION_RETURN` | `{fleet_id, roll_seed}` | Résout le résultat de l'expédition |
| `DEBRIS_EXPIRE` | `{debris_id}` | Supprime le champ de débris non récolté |
| `DIPLOMACY_EXPIRE` | `{treaty_id}` | Expire un pacte diplomatique |
| `OFFICER_EXPIRE` | `{empire_id, officer_id}` | Désactive l'officier premium |
| `MARKET_EXPIRE` | `{offer_id}` | Annule une offre de marché expirée |

### 8.3 Architecture du Worker (pseudo-code Rust)

```rust
// workers/event_processor.rs
pub async fn run(pool: PgPool, redis: RedisClient) {
    loop {
        // Lock distribué — une seule instance active par univers
        if !acquire_lock(&redis, "worker_lock", 10).await { 
            sleep(Duration::from_millis(500)).await;
            continue;
        }

        let events = sqlx::query_as!(
            Event,
            "SELECT * FROM event_queue
             WHERE execution_time <= NOW()
             ORDER BY execution_time ASC
             LIMIT 100"
        )
        .fetch_all(&pool)
        .await?;

        for event in events {
            let result = match event.event_type.as_str() {
                "FLEET_ARRIVAL"     => fleet::handle_arrival(&pool, &event).await,
                "BUILD_COMPLETE"    => building::handle_complete(&pool, &event).await,
                "RESEARCH_COMPLETE" => research::handle_complete(&pool, &event).await,
                "NPC_ATTACK"        => combat::handle_npc_attack(&pool, &event).await,
                "EXPEDITION_RETURN" => expedition::handle_return(&pool, &event).await,
                _                   => Err(anyhow!("Unknown event type")),
            };

            if result.is_ok() {
                sqlx::query!("DELETE FROM event_queue WHERE id = $1", event.id)
                    .execute(&pool).await?;
            } else {
                // Incrémenter retry_count, désactiver si > 3
                log::error!("Event {} failed: {:?}", event.id, result);
            }
        }

        release_lock(&redis, "worker_lock").await;
        sleep(Duration::from_millis(500)).await;
    }
}
```

---

## 9. Équilibrage & Formules

### 9.1 Coût des Bâtiments

```
coût_niveau_N = base_cost × multiplicateur^(N-1)
```

Multiplicateurs typiques : `1.5` pour les mines, `2.0` pour les laboratoires avancés.

### 9.2 Coût de Construction des Vaisseaux

Le coût d'un vaisseau est **fixe** (défini dans le JSON). La rapidité de construction dépend des bâtiments :

```
temps_construction = temps_base / (1 + lvl_chantier) × réduction_robots × réduction_nanites
```

### 9.3 Formule de Classement (Points)

```
points = (métal_consommé + cristal_consommé × 2 + deutérium_consommé × 3) / 1000
```

- Tous les bâtiments construits, vaisseaux produits et recherches contribuent aux points
- Les vaisseaux **détruits** font **perdre** des points (puissance nette)
- Les classements sont recalculés toutes les heures via un job dédié

### 9.4 Protection des Nouveaux Joueurs

- Protection active pour les joueurs avec `< 5 000 points`
- La protection est **levée immédiatement** si le joueur attaque un joueur hors-protection
- Ratio de puissance : attaque bloquée si la cible est `< 20%` ou `> 500%` des points de l'attaquant

---

## 10. Contenu Data-Driven — Schémas JSON

> Tous les fichiers de configuration se trouvent dans `game_data/`. Le backend les charge au démarrage et les expose via `/api/gamedata`. Le frontend les consomme au premier chargement et les met en cache.

### 10.1 Schéma Bâtiment

```json
{
  "id": "bldg_metal_mine",
  "name": { "fr": "Mine de Métal", "en": "Metal Mine" },
  "type": "resource_generator",
  "icon": "icon_metal_mine",
  "description": { "fr": "...", "en": "..." },
  "base_cost": { "metal": 60, "crystal": 15, "deuterium": 0 },
  "energy_cost_formula": "10 * level * pow(1.1, level)",
  "cost_multiplier": 1.5,
  "production_formula": "30 * level * pow(1.1, level)",
  "max_level": 40,
  "requirements": []
}
```

### 10.2 Schéma Vaisseau

```json
{
  "id": "ship_cruiser",
  "name": { "fr": "Croiseur", "en": "Cruiser" },
  "class": "MEDIUM_COMBAT",
  "icon": "icon_cruiser",
  "hull": 2700,
  "shield": 50,
  "attack": 400,
  "speed_base": 15000,
  "cargo_capacity": 800,
  "fuel_per_km": 0.3,
  "base_cost": { "metal": 20000, "crystal": 7000, "deuterium": 2000 },
  "build_time_seconds": 3600,
  "rapid_fire": {
    "ship_light_fighter": { "chance": 0.833 },
    "turret_rocket":      { "chance": 1.0 },
    "turret_laser_light": { "chance": 1.0 }
  },
  "requirements": [
    { "type": "building", "id": "bldg_shipyard",      "level": 5 },
    { "type": "tech",     "id": "tech_impulse",        "level": 4 },
    { "type": "tech",     "id": "tech_ion",             "level": 2 }
  ]
}
```

### 10.3 Schéma Technologie

```json
{
  "id": "tech_plasma",
  "name": { "fr": "Technologie Plasma", "en": "Plasma Technology" },
  "category": "FUNDAMENTAL",
  "icon": "icon_plasma",
  "base_cost": { "metal": 2000, "crystal": 4000, "deuterium": 1000 },
  "cost_multiplier": 2.0,
  "effect": {
    "type": "damage_bonus",
    "target": "plasma_weapons",
    "value_per_level": 0.15
  },
  "requirements": [
    { "type": "building", "id": "bldg_research_lab", "level": 4 },
    { "type": "tech",     "id": "tech_laser",         "level": 10 },
    { "type": "tech",     "id": "tech_ion",             "level": 5 }
  ],
  "max_level": 15
}
```

### 10.4 Ajout de Nouveau Contenu

Pour ajouter un nouveau vaisseau/bâtiment/technologie :

1. Créer un fichier JSON dans `game_data/{ships|buildings|technologies}/`
2. Respecter le schéma ci-dessus (validé au démarrage du backend)
3. Ajouter les icônes dans `public/icons/`
4. **Zéro recompilation** — le backend recharge les configs au prochain démarrage

---

# PARTIE II — ARCHITECTURE TECHNIQUE

---

## 11. Vue d'ensemble & Stack

### 11.1 Stack Technologique

| Couche | Technologie | Justification |
|--------|-------------|---------------|
| **Backend API** | Rust + Axum | Performances, sécurité mémoire, async natif, concurrence |
| **Worker Async** | Rust + Tokio | Event queue processing haute performance |
| **ORM / Queries** | sqlx | Requêtes SQL compilées, typage fort, migrations intégrées |
| **Base de données** | PostgreSQL 16 | ACID, JSONB natif, requêtes complexes, Row-Level Security |
| **Cache & Pub/Sub** | Redis 7 (Valkey) | Cache sessions, pub/sub WebSocket, rate limiting |
| **Real-Time** | WebSocket (tokio-tungstenite) | Push events temps réel |
| **Documentation API** | utoipa + Swagger UI | API publique auto-générée depuis les annotations Rust |
| **Frontend** | Next.js 15 (React 19) | SSR, App Router, streaming |
| **UI Components** | Radix UI + CVA + Tailwind | Composants accessibles, variants typés |
| **State Management** | Zustand + TanStack Query | State global léger, cache serveur géré |
| **Animations** | Framer Motion | Animations complexes (flotte, combat, tech tree) |
| **Visualisations** | D3.js | Carte galactique, arbre des technologies |
| **Auth** | JWT + Refresh Tokens | Stateless, sécurisé, refresh automatique |
| **CDN / Assets** | Cloudflare R2 + CDN | Icônes vaisseaux, assets du jeu |
| **Infra** | Docker + Coolify | Auto-deploy, SSL, reverse proxy Traefik |
| **Monitoring** | Prometheus + Grafana | Métriques serveur, latences API, alertes |
| **Logs** | Loki + Grafana | Logs structurés JSON, corrélation avec traces |

### 11.2 Flux de Données Global

```
Client (Browser)
  ──HTTPS/WSS──► Traefik (Reverse Proxy / SSL)
                    ├──► Axum API Server (Rust)
                    │       ├──↔ PostgreSQL  (état persistant)
                    │       ├──↔ Redis        (cache, sessions, pub/sub)
                    │       └──↔ Tokio Worker (event_queue processor)
                    └──► Next.js Frontend (SSR)

Redis Pub/Sub ──► WebSocket Hub ──► Push events ──► Client
```

---

## 12. Backend Rust — Structure Modulaire

> **Principe cardinal** : chaque domaine fonctionnel est dans son propre fichier `.rs`. Jamais de fichier dépassant ~300 lignes. Si un handler grossit, extraire la logique dans un `service/`.

### 12.1 Arborescence Complète

```
backend/
├── Cargo.toml
├── Cargo.lock
├── .env.example
├── migrations/                         # Migrations sqlx (numérotées)
│   ├── 001_create_users.sql
│   ├── 002_create_empires.sql
│   ├── 003_create_planets.sql
│   ├── 004_create_buildings.sql
│   ├── 005_create_technologies.sql
│   ├── 006_create_fleets.sql
│   ├── 007_create_event_queue.sql
│   ├── 008_create_combat_reports.sql
│   ├── 009_create_market.sql
│   └── 010_create_alliances.sql
└── src/
    ├── main.rs                         # Entry point : setup serveur + workers
    ├── config.rs                       # Variables d'environnement (dotenvy)
    ├── error.rs                        # AppError : impl IntoResponse pour Axum
    │
    ├── db/
    │   ├── mod.rs
    │   └── pool.rs                     # PgPool + RedisPool setup
    │
    ├── models/                         # Structs DB (sqlx FromRow + Serialize)
    │   ├── mod.rs
    │   ├── user.rs                     # User, CreateUserDto
    │   ├── empire.rs                   # Empire, EmpireStats
    │   ├── planet.rs                   # Planet, PlanetResources
    │   ├── building.rs                 # Building, BuildingState
    │   ├── technology.rs               # Technology, TechState
    │   ├── fleet.rs                    # Fleet, FleetUnit, FleetMission
    │   ├── event.rs                    # EventQueue entry
    │   ├── combat.rs                   # CombatReport, CombatRound
    │   ├── market.rs                   # MarketOffer
    │   ├── alliance.rs                 # Alliance, AllianceMember
    │   └── debris.rs                   # DebrisField
    │
    ├── handlers/                       # Axum route handlers (thin layer)
    │   ├── mod.rs
    │   ├── auth.rs                     # /api/auth/*
    │   ├── empire.rs                   # /api/empire
    │   ├── planet.rs                   # /api/planets/:id
    │   ├── building.rs                 # /api/planets/:id/buildings
    │   ├── research.rs                 # /api/research
    │   ├── fleet.rs                    # /api/fleets
    │   ├── combat.rs                   # /api/combat/reports
    │   ├── market.rs                   # /api/market
    │   ├── diplomacy.rs                # /api/diplomacy, /api/alliances
    │   ├── galaxy.rs                   # /api/galaxy (carte)
    │   └── gamedata.rs                 # /api/gamedata (configs JSON)
    │
    ├── services/                       # Logique métier (appelés par les handlers)
    │   ├── mod.rs
    │   ├── auth_service.rs             # JWT, bcrypt, sessions
    │   ├── resource_service.rs         # Calcul ressources, énergie, prod/h
    │   ├── build_service.rs            # File de construction, prérequis
    │   ├── research_service.rs         # File de recherche, tech tree
    │   ├── fleet_service.rs            # Dispatch, calcul vol, vérifications
    │   ├── combat_service.rs           # Résolution de combat (déterministe)
    │   ├── espionage_service.rs        # Génération rapports espionnage
    │   ├── market_service.rs           # Matching offres, commissions
    │   ├── alliance_service.rs         # Gestion alliances, rangs
    │   ├── ranking_service.rs          # Calcul & mise à jour classements
    │   ├── requirements_service.rs     # Vérification récursive des prérequis
    │   └── event_service.rs            # Création/scheduling d'événements
    │
    ├── workers/                        # Tâches de fond (Tokio tasks)
    │   ├── mod.rs
    │   ├── event_processor.rs          # Consommateur principal event_queue
    │   ├── combat_resolver.rs          # Handler FLEET_ARRIVAL → combat
    │   ├── expedition_resolver.rs      # Handler EXPEDITION_RETURN
    │   ├── resource_tick.rs            # Synchronisation ressources (30s)
    │   ├── ranking_updater.rs          # Recalcul classements (horaire)
    │   ├── npc_spawner.rs              # Génération attaques PvE aléatoires
    │   └── debris_cleaner.rs           # Nettoyage champs de débris expirés
    │
    ├── websocket/
    │   ├── mod.rs
    │   ├── hub.rs                      # Gestionnaire connexions WS + rooms
    │   ├── events.rs                   # Types d'événements WS (serde)
    │   └── broadcaster.rs              # Pub/Sub Redis → push WS clients
    │
    ├── gamedata/
    │   ├── mod.rs
    │   ├── loader.rs                   # Chargement & validation JSON au boot
    │   ├── schema.rs                   # Structs Rust des configs JSON
    │   └── registry.rs                 # GameDataRegistry (Arc<RwLock<...>>)
    │
    ├── middleware/
    │   ├── mod.rs
    │   ├── auth_middleware.rs          # JWT extraction + validation
    │   ├── rate_limit.rs               # Rate limiting par IP via Redis
    │   └── request_id.rs              # Injection X-Request-ID
    │
    └── routes/
        ├── mod.rs
        └── router.rs                   # Assemblage de toutes les routes Axum
```

### 12.2 Principe Handler → Service

```rust
// handlers/fleet.rs  — thin layer, délègue au service
#[utoipa::path(post, path = "/api/fleets/dispatch", ...)]
pub async fn dispatch_fleet(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Json(payload): Json<DispatchFleetDto>,
) -> Result<Json<FleetResponse>, AppError> {
    let fleet = fleet_service::dispatch(&state.db, &state.redis, user.empire_id, payload).await?;
    Ok(Json(fleet.into()))
}

// services/fleet_service.rs — logique métier complète
pub async fn dispatch(
    pool: &PgPool,
    redis: &RedisClient,
    empire_id: Uuid,
    dto: DispatchFleetDto,
) -> Result<Fleet, AppError> {
    // 1. Vérifications
    requirements_service::check_fleet_requirements(pool, empire_id, &dto).await?;
    // 2. Calcul vol
    let (flight_time, deuterium_cost) = calculate_flight(pool, empire_id, &dto).await?;
    // 3. Création flotte en DB
    let fleet = create_fleet(pool, empire_id, &dto, flight_time).await?;
    // 4. Scheduling événement
    event_service::schedule(pool, EventType::FleetArrival, fleet.id, flight_time).await?;
    Ok(fleet)
}
```

### 12.3 Dépendances Rust (Cargo.toml)

```toml
[dependencies]
axum          = { version = "0.7", features = ["ws", "multipart"] }
tokio         = { version = "1",   features = ["full"] }
sqlx          = { version = "0.8", features = ["postgres", "runtime-tokio-rustls", "json", "chrono", "uuid"] }
redis         = { version = "0.24", features = ["tokio-comp"] }
serde         = { version = "1",   features = ["derive"] }
serde_json    = "1"
uuid          = { version = "1",   features = ["v4", "serde"] }
chrono        = { version = "0.4", features = ["serde"] }
jsonwebtoken  = "9"
argon2        = "0.5"
dotenvy       = "0.15"

# API Documentation
utoipa        = { version = "4", features = ["axum_extras", "chrono", "uuid"] }
utoipa-swagger-ui = { version = "7", features = ["axum"] }

# HTTP middleware
tower         = "0.4"
tower-http    = { version = "0.5", features = ["cors", "trace", "compression-gzip", "request-id"] }

# Observabilité
tracing                 = "0.1"
tracing-subscriber      = { version = "0.3", features = ["env-filter", "json"] }
tracing-loki            = "0.2"

# Utilitaires
rand          = "0.8"
thiserror     = "1"
anyhow        = "1"
moka          = { version = "0.12", features = ["future"] }  # Cache in-memory local
```

---

## 13. Base de Données — Schémas SQL

### 13.1 Utilisateurs

```sql
CREATE TABLE users (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email           TEXT UNIQUE NOT NULL,
    username        TEXT UNIQUE NOT NULL,       -- Identifiant de connexion (immuable)
    display_name    TEXT NOT NULL,              -- Nom affiché en jeu (modifiable)
    password_hash   TEXT NOT NULL,
    dark_matter     BIGINT DEFAULT 0,
    is_banned       BOOLEAN DEFAULT FALSE,
    ban_reason      TEXT,
    last_login_at   TIMESTAMPTZ,
    created_at      TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email    ON users(email);
```

### 13.2 Empires

```sql
CREATE TABLE empires (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id      UUID REFERENCES users(id) ON DELETE CASCADE,
    universe_id  UUID NOT NULL,
    name         TEXT NOT NULL,
    points       BIGINT DEFAULT 0,
    fleet_points BIGINT DEFAULT 0,
    research_points BIGINT DEFAULT 0,
    rank         INT,
    alliance_id  UUID,
    is_protected BOOLEAN DEFAULT TRUE,         -- Protection nouveaux joueurs
    created_at   TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, universe_id)
);
```

### 13.3 Planètes

```sql
CREATE TABLE planets (
    id                   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    empire_id            UUID REFERENCES empires(id),
    name                 TEXT NOT NULL,
    galaxy               SMALLINT NOT NULL CHECK (galaxy BETWEEN 1 AND 9),
    system               SMALLINT NOT NULL CHECK (system BETWEEN 1 AND 499),
    position             SMALLINT NOT NULL CHECK (position BETWEEN 1 AND 15),
    planet_type          TEXT DEFAULT 'COLONY',   -- HOMEWORLD | COLONY | MOON | ABANDONED
    slots_total          SMALLINT NOT NULL,
    temperature          SMALLINT NOT NULL,       -- Affecte rendement deutérium
    metal                NUMERIC(20,2) DEFAULT 500,
    crystal              NUMERIC(20,2) DEFAULT 300,
    deuterium            NUMERIC(20,2) DEFAULT 200,
    last_resource_update TIMESTAMPTZ DEFAULT NOW(),
    created_at           TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(galaxy, system, position)
);

CREATE INDEX idx_planets_empire    ON planets(empire_id);
CREATE INDEX idx_planets_location  ON planets(galaxy, system, position);
```

### 13.4 Bâtiments

```sql
CREATE TABLE buildings (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    planet_id   UUID REFERENCES planets(id) ON DELETE CASCADE,
    building_id TEXT NOT NULL,                  -- Ref vers game_data JSON
    level       SMALLINT DEFAULT 0,
    is_building BOOLEAN DEFAULT FALSE,
    UNIQUE(planet_id, building_id)
);
```

### 13.5 Technologies

```sql
CREATE TABLE technologies (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    empire_id      UUID REFERENCES empires(id) ON DELETE CASCADE,
    tech_id        TEXT NOT NULL,
    level          SMALLINT DEFAULT 0,
    is_researching BOOLEAN DEFAULT FALSE,
    UNIQUE(empire_id, tech_id)
);
```

### 13.6 Flottes & Unités

```sql
CREATE TABLE fleets (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    empire_id        UUID REFERENCES empires(id),
    mission          TEXT NOT NULL,
    status           TEXT DEFAULT 'IDLE',
    origin_planet_id UUID REFERENCES planets(id),
    dest_galaxy      SMALLINT,
    dest_system      SMALLINT,
    dest_position    SMALLINT,
    departure_time   TIMESTAMPTZ,
    arrival_time     TIMESTAMPTZ,
    return_time      TIMESTAMPTZ,
    cargo_metal      NUMERIC(20,2) DEFAULT 0,
    cargo_crystal    NUMERIC(20,2) DEFAULT 0,
    cargo_deuterium  NUMERIC(20,2) DEFAULT 0,
    speed_percent    SMALLINT DEFAULT 100,
    is_acs           BOOLEAN DEFAULT FALSE,
    acs_group_id     UUID
);

CREATE TABLE fleet_units (
    fleet_id  UUID REFERENCES fleets(id) ON DELETE CASCADE,
    ship_id   TEXT NOT NULL,
    quantity  INT  NOT NULL CHECK (quantity > 0),
    PRIMARY KEY (fleet_id, ship_id)
);

CREATE INDEX idx_fleets_empire  ON fleets(empire_id);
CREATE INDEX idx_fleets_status  ON fleets(status);
CREATE INDEX idx_fleets_arrival ON fleets(arrival_time) WHERE status = 'OUTBOUND';
```

### 13.7 File d'Événements

```sql
CREATE TABLE event_queue (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    empire_id      UUID REFERENCES empires(id),
    planet_id      UUID REFERENCES planets(id),
    event_type     TEXT NOT NULL,
    payload        JSONB NOT NULL DEFAULT '{}',
    execution_time TIMESTAMPTZ NOT NULL,
    retry_count    SMALLINT DEFAULT 0,
    created_at     TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_event_queue_execution ON event_queue(execution_time);
CREATE INDEX idx_event_queue_empire    ON event_queue(empire_id);
```

### 13.8 Rapports de Combat

```sql
CREATE TABLE combat_reports (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    attacker_id    UUID REFERENCES empires(id),
    defender_id    UUID REFERENCES empires(id),
    planet_id      UUID REFERENCES planets(id),
    outcome        TEXT NOT NULL,              -- ATTACKER_WIN | DEFENDER_WIN | DRAW
    rounds         JSONB NOT NULL,             -- Détail de chaque round
    loot           JSONB,
    debris         JSONB,
    attacker_read  BOOLEAN DEFAULT FALSE,
    defender_read  BOOLEAN DEFAULT FALSE,
    created_at     TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_combat_attacker ON combat_reports(attacker_id);
CREATE INDEX idx_combat_defender ON combat_reports(defender_id);
```

### 13.9 Champs de Débris

```sql
CREATE TABLE debris_fields (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    galaxy     SMALLINT NOT NULL,
    system     SMALLINT NOT NULL,
    position   SMALLINT NOT NULL,
    metal      NUMERIC(20,2) DEFAULT 0,
    crystal    NUMERIC(20,2) DEFAULT 0,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### 13.10 Marché

```sql
CREATE TABLE market_offers (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    empire_id    UUID REFERENCES empires(id),
    offer_type   TEXT NOT NULL,               -- BUY | SELL
    resource     TEXT NOT NULL,               -- metal | crystal | deuterium
    quantity     NUMERIC(20,2) NOT NULL,
    price        NUMERIC(20,2) NOT NULL,      -- En Crédits par unité
    status       TEXT DEFAULT 'ACTIVE',       -- ACTIVE | FILLED | EXPIRED | CANCELLED
    expires_at   TIMESTAMPTZ NOT NULL,
    created_at   TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_market_resource ON market_offers(resource, offer_type) WHERE status = 'ACTIVE';
```

### 13.11 Alliances

```sql
CREATE TABLE alliances (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT UNIQUE NOT NULL,
    tag         TEXT UNIQUE NOT NULL CHECK (length(tag) <= 5),
    description TEXT,
    leader_id   UUID REFERENCES empires(id),
    max_members INT DEFAULT 50,
    created_at  TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE alliance_members (
    alliance_id UUID REFERENCES alliances(id) ON DELETE CASCADE,
    empire_id   UUID REFERENCES empires(id)  ON DELETE CASCADE,
    rank        TEXT DEFAULT 'MEMBER',        -- RECRUIT | MEMBER | OFFICER | CO_LEADER | LEADER
    joined_at   TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (alliance_id, empire_id)
);
```

---

## 14. Cache & Real-Time (Redis / WebSocket)

### 14.1 Stratégie de Cache Redis

| Clé Redis | Type | TTL | Contenu |
|-----------|------|-----|---------|
| `session:{token}` | String | 15 min | JWT payload, empire_id, user_id |
| `gamedata:buildings` | String (JSON) | ∞ (invalidé au deploy) | Configs bâtiments parsées |
| `gamedata:ships` | String (JSON) | ∞ | Configs vaisseaux parsées |
| `gamedata:techs` | String (JSON) | ∞ | Configs technologies parsées |
| `planet:{id}:resources` | Hash | 30 sec | Metal, cristal, deutérium, timestamp |
| `empire:{id}:points` | String | 5 min | Points de classement cachés |
| `ratelimit:{ip}:{route}` | Counter | 60 sec | Rate limiting par route |
| `ws:room:{empire_id}` | Set | Dynamique | Sessions WebSocket actives |
| `worker_lock:{universe_id}` | String | 10 sec | Lock distribué du worker |

### 14.2 Événements WebSocket Push

| Event | Trigger | Payload |
|-------|---------|---------|
| `FLEET_DEPARTURE` | Lancement d'une flotte | `{fleet_id, arrival_time, mission}` |
| `FLEET_ARRIVAL` | Arrivée à destination | `{fleet_id, outcome}` |
| `FLEET_RETURN` | Retour de flotte | `{fleet_id, cargo}` |
| `COMBAT_REPORT` | Fin de combat | `{report_id, outcome, loot}` |
| `BUILD_COMPLETE` | Bâtiment terminé | `{planet_id, building_id, level}` |
| `RESEARCH_COMPLETE` | Recherche terminée | `{tech_id, level}` |
| `RESOURCE_UPDATE` | Sync ressources (30s) | `{planet_id, metal, crystal, deuterium}` |
| `ATTACK_INCOMING` | Alerte espionnage | `{galaxy, system, position, eta}` |
| `ALLIANCE_MESSAGE` | Message d'alliance | `{from, content, timestamp}` |
| `MARKET_TRADE` | Transaction complétée | `{offer_id, resources, credits}` |

---

## 15. API REST & Swagger

### 15.1 Documentation API avec utoipa

L'API expose une documentation **Swagger UI** interactive à `/swagger-ui` et une spécification **OpenAPI 3.0** à `/api-docs/openapi.json`. Chaque handler Rust est annoté avec `#[utoipa::path(...)]`.

```rust
// routes/router.rs
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(title = "Space Conquest API", version = "1.0.0", description = "API publique du jeu"),
    paths(
        handlers::auth::register,
        handlers::auth::login,
        handlers::fleet::dispatch_fleet,
        handlers::fleet::get_fleets,
        // ...
    ),
    components(schemas(
        models::fleet::Fleet,
        models::fleet::DispatchFleetDto,
        // ...
    )),
    tags(
        (name = "auth",    description = "Authentification & sessions"),
        (name = "empire",  description = "Gestion de l'empire"),
        (name = "planets", description = "Gestion planétaire"),
        (name = "fleet",   description = "Flottes & missions"),
        (name = "research",description = "Arbre des technologies"),
        (name = "market",  description = "Marchés & échanges"),
        (name = "gamedata",description = "Configs publiques du jeu"),
    )
)]
pub struct ApiDoc;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", api_routes(state))
}
```

### 15.2 Routes Authentification

| Méthode | Route | Auth | Description |
|---------|-------|:----:|-------------|
| `POST` | `/api/auth/register` | ✗ | Créer un compte + empire initial |
| `POST` | `/api/auth/login` | ✗ | Login → access_token + refresh_token |
| `POST` | `/api/auth/refresh` | ✗ | Renouveler le token |
| `POST` | `/api/auth/logout` | ✓ | Invalider la session |
| `PUT` | `/api/auth/display-name` | ✓ | Modifier le `display_name` |
| `PUT` | `/api/auth/password` | ✓ | Changer le mot de passe |

### 15.3 Routes Empire & Planètes

| Méthode | Route | Auth | Description |
|---------|-------|:----:|-------------|
| `GET` | `/api/empire` | ✓ | Infos complètes de l'empire |
| `GET` | `/api/empire/planets` | ✓ | Liste des planètes |
| `GET` | `/api/planets/:id` | ✓ | Détail planète (bâtiments, ressources) |
| `GET` | `/api/planets/:id/resources` | ✓ | Ressources calculées temps réel |
| `POST` | `/api/planets/:id/build` | ✓ | Lancer construction/upgrade bâtiment |
| `DELETE` | `/api/planets/:id/build` | ✓ | Annuler construction en cours |
| `GET` | `/api/planets/:id/defense` | ✓ | État des défenses planétaires |
| `POST` | `/api/planets/:id/defense/build` | ✓ | Construire des défenses |

### 15.4 Routes Technologies

| Méthode | Route | Auth | Description |
|---------|-------|:----:|-------------|
| `GET` | `/api/research` | ✓ | État de toutes les technologies de l'empire |
| `POST` | `/api/research/start` | ✓ | Démarrer une recherche |
| `DELETE` | `/api/research/cancel` | ✓ | Annuler la recherche en cours |

### 15.5 Routes Flottes

| Méthode | Route | Auth | Description |
|---------|-------|:----:|-------------|
| `GET` | `/api/fleets` | ✓ | Toutes les flottes actives |
| `GET` | `/api/fleets/:id` | ✓ | Détail d'une flotte |
| `POST` | `/api/fleets/dispatch` | ✓ | Lancer une mission de flotte |
| `POST` | `/api/fleets/:id/recall` | ✓ | Rappeler une flotte en transit |
| `GET` | `/api/combat/reports` | ✓ | Historique des rapports de combat |
| `GET` | `/api/combat/reports/:id` | ✓ | Détail d'un rapport |

### 15.6 Routes Marché & Diplomatie

| Méthode | Route | Auth | Description |
|---------|-------|:----:|-------------|
| `GET` | `/api/market/npc` | ✓ | Taux NPC actuels |
| `POST` | `/api/market/npc/trade` | ✓ | Effectuer un échange NPC |
| `GET` | `/api/market/offers` | ✓ | Offres P2P actives |
| `POST` | `/api/market/offers` | ✓ | Créer une offre |
| `DELETE` | `/api/market/offers/:id` | ✓ | Annuler une offre |
| `GET` | `/api/alliances` | ✓ | Liste des alliances (classement) |
| `GET` | `/api/alliances/:id` | ✗ | Profil public d'une alliance |
| `POST` | `/api/alliances` | ✓ | Créer une alliance |
| `POST` | `/api/alliances/:id/join` | ✓ | Rejoindre une alliance |
| `GET` | `/api/diplomacy/treaties` | ✓ | Pactes actifs |
| `POST` | `/api/diplomacy/treaties` | ✓ | Proposer un pacte |

### 15.7 Routes Carte & Game Data (Publiques)

| Méthode | Route | Auth | Description |
|---------|-------|:----:|-------------|
| `GET` | `/api/galaxy` | ✓ | Secteurs visibles de la galaxie |
| `GET` | `/api/galaxy/:g/:s` | ✓ | Détail d'un système solaire |
| `GET` | `/api/gamedata/buildings` | ✗ | Configs bâtiments (JSON) |
| `GET` | `/api/gamedata/ships` | ✗ | Configs vaisseaux |
| `GET` | `/api/gamedata/technologies` | ✗ | Configs technologies |
| `GET` | `/api/gamedata/defenses` | ✗ | Configs défenses |

> Les routes `/api/gamedata/*` et `/swagger-ui` sont **publiques** (pas d'authentification requise). Idéal pour le développement, les tests et l'intégration d'outils tiers.

---

## 16. Frontend Next.js

### 16.1 Structure du Projet

```
frontend/
├── app/
│   ├── layout.tsx                  # Root layout (fonts, providers)
│   ├── page.tsx                    # Landing / Login
│   ├── (game)/                     # Route group — shell authentifié
│   │   ├── layout.tsx              # Shell : nav + WebSocket provider
│   │   ├── dashboard/
│   │   │   └── page.tsx            # Empire Dashboard
│   │   ├── galaxy/
│   │   │   └── page.tsx            # Carte Galactique D3
│   │   ├── planets/
│   │   │   ├── page.tsx            # Liste planètes
│   │   │   └── [id]/
│   │   │       ├── page.tsx        # Vue planète
│   │   │       ├── buildings/      # Gestion bâtiments
│   │   │       └── defense/        # Gestion défenses
│   │   ├── fleet/
│   │   │   └── page.tsx            # Fleet Command
│   │   ├── research/
│   │   │   └── page.tsx            # Tech Tree interactif
│   │   ├── market/
│   │   │   └── page.tsx            # Marché
│   │   ├── diplomacy/
│   │   │   └── page.tsx            # Diplomatie & Alliances
│   │   └── logs/
│   │       └── page.tsx            # Mandates & Logs
├── components/
│   ├── ui/                         # Design System atoms (Button, Badge, etc.)
│   │   ├── Button.tsx
│   │   ├── Card.tsx
│   │   ├── Badge.tsx
│   │   ├── Countdown.tsx
│   │   ├── ProgressBar.tsx
│   │   └── ResourceChip.tsx
│   └── game/                       # Composants jeu
│       ├── ResourceBar.tsx         # Barre ressources (top nav)
│       ├── FleetCard.tsx
│       ├── TechNode.tsx
│       ├── BuildingCard.tsx
│       ├── CombatReport.tsx
│       ├── GalaxyMap.tsx           # Canvas D3
│       └── TechTree.tsx            # Graphe D3 interactif
├── hooks/
│   ├── useWebSocket.ts             # Connexion WS + gestion events
│   ├── useResources.ts             # Interpolation ressources côté client
│   ├── useGameData.ts              # Cache configs JSON
│   └── useCountdown.ts             # Timer universel (rAF)
├── stores/
│   ├── empireStore.ts              # Zustand : state empire
│   ├── planetStore.ts              # Zustand : state planètes actives
│   └── uiStore.ts                  # Zustand : modals, panels
└── lib/
    ├── api.ts                      # API client typé (fetch wrapper)
    ├── formulas.ts                 # Mirror des formules Rust (prod, coût)
    └── ws.ts                       # WebSocket client singleton
```

### 16.2 Interpolation des Ressources Côté Client

Pour éviter de poller le serveur en continu, le frontend calcule les ressources **localement** en mimant les formules de production. Une sync serveur toutes les 30s via WebSocket corrige la dérive.

```typescript
// hooks/useResources.ts
const productionPerSecond = useMemo(() => {
  const energyRatio = Math.min(1, energyProduced / Math.max(1, energyConsumed));
  return {
    metal:     metalMineProduction    * energyRatio / 3600,
    crystal:   crystalMineProduction  * energyRatio / 3600,
    deuterium: deuteriumProduction    * energyRatio / 3600,
  };
}, [buildings, technologies, energyProduced, energyConsumed]);

// Tick local toutes les secondes via requestAnimationFrame
useInterval(() => {
  setResources(r => ({
    metal:     Math.min(r.metal     + production.metal,     metalCap),
    crystal:   Math.min(r.crystal   + production.crystal,   crystalCap),
    deuterium: Math.min(r.deuterium + production.deuterium, deuteriumCap),
  }));
}, 1000);
```

---

## 17. Design System

> Basé sur les spécifications "Tactical Obsidian" du fichier `design-system.md`.

### 17.1 Identité Visuelle — "Tactical Obsidian"

L'interface vise un look **high-fidelity military bridge**, non pas un "menu de jeu". Les principes directeurs :

- **Asymétrie intentionnelle** : modules de données lourdes équilibrés par des espaces sombres "vides"
- **Zéro border-radius** : angles droits = précision et fabrication haute technologie
- **Règle anti-bordures** : pas de `1px solid` pour sectionner — les limites sont créées par les variations de fond
- **Dark Space = 60% de l'écran** : ne pas surcharger visuellement dans un jeu de données dense

### 17.2 Palette de Couleurs

| Token CSS | Hex | Usage |
|-----------|-----|-------|
| `--surface` | `#0B1323` | Fond principal — le vide infini |
| `--surface-container` | `#18202F` | Panneaux tactiques standards |
| `--surface-container-high` | `#222A3A` | Readouts de données élevés |
| `--surface-container-lowest` | `#060E1D` | Champs "enfoncés" (inputs) |
| `--surface-bright` | `#31394A` | Hover sur items de liste |
| `--primary` | `#C3F5FF` | Texte haute importance, titres |
| `--primary-container` | `#00E5FF` | Actions primaires, bordures actives |
| `--secondary` | `#FFB692` | Alertes Warning (non-rouge) |
| `--on-surface` | `#DBE2F8` | Texte principal (jamais 100% blanc) |
| `--on-surface-variant` | `#BAC9CC` | Texte secondaire, labels |
| `--outline-variant` | `#3B494C` | "Ghost border" à 15% d'opacité |
| `--surface-tint` | `#00DAF3` | Ombre ambiante (5% opacité, 48px blur) |
| `--error` | `#FFAB4F` | Ressources critiques (orange, pas rouge) |

### 17.3 Typographie

Système à **deux polices** :

| Rôle | Police | Taille | Style |
|------|--------|--------|-------|
| Display / Headlines | `Space Grotesk` | 32–56px | Bold, Uppercase, letter-spacing 0.1em |
| Noms de sections | `Space Grotesk` | 18px | SemiBold, Uppercase |
| Body / Descriptions | `Inter` | 14px | Regular |
| Data / Labels | `Space Grotesk` | 11–12px | Bold ou Medium, ALL CAPS, letter-spacing 0.05rem |
| Chiffres / Telemetry | `JetBrains Mono` | 12–14px | Regular (valeurs numériques, logs) |

### 17.4 Composants Clés

#### Boutons (CVA)

```tsx
const buttonVariants = cva(
  "font-['Space_Grotesk'] tracking-widest uppercase text-sm transition-all duration-200 rounded-none",
  {
    variants: {
      variant: {
        primary:   "bg-gradient-to-br from-[#c3f5ff] to-[#00e5ff] text-black hover:brightness-110",
        secondary: "border border-[#00e5ff]/20 text-[#00e5ff] hover:border-[#00e5ff]/60",
        danger:    "bg-[#ffb692] text-black hover:brightness-110",
        ghost:     "text-[#bac9cc] hover:text-[#dbe2f8]",
      },
      size: {
        sm: "px-3 py-1.5 text-xs",
        md: "px-5 py-2.5",
        lg: "px-8 py-3 text-base",
      }
    },
    defaultVariants: { variant: "primary", size: "md" }
  }
);
```

#### Data Cards (Panneaux)

```tsx
// Règle "No-Line" + "Glass & Gradient"
<div className="
  bg-gradient-to-br from-[#18202f] to-[#111927]
  shadow-[0_0_15px_rgba(0,229,255,0.05)]
  border-0
  p-4
">
```

#### Resource Chips (Indicateurs de Ressources)

```tsx
// Barre verticale cyan à gauche, pas de fond
<div className="flex items-center gap-2">
  <div className="w-0.5 h-6 bg-[#00e5ff]" />      {/* Barre accent */}
  <div>
    <span className="font-mono text-xs text-[#bac9cc] uppercase tracking-widest">Metal</span>
    <span className="font-mono text-sm text-[#dbe2f8]">{formatNumber(metal)}</span>
  </div>
</div>
// Si ressource critique → barre passe à --error + pulse animation
```

#### Telemetry Feed

```tsx
// Colonne verticale dense, low-contrast, événements galactiques
<div className="space-y-1 text-xs font-mono">
  {events.map(e => (
    <div key={e.id} className="flex gap-3 text-[#bac9cc]/60">
      <span className="text-[#00e5ff]/40">{e.timestamp}</span>
      <span>{e.message}</span>
    </div>
  ))}
</div>
```

### 17.5 Layout Shell de Base

```html
<div class="bg-[#0b1323] min-h-screen text-[#dbe2f8] font-['Inter']">
  <!-- Top Nav : h-14, bg-[#18202f], border-b border-[#00e5ff]/10 -->
  <nav class="fixed top-0 w-full h-14 bg-[#18202f] z-50
              border-b border-[#3b494c]/30 flex items-center px-6">
  </nav>

  <div class="flex pt-14">
    <!-- Side Nav : w-64, bg-[#18202f] -->
    <aside class="w-64 fixed left-0 top-14 h-[calc(100vh-3.5rem)]
                  bg-[#18202f]">
      <!-- Item actif : indicateur vertical 2px cyan + bg-[#31394a] -->
    </aside>

    <!-- Main Content -->
    <main class="ml-64 flex-1 p-8 overflow-y-auto
                 bg-[url('/grid-texture.svg')] bg-fixed">
    </main>
  </div>
</div>
```

### 17.6 Do's & Don'ts

| ✅ Do | ❌ Don't |
|-------|---------|
| Utiliser `0px` border-radius partout | Arrondir les coins |
| Contraste de taille extrême (display vs label-sm) | Hiérarchie typographique uniforme |
| Laisser `surface` dominer 60% de l'écran | Surcharger chaque pixel |
| `--secondary` (orange) pour les alertes importantes | Rouge arcade pour les warnings |
| Texte `--on-surface` (#dbe2f8) | Texte 100% blanc |
| Layouts alignés à gauche (F-pattern) | Layouts centrés pour les données |
| Ombres teintées cyan à 5% opacité | Ombres noires drop-shadow |
| Définir les limites par changement de fond | Bordures `1px solid` pour sectionner |

---

## 18. Infrastructure & Déploiement

### 18.1 docker-compose.yml

```yaml
version: "3.9"

services:
  postgres:
    image: postgres:16-alpine
    restart: unless-stopped
    volumes: ["pgdata:/var/lib/postgresql/data"]
    environment:
      POSTGRES_DB: spaceconquest
      POSTGRES_USER: ${DB_USER}
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${DB_USER}"]
      interval: 10s

  redis:
    image: valkey/valkey:7-alpine
    restart: unless-stopped
    command: valkey-server --save 60 1 --maxmemory 512mb --maxmemory-policy allkeys-lru
    volumes: ["redisdata:/data"]

  backend:
    build: ./backend
    restart: unless-stopped
    environment:
      DATABASE_URL: postgres://${DB_USER}:${DB_PASSWORD}@postgres/spaceconquest
      REDIS_URL: redis://redis:6379
      JWT_SECRET: ${JWT_SECRET}
      GAME_DATA_PATH: /app/game_data
      RUST_LOG: info
    depends_on:
      postgres: { condition: service_healthy }
      redis:    { condition: service_started }
    volumes: ["./game_data:/app/game_data:ro"]
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.api.rule=Host(`api.space-conquest.gg`)"
      - "traefik.http.routers.api.tls.certresolver=letsencrypt"

  worker:
    build: ./backend
    restart: unless-stopped
    command: ["/app/space-conquest", "--mode=worker"]
    environment:
      DATABASE_URL: postgres://${DB_USER}:${DB_PASSWORD}@postgres/spaceconquest
      REDIS_URL: redis://redis:6379
      GAME_DATA_PATH: /app/game_data
    depends_on: [postgres, redis]
    volumes: ["./game_data:/app/game_data:ro"]

  frontend:
    build: ./frontend
    restart: unless-stopped
    environment:
      NEXT_PUBLIC_API_URL: https://api.space-conquest.gg
      NEXT_PUBLIC_WS_URL:  wss://api.space-conquest.gg/ws
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.web.rule=Host(`space-conquest.gg`)"
      - "traefik.http.routers.web.tls.certresolver=letsencrypt"

  traefik:
    image: traefik:v3
    restart: unless-stopped
    command:
      - "--providers.docker=true"
      - "--entrypoints.web.address=:80"
      - "--entrypoints.websecure.address=:443"
      - "--certificatesresolvers.letsencrypt.acme.httpchallenge=true"
      - "--certificatesresolvers.letsencrypt.acme.email=${ACME_EMAIL}"
    ports: ["80:80", "443:443"]
    volumes: ["traefik_data:/data", "/var/run/docker.sock:/var/run/docker.sock:ro"]

volumes:
  pgdata:
  redisdata:
  traefik_data:
```

### 18.2 Scalabilité

- Le **backend API** est stateless → scalable horizontalement (plusieurs instances derrière Traefik)
- Le **worker** tourne en instance UNIQUE par univers (lock Redis `SETNX`)
- **PostgreSQL** : Row-Level Security activé, connexions poolées via PgBouncer en production
- **Redis Cluster** pour les univers à forte charge (> 1 000 connexions WS simultanées)
- **Coolify** : auto-deploy sur push `main`, rollback en un clic

---

## 19. Sécurité & Anti-Cheat

### 19.1 Authentification

- **Access tokens JWT** (15 min) + **Refresh tokens** (30 jours) stockés en cookie `HttpOnly Secure SameSite=Strict`
- Rate limiting sur `/api/auth/*` : **10 req/min** par IP
- **Argon2id** pour le hashage des mots de passe (paramètres recommandés OWASP 2024)
- Rotation automatique du refresh token à chaque utilisation (rotation policy)

### 19.2 Validation des Inputs

- Toutes les requêtes validées côté backend avec des structs Rust typés (serde + validator)
- **Jamais confiance aux données client** — les ressources sont toujours recalculées côté serveur
- Vérification des prérequis côté backend avant toute action (construction, recherche, flotte)
- Les IDs de ressources sont vérifiés contre le `GameDataRegistry` — un ID inconnu = `400 Bad Request`

### 19.3 Anti-Cheat

- Aucune logique de jeu côté client — tout est calculé côté serveur
- Log de toutes les actions sensibles avec timestamp, IP, et `empire_id`
- Détection d'activité anormale : > 100 req/s par empire → flag automatique pour revue
- **Seed de combat cryptographique** : hash `SHA-256(fleet_id + planet_id + timestamp)` → seed reproductible pour replays vérifiables
- Protection CSRF via `SameSite=Strict` + vérification `Origin` header
- Headers de sécurité via `tower-http` : `X-Content-Type-Options`, `X-Frame-Options`, `Content-Security-Policy`

### 19.4 Rate Limiting par Route

| Route | Limite | Fenêtre |
|-------|--------|---------|
| `POST /api/auth/*` | 10 req | 1 min |
| `POST /api/fleets/dispatch` | 30 req | 1 min |
| `POST /api/market/offers` | 20 req | 1 min |
| `GET /api/galaxy` | 60 req | 1 min |
| `GET /api/gamedata/*` | 120 req | 1 min |
| Toutes autres routes | 100 req | 1 min |

---

## 20. Roadmap & Phases

### Phase 0 — Fondations (4 semaines)

- [ ] Setup Rust/Axum + PostgreSQL + Redis + Docker Compose
- [ ] Auth système complet (register, login, JWT, refresh)
- [ ] Loader `game_data` JSON + validation au démarrage
- [ ] API `/api/gamedata/*` publique
- [ ] Swagger UI (`/swagger-ui`) fonctionnel
- [ ] Migrations sqlx (toutes les tables)
- [ ] Scaffold Next.js 15 + Design System tokens + layout shell

### Phase 1 — MVP Économique (6 semaines)

- [ ] Calcul ressources planétaires (formules, énergie, stockage)
- [ ] File de construction bâtiments (worker `BUILD_COMPLETE`)
- [ ] Système de technologies (worker `RESEARCH_COMPLETE`)
- [ ] WebSocket : `RESOURCE_UPDATE`, `BUILD_COMPLETE`, `RESEARCH_COMPLETE`
- [ ] Frontend : Dashboard, Planètes, Bâtiments, Arbre de Technologies

### Phase 2 — Flottes & Combat (6 semaines)

- [ ] Dispatcher de flotte complet (toutes les missions)
- [ ] Résolveur de combat déterministe (`combat_service.rs`)
- [ ] Rapports de combat + champs de débris
- [ ] Espionnage & sondes
- [ ] Frontend : Fleet Command, Carte Galactique D3
- [ ] Alertes WebSocket : `ATTACK_INCOMING`, `COMBAT_REPORT`

### Phase 3 — Multijoueur & Économie (4 semaines)

- [ ] Marché NPC (taux fluctuants) + marché P2P (matching)
- [ ] Alliances & diplomatie (pactes, ACS)
- [ ] Classement global + recalcul horaire
- [ ] Système de Matière Noire & Officiers premium
- [ ] Protection nouveaux joueurs (bubble + ratio de puissance)

### Phase 4 — Contenu & Polissage (4 semaines)

- [ ] Missions PvE / Mandates (ruines, pirates, expéditions)
- [ ] Lune (colonisation orbitale, Destructeur de Lune)
- [ ] Missiles interplanétaires (`bldg_missile_silo`)
- [ ] Univers multiples & gestion des saisons
- [ ] Mobile responsive polish
- [ ] Monitoring Prometheus/Grafana + alertes critiques
- [ ] Tests de charge (k6) — objectif : 3 000 joueurs simultanés/univers

---

## Annexe A — Variables d'Environnement

```env
# Database
DATABASE_URL=postgres://user:password@localhost:5432/spaceconquest

# Redis
REDIS_URL=redis://localhost:6379

# Auth
JWT_SECRET=<secret_256bits_min>
JWT_EXPIRY_MINUTES=15
REFRESH_TOKEN_EXPIRY_DAYS=30

# Game
GAME_DATA_PATH=./game_data
UNIVERSE_ID=<uuid>

# Server
HOST=0.0.0.0
PORT=8080
RUST_LOG=info,sqlx=warn

# Monitoring
LOKI_URL=http://loki:3100
```

---

## Annexe B — Commandes de Développement

```bash
# Backend
cargo run                          # Démarrer le serveur API
cargo run -- --mode=worker         # Démarrer en mode worker uniquement
cargo sqlx migrate run             # Appliquer les migrations
cargo sqlx prepare                 # Générer le cache sqlx pour CI
cargo test                         # Tests unitaires

# Frontend
pnpm dev                           # Dev server
pnpm build && pnpm start           # Production build

# Docker
docker compose up -d               # Démarrer tous les services
docker compose logs -f backend     # Logs backend en temps réel
docker compose exec postgres psql -U user -d spaceconquest  # Accès DB

# Game Data
# Ajouter un nouveau vaisseau :
# 1. cp game_data/ships/battleship.json game_data/ships/mon_vaisseau.json
# 2. Éditer le JSON
# 3. Redémarrer le backend → validation automatique au boot
```

---

*— FIN DU DOCUMENT —*  
*SPACE CONQUEST 4X // MASTER DOCUMENT v1.1*
