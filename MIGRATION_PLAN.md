# MIGRATION PLAN — Space Conquest → Conquest
> Généré par @project-manager-senior | Référence : SPACE_CONQUEST_MASTER(1).md + ADDENDUM_v1.2 + INFRA_v1.3

## Légende
## Statut Global

## EPIC 1 — Fondations & Cœur (Semaine 1) [~80% DONE]
### Statut actuel
### Tâches restantes
  - @game-designer : validation planète (slots/temp), formules ressources
  - @engineering-backend-architect : galaxy seeding, planet generation endpoint
  - @engineering-frontend-developer : ResourceTicker (rAF), layout empire onboarding

## EPIC 2 — Infrastructures (Semaine 2) [~60% DONE]
### Statut actuel
### Tâches restantes
  - @game-designer : validation tech tree prérequis + équilibre temps de recherche
  - @engineering-backend-architect : research handler, shipyard handler (ships+defenses), routes + services
  - @engineering-frontend-developer : research UI (start/cancel + timer), shipyard UI, defenses UI

## EPIC 3 — Flottes & Mouvements (Semaines 3-4) [~10% DONE]
### Statut actuel
### Tâches restantes
  - @game-designer : validation formules vol (distance G:S:P, fuel, speed%), catalogue vaisseaux + rapid fire
  - @engineering-backend-architect : fleet_service, fleet handler (send/recall/list), fleet worker (arrival/return), galaxy handler
  - @engineering-frontend-developer : galaxy map, fleet dispatch modal, fleet tracker timeline, FleetCard

## EPIC 4 — PvP & Asynchrone (Semaine 5) [~5% DONE — schema only]
### Statut actuel
### Tâches restantes
  - @game-designer : validation moteur combat (6 rounds, rapid fire, debris%), équilibre espionnage
  - @engineering-backend-architect : combat engine (stateless resolver), combat worker, espionage service, debris handler, rapports
  - @engineering-frontend-developer : battle report UI, espionage report UI, debris field indicator

## EPIC 5 — Systèmes Sociaux (Semaine 6) [~0% DONE — schema only]
### Statut actuel
### Tâches restantes
  - @game-designer : validation marché (fourchettes prix, anti-abus), règles alliances (taille max, diplomatie)
  - @engineering-backend-architect : messaging handler, alliance handler (create/join/leave/invite), market handler (create/accept/cancel offer)
  - @engineering-frontend-developer : messaging UI (inbox/compose), alliance dashboard, market UI (order book)

## EPIC 6 — Classements & Rétention (Semaine 7)
  - @game-designer : scoring formula (points = fleet + buildings + research), achievements catalogue, PvE expeditions design
  - @engineering-backend-architect : ranking service (periodic recalc), achievements trigger engine, expedition worker
  - @engineering-frontend-developer : leaderboard page, achievements panel, expedition log UI

## EPIC 7 — Panel Admin & Opérations (Semaine 8)
  - @game-designer : N/A (vérifie cohérence outils GM avec game balance)
  - @engineering-backend-architect : admin module (séparé), JWT admin + TOTP, audit log immuable, routes admin (users/universes/gamedata/moderation)
  - @engineering-frontend-developer : /admin app Next.js séparée (dashboard, user manager, gamedata editor live, senate manager)

## EPIC 8 — Sénat Galactique & Diplomatie Avancée (Semaine 9)
  - @game-designer : design propositions (buffs/debuffs univers), règles de vote (poids par points), effets actifs
  - @engineering-backend-architect : senate handler (propositions, votes, effets actifs), ACS fleet handler (attaque coordonnée)
  - @engineering-frontend-developer : senate UI (vote, propositions actives), ACS fleet modal

## Règles Transversales PM
  - Ordre strict : Migration SQL → Service Rust → Handler Axum → Next.js Integration
  - @game-designer valide AVANT chaque démarrage d'Epic (checkpoint)
  - Aucune feature UI sans endpoint backend fonctionnel
  - Tests : chaque Epic = migration + handler + 1 happy-path curl/playwright test
