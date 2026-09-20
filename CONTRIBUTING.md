# Guide de contribution — Myrmex

> Document de référence : [`README.txt`](README.txt) — ce guide définit les règles de collaboration sur le dépôt partagé.

## 1. Répartition des tâches

Le projet est développé par trois contributeurs. Chacun reste maître de son domaine :

| Domaine | Responsable | Périmètre |
|---|---|---|
| **Backend** | @Nixo | Architecture serveur, API, Axum, Tokio, gestion des utilisateurs, serveurs/canaux, permissions, modération, PostgreSQL, migrations, stockage, déploiement |
| **Authentification / identité** | @Nixo | Création d'identité, clés publiques, sessions, gestion des appareils, révocation, récupération cryptographique — **sans e-mail ni téléphone** |
| **E2EE / cryptographie** | @Nixo | Architecture E2EE, intégration Signal Protocol, gestion des clés, sessions cryptographiques, chiffrement/déchiffrement, vérification d'identité, rotation des clés, sécurité du stockage local |
| **Protocole** | @Nixo + @notkiwwy | Format des événements, identifiants, synchronisation, versioning, compatibilité, fédération |
| **Frontend** | @Shoppixx | React, TypeScript, Tauri v2, gestion d'état, navigation, composants, affichage conversations/serveurs/canaux, gestion des fichiers, paramètres utilisateur |
| **UI/UX** | @Shoppixx | Design system, interface de messagerie, états de connexion/synchronisation, erreurs, accessibilité, responsive — **ne doit pas être une copie visuelle de Discord** |
| **Réseau** | @notkiwwy | Connexions client↔serveur et serveur↔serveur, reconnexion, routage, erreurs réseau, support Tor |
| **WebSocket** | @notkiwwy | Temps réel, événements, heartbeat, reconnexion, synchronisation |
| **Synchronisation** | @notkiwwy | Messages, conversations, multi-appareils, événements manquants, récupération, ordonnancement, déduplication |
| **Fédération** | @notkiwwy + @Nixo | Protocole serveur↔serveur, authentification des serveurs, échange d'événements, découverte, serveurs bloqués, résilience réseau |

## 2. Coordination entre domaines

Les zones partagées imposent une coordination stricte :

- **Le protocole doit être défini avant de multiplier les implémentations** — pas d'implémentation parallèle sans spécification commune.
- **L'E2EE suppose les mêmes hypothèses cryptographiques** côté frontend, backend et réseau — toute évolution doit être validée par @Nixo.
- **Le format des événements de fédération et la synchronisation doivent être communs** au serveur et au client.
- **Toute modification d'API doit être documentée** pour ne pas casser le frontend.

## 3. Workflow Git

### 3.1 Branches

- `master` (ou `main`) : branche stable, toujours déployable.
- Branches de fonctionnalité : `feat/<domaine>/<description>` (ex. `feat/crypto/rotation-cles`).
- Branches de correction : `fix/<domaine>/<description>`.
- Branches de documentation : `docs/<description>`.

### 3.2 Commits

- Messages en **français**, format conventionnel :

```
<type>(<portée>): <description>

<détails éventuels>
```

- Types : `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `perf`.
- Portée : domaine concerné (`crypto`, `protocol`, `server`, `frontend`, `network`, `storage`, `federation`, `ui`…).
- Exemple : `feat(crypto): ajouter la rotation des clés de session`

### 3.3 Processus de contribution

1. Créer une branche depuis `master` à jour.
2. Implémenter avec des commits atomiques et descriptifs.
3. Vérifier : `cargo fmt`, `cargo clippy`, `cargo test` (et les tests frontend le cas échéant).
4. Ouvrir une **pull request** vers `master`.
5. La PR doit être **revue par au moins un autre contributeur** — en particulier pour les zones partagées (protocole, E2EE, fédération, API).
6. Ne pas merger soi-même une PR touchant un domaine partagé sans validation.

## 4. Règles de développement (non négociables)

1. **Ne jamais faire confiance au client** — validation systématique côté serveur.
2. **Ne jamais envoyer de clé privée au serveur** — les clés privées restent côté client.
3. **Aucune crypto maison** — uniquement des implémentations éprouvées (Signal Protocol, etc.).
4. **Ne pas stocker de données personnelles inutiles** — minimiser les données collectées.
5. **Documenter le protocole** — toute modification du protocole doit être documentée dans `docs/protocol/` dans la même PR.
6. **Tester les fonctionnalités critiques** : authentification, permissions, E2EE, synchronisation, fédération, reconnexion, gestion des appareils.

## 5. Tests requis

Toute PR doit couvrir, selon le domaine touché :

- **Authentification / identité** : création d'identité, sessions, gestion des appareils, révocation.
- **Permissions** : contrôle d'accès serveurs/canaux.
- **E2EE** : chiffrement/déchiffrement, rotation des clés, vérification d'identité.
- **Synchronisation** : ordonnancement, déduplication, événements manquants, multi-appareils.
- **Fédération** : échange d'événements entre serveurs, blocage de serveurs.
- **Reconnexion** : reprise après coupure, resynchronisation post-déconnexion.

## 6. Documentation

- Le `README.txt` est le **document maître** : toute évolution majeure d'architecture ou de répartition doit y être reflétée.
- Les spécifications techniques vivent dans `docs/protocol/`.
- Le guide de démarrage pratique est dans `docs/DEVELOPMENT.md`.
- Toute PR modifiant le comportement ou l'API doit mettre à jour la documentation associée.