# Architecture E2EE — Myrmex

> **Statut :** brouillon — phase de conception / architecture
> **Responsable :** @Nixo
> **Dépend de :** [docs/protocol/events.md](../protocol/events.md), [docs/identity/keys.md](../identity/keys.md), [docs/identity/sessions.md](../identity/sessions.md)

## Objectif

Garantir que **le contenu des messages n'est jamais lisible par le serveur** ni par un observateur réseau. Le chiffrement de bout en bout (E2EE) est **activé par défaut** pour toutes les conversations.

## Principes

1. **Aucune cryptographie maison** — uniquement le Signal Protocol (X3DH + Double Ratchet) et des primitives éprouvées (X25519, Ed25519, SHA-256, AES-256).
2. **La clé privée ne quitte jamais l'appareil** — le serveur ne stocke que des clés publiques et relaie des paquets chiffrés.
3. **Le serveur est un simple relais** — il ne peut ni lire, ni modifier, ni rejouer le contenu (signatures Ed25519 + AEAD).
4. **La révocation d'un appareil invalide ses sessions E2EE** — cohérent avec [docs/identity/devices.md](../identity/devices.md).
5. **Chaque fonctionnalité doit répondre** : « Quelles informations cette fonctionnalité révèle-t-elle au serveur ou à un observateur réseau ? »

## Documents

| Document | Contenu |
|----------|---------|
| [signal.md](signal.md) | Intégration du Signal Protocol (X3DH, Double Ratchet, primitives) |
| [keys.md](keys.md) | Gestion des clés (prekeys, bundles, rotation) |
| [sessions.md](sessions.md) | Sessions cryptographiques (états, cycle de vie) |
| [messages.md](messages.md) | Chiffrement / déchiffrement des messages (payload E2EE) |
| [verification.md](verification.md) | Vérification des identités (safety numbers) |
| [storage.md](storage.md) | Sécurité du stockage local des clés |

## Références croisées

- Payload E2EE : [docs/protocol/events.md](../protocol/events.md)
- Identifiants `sess_` / `k_` : [docs/protocol/identifiers.md](../protocol/identifiers.md)
- Hiérarchie des clés : [docs/identity/keys.md](../identity/keys.md)
- Sessions d'authentification vs sessions E2EE : [docs/identity/sessions.md](../identity/sessions.md)
- Cycle de vie des appareils : [docs/identity/devices.md](../identity/devices.md)