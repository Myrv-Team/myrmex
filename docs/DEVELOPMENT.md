# Guide de développement — Myrmex

> Document de référence : [`README.txt`](../README.txt) — ce guide en est le complément pratique pour démarrer le développement.

## 1. Vue d'ensemble

Myrmex est une plateforme de messagerie **décentralisée, fédérée et respectueuse de la vie privée** :

- **E2EE par défaut** (Signal Protocol) — les clés privées ne quittent jamais le client.
- **Aucune identité réelle requise** : pas d'e-mail, pas de numéro de téléphone, pas de nom réel.
- **Fédération** : n'importe qui peut déployer son propre serveur ; le serveur principal n'est qu'un point de bootstrap, pas une autorité permanente.
- **Tor** : utilisable via Tor, support des services onion, aucune hypothèse de connexion Internet classique.

## 2. Stack technique

| Couche | Technologie |
|---|---|
| Client desktop | Tauri v2, Rust, React, TypeScript, Vite, HTML, CSS |
| Backend | Rust, Tokio, Axum, WebSocket |
| Base de données | PostgreSQL |
| Cryptographie | Signal Protocol (aucune crypto maison) |

## 3. Prérequis

- **Rust** : toolchain stable (`rustup`) + `cargo`
- **Node.js** : version LTS récente + `npm` (ou `pnpm`/`yarn`)
- **PostgreSQL** : serveur local ou distant pour le développement
- **Tauri v2** : CLI (`cargo install tauri-cli --version "^2"` ou via `npm`)
- **Git** : pour le versionnement

## 4. Structure du dépôt

```
myrmex/
├── apps/
│   ├── desktop/
│   │   ├── src/          # Frontend React / TypeScript (Vite)
│   │   └── src-tauri/    # Coquille Tauri v2 (Rust)
│   └── server/           # Serveur Axum (Rust)
├── crates/
│   ├── myrmex-core/      # Logique métier partagée
│   ├── myrmex-crypto/    # E2EE, Signal Protocol, gestion des clés
│   ├── myrmex-protocol/  # Format des événements, identifiants, versioning
│   ├── myrmex-network/   # Connexions client↔serveur et serveur↔serveur
│   ├── myrmex-federation/ # Protocole de fédération
│   └── myrmex-storage/   # Persistance, migrations PostgreSQL
├── docs/
│   ├── protocol/         # Spécifications du protocole Myrmex
│   ├── DEVELOPMENT.md    # Ce guide
│   └── ...
├── CONTRIBUTING.md       # Règles de contribution
└── README.txt            # Document maître du projet
```

## 5. Mise en place du workspace

### 5.1 Workspace Cargo (racine)

Le dépôt est un monorepo Rust. Le fichier `Cargo.toml` racine doit déclarer le workspace :

```toml
[workspace]
resolver = "2"
members = [
    "apps/server",
    "apps/desktop/src-tauri",
    "crates/myrmex-core",
    "crates/myrmex-crypto",
    "crates/myrmex-protocol",
    "crates/myrmex-network",
    "crates/myrmex-federation",
    "crates/myrmex-storage",
]
```

### 5.2 Base de données

- Créer une base PostgreSQL dédiée au développement (ex. `myrmex_dev`).
- Les migrations vivent dans `crates/myrmex-storage/` (dossier `migrations/`).
- Ne jamais committer de secrets (`.env`, mots de passe) — utiliser un `.env.example`.

## 6. Commandes de build / run

> À adapter une fois les crates et apps initialisées (phase actuelle : conception).

### Serveur (`apps/server`)

```bash
# Lancer le serveur en développement
cargo run -p myrmex-server

# Tests
cargo test -p myrmex-server
```

### Client desktop (`apps/desktop`)

```bash
# Installer les dépendances frontend
cd apps/desktop && npm install

# Lancer en mode développement (Vite + Tauri)
npm run tauri dev

# Build de production
npm run tauri build
```

### Workspace complet

```bash
# Compiler tout le workspace
cargo build --workspace

# Tester tout le workspace
cargo test --workspace

# Vérifier le formatage
cargo fmt --all -- --check

# Linter
cargo clippy --workspace -- -D warnings
```

## 7. Conventions de code

- **Rust** : suivre `rustfmt` et `clippy` sans avertissement (`-D warnings`).
- **TypeScript / React** : TypeScript strict, composants fonctionnels, hooks.
- **Nommage** : clair et descriptif ; commentaires en français ou en anglais, mais cohérents dans tout le dépôt.
- **Commits** : messages en français, format conventionnel (voir [`CONTRIBUTING.md`](../CONTRIBUTING.md)).
- **Pas de crypto maison** : toute opération cryptographique passe par des bibliothèques éprouvées (Signal Protocol, etc.).

## 8. Règles de développement (issues du README)

1. **Ne jamais faire confiance au client** — toute entrée client doit être validée côté serveur.
2. **Ne jamais envoyer de clé privée au serveur** — les clés privées restent côté client.
3. **Aucune crypto maison** — utiliser des implémentations éprouvées.
4. **Ne pas stocker de données personnelles inutiles** — minimiser les données collectées.
5. **Documenter le protocole** — toute modification du protocole doit être documentée dans `docs/protocol/`.
6. **Tester les fonctionnalités critiques** : authentification, permissions, E2EE, synchronisation, fédération, reconnexion, gestion des appareils.

## 9. Ordre de priorité de développement

1. Architecture du protocole
2. Identité cryptographique
3. E2EE
4. Communication client↔serveur
5. Synchronisation
6. Serveurs fédérés
7. Support Tor
8. Interface complète
9. Fonctionnalités communautaires

## 10. Phase actuelle : conception / architecture

Priorités en cours :

1. Définir le protocole Myrmex
2. Définir le modèle d'identité
3. Définir l'architecture E2EE
4. Définir le protocole de fédération
5. Définir la synchronisation
6. Mettre en place le workspace Rust / Tauri 2
7. Construire un prototype minimal client↔serveur

## 11. Répartition des responsabilités

| Domaine | Responsable |
|---|---|
| Backend, authentification/identité, E2EE/crypto, protocole (partagé) | @Nixo |
| Frontend (React/TypeScript/Tauri v2), UI/UX | @Shoppixx |
| Réseau, WebSocket, synchronisation, fédération (partagé) | @notkiwwy |

Voir [`CONTRIBUTING.md`](../CONTRIBUTING.md) pour les règles de coordination entre les trois domaines.