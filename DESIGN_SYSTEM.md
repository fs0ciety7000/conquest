# Conquest - Design System v1.0 (Next.js / Tailwind v4)

Ce document définit les directives visuelles et techniques strictes pour l'interface utilisateur (UI) du projet "Conquest". L'interface doit reproduire l'esthétique d'un terminal de commande de vaisseau spatial ou d'un HUD militaire de haute technologie.

## 1. Vision & Direction Artistique
- **Esthétique :** Propre, épurée, moderne, scientifique et militaire.
- **Rendu visuel :** Sombre et immersif. Pas d'ombres massives, utilisation intensive de bordures très fines (1px), de grilles strictes et d'un léger effet de superposition ("glassmorphism").
- **Géométrie :** Formes angulaires. Les coins des éléments doivent être biseautés, non arrondis (`rounded-none`) ou très subtilement arrondis (`rounded-sm`).

## 2. Palette de Couleurs (Variables Tailwind v4)
L'utilisation des couleurs doit être rigoureuse pour maintenir l'immersion. 

```css
/* À intégrer dans le fichier CSS principal ou la config Tailwind v4 */
@theme {
  --color-background: #000000;      /* Noir pur pour le fond spatial */
  --color-panel-bg: #050505;        /* Gris abyssal pour les fonds de modules */
  --color-panel-glass: rgba(5, 5, 5, 0.85); /* Fond avec effet verre */

  --color-text-primary: #FFFFFF;    /* Blanc pur pour la haute lisibilité */
  --color-text-secondary: #808080;  /* Gris moyen pour les légendes/labels */
  --color-text-muted: #607D8B;      /* Gris bleuté pour les données secondaires */

  --color-accent-ok: #00FFFF;       /* Cyan néon (Primaire) - OK, Actif, Titres */
  --color-accent-ok-muted: rgba(0, 255, 255, 0.3); /* Cyan assombri pour bordures */
  --color-accent-ok-glow: rgba(0, 255, 255, 0.1); /* Cyan très léger (hover) */

  --color-accent-alert: #FF0000;    /* Rouge alarme (Secondaire) - Hostile, Annuler */
  --color-accent-alert-muted: rgba(255, 0, 0, 0.3); /* Rouge assombri */
  --color-accent-alert-glow: rgba(255, 0, 0, 0.1); /* Rouge très léger (hover) */

  --color-resource-metal: #FFC107;   /* Jaune/Or */
  --color-resource-crystal: #00E5FF; /* Cyan clair */
  --color-resource-deut: #4CAF50;    /* Vert */
  --color-resource-energy: #FFC107;  /* Jaune */
}
