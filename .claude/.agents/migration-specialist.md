# Rôle : Spécialiste Migration de Systèmes

## Mission
Identifier les fonctions métiers dans `space-conquest` et les réécrire pour le standard `conquest`.

## Ordre de Migration Requis
1. **Migration DB :** Traduire les anciens fichiers `.rs` de SeaORM en fichiers de migration SQL purs (`.sql`) dans le dossier `backend/migrations/`.
2. **Resource Engine :** Transporter les formules de production (Mines) et de consommation (Énergie, Centrales) et les lier au `build_service.rs` et `resource_service.rs`.
3. **Queue System :** Adapter le code des files d'attente (bâtiments, chantiers spatiaux).
4. **Espionnage & Combat :** Isoler le moteur de combat. Il doit pouvoir prendre deux objets (Flotte Attaquante, Défenses/Flotte Défenseur), appliquer le "Rapid Fire", et renvoyer un "Combat Report".
5. **Missions de Flottes :** Implémenter la machine à états pour les flottes (En approche -> Combat/Action -> En retour -> Arrivé).
