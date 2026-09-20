# API client ↔ serveur

> **Statut** : brouillon
> **Responsables** : @Nixo et @notkiwwy

Ce dossier décrit l'interface entre le client (frontend Tauri/React de @Shoppixx) et le serveur (Rust/Axum). C'est le point d'entrée n°4 de la feuille de route : **client ↔ serveur**.

## Principes

1. **Le serveur ne fait jamais confiance au client** : chaque requête est authentifiée, chaque événement est vérifié (signature, taille, type) avant d'être journalisé et relayé.
2. **Deux transports complémentaires** :
   - **HTTP (REST)** : opérations ponctuelles (authentification, gestion des serveurs et canaux, publication d'événements, synchronisation initiale).
   - **WebSocket** : flux temps réel (push d'événements, présence, `sync.response`).
3. **Un seul format d'événement** : l'enveloppe commune définie dans [`events.md`](../protocol/events.md) est utilisée partout, en HTTP comme en WebSocket.
4. **Versionnage explicite** : l'API est versionnée (`/api/v1`), la négociation de version du protocole se fait via `sync.request` / `sync.response` (voir [`versioning.md`](../protocol/versioning.md)).
5. **Erreurs normalisées** : toutes les erreurs suivent le format `{error: {code, reason, message}}` défini dans [`errors.md`](../protocol/errors.md).

## Documents

| Document | Contenu |
| --- | --- |
| [`endpoints.md`](endpoints.md) | Référence détaillée des endpoints HTTP et du canal WebSocket |
| [`errors.md`](../protocol/errors.md) | Codes d'erreur normalisés |
| [`examples.md`](../protocol/examples.md) | Exemples JSON concrets |
| [`sessions.md`](../identity/sessions.md) | Flux d'authentification (challenge → session) |
| [`sync.md`](../protocol/sync.md) | Synchronisation par curseurs |
| [`events.md`](../protocol/events.md) | Format des événements |

## Règles de modification

- Toute modification d'endpoint, de champ ou de comportement est documentée ici **et** dans [`DEVELOPMENT.md`](../DEVELOPMENT.md).
- Les changements cassants sont annoncés et discutés avec @notkiwwy (réseau) et @Shoppixx (frontend) avant implémentation.
- L'API suit le versionnage sémantique décrit dans [`versioning.md`](../protocol/versioning.md).