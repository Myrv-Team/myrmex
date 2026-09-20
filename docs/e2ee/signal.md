# Intégration du Signal Protocol

> **Statut :** brouillon — phase de conception / architecture
> **Responsable :** @Nixo

## Pourquoi Signal Protocol

Règle n°3 du README : **aucune cryptographie maison**. Le Signal Protocol est la référence pour la messagerie E2EE (utilisé par Signal, WhatsApp, Messenger). Il combine :

- **X3DH** (Extended Triple Diffie-Hellman) : établissement d'une clé de session initiale entre deux appareils sans interaction synchrone, via un échange de prekeys.
- **Double Ratchet** : chiffrement asymétrique continu des messages, avec ratchet symétrique (clés de chaîne) et ratchet Diffie-Hellman (clés de session renouvelées à chaque message).

## Primitives retenues

| Usage | Primitive |
|-------|-----------|
| Échange de clés / X3DH | X25519 (courbe Curve25519) |
| Signatures (événements, identité) | Ed25519 |
| Dérivation de clés | SHA-256 / HKDF |
| Chiffrement symétrique des messages | AES-256 (mode AEAD recommandé par Signal) |
| Identifiants de clés | `k_` + ULID (voir [docs/protocol/identifiers.md](../protocol/identifiers.md)) |

## Rôles

- **Client émetteur** : chiffre le message avec la session E2EE du destinataire.
- **Serveur** : relaie le payload E2EE sans jamais le déchiffrer. Il vérifie uniquement la signature Ed25519 de l'enveloppe d'événement (voir [docs/protocol/events.md](../protocol/events.md)).
- **Client destinataire** : déchiffre avec sa session E2EE locale.

## Établissement d'une session (X3DH)

1. Le destinataire publie son **prekey bundle** (clé d'identité, signed prekey, one-time prekeys) sur le serveur — clés publiques uniquement.
2. L'émetteur récupère le bundle et exécute X3DH pour dériver une clé de session initiale.
3. Le premier message est envoyé avec `"prekey": true` (voir [messages.md](messages.md)).
4. Le destinataire répond ; le Double Ratchet prend le relais pour tous les messages suivants.

> Le détail du cycle de vie des sessions est dans [sessions.md](sessions.md), la gestion des prekeys dans [keys.md](keys.md).

## Règles

- Le serveur ne doit **jamais** recevoir de clé privée, de clé de session, ni de texte en clair.
- Toute primitive non listée ci-dessus doit être validée avant intégration.
- Les clés de session sont échangées hors-bande via le protocole d'identité (voir [docs/identity/sessions.md](../identity/sessions.md)).