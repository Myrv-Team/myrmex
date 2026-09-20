# Exemples concrets

> **Statut** : brouillon
> **Responsables** : @Nixo et @notkiwwy

Ce document fournit des exemples JSON **complets et travaillés** pour chaque brique du protocole. Il est destiné à @Shoppixx (frontend) pour construire les types TypeScript, et à @notkiwwy (réseau) pour les tests d'intégration.

> Les identifiants ULID ci-dessous sont fictifs mais respectent le format (préfixe + 26 caractères, voir [`identifiers.md`](identifiers.md)).

## 1. Enveloppe commune

Tout événement est transporté dans l'enveloppe commune définie dans [`events.md`](events.md) :

```json
{
  "v": 1,
  "id": "evt_01J2XK3Q8Z9W4Y5R6T7U8V9W0X",
  "type": "message.create",
  "ts": "2026-08-23T21:15:00.000Z",
  "author": "usr_01J2XK2M7P3Q4R5S6T7U8V9W0X",
  "device": "dev_01J2XK2N8Q4R5S6T7U8V9W0X1",
  "server": "srv_01J2XK1L6P2Q3R4S5T6U7V8W0X",
  "channel": "chn_01J2XK4R9S5T6U7V8W9X0Y1Z2",
  "sig": "base64url(ed25519_signature)",
  "payload": {}
}
```

| Champ | Exemple | Règle |
|---|---|---|
| `v` | `1` | Entier = version majeure (voir [`versioning.md`](versioning.md)) |
| `id` | `evt_...` | ULID unique, ordre lexicographique = ordre chronologique |
| `type` | `message.create` | `domaine.action` |
| `ts` | `2026-08-23T21:15:00.000Z` | ISO 8601 UTC, réécrit par le serveur |
| `author` | `usr_...` | Identité de l'auteur |
| `device` | `dev_...` | Appareil émetteur |
| `server` | `srv_...` | Serveur émetteur |
| `channel` | `chn_...` | Optionnel (absent pour les événements serveur) |
| `sig` | `base64url(...)` | Signature Ed25519 de l'enveloppe + payload (hors `sig`) |
| `payload` | `{}` | Contenu spécifique au type |

## 2. Message chiffré (E2EE)

Un `message.create` avec payload E2EE (format défini dans [`messages.md`](../e2ee/messages.md)) :

```json
{
  "v": 1,
  "id": "evt_01J2XK3Q8Z9W4Y5R6T7U8V9W0X",
  "type": "message.create",
  "ts": "2026-08-23T21:15:00.000Z",
  "author": "usr_01J2XK2M7P3Q4R5S6T7U8V9W0X",
  "device": "dev_01J2XK2N8Q4R5S6T7U8V9W0X1",
  "server": "srv_01J2XK1L6P2Q3R4S5T6U7V8W0X",
  "channel": "chn_01J2XK4R9S5T6U7V8W9X0Y1Z2",
  "sig": "base64url(ed25519_signature)",
  "payload": {
    "ciphertext": "base64url(aes256_aead_ciphertext)",
    "nonce": "base64url(12_octets)",
    "session_id": "sess_01J2XK5S0T6U7V8W9X0Y1Z2A3B",
    "sender_key_id": "k_01J2XK5T1U7V8W9X0Y1Z2A3B4C",
    "prekey": false
  }
}
```

- `prekey: true` uniquement pour le **premier message** d'une session (partie X3DH).
- Le serveur ne voit que ce payload chiffré : il ne peut pas lire le contenu.

## 3. Événement non chiffré (métadonnée)

Exemple de `member.join` (non chiffré, lisible par le serveur) :

```json
{
  "v": 1,
  "id": "evt_01J2XK6U2V8W9X0Y1Z2A3B4C5D",
  "type": "member.join",
  "ts": "2026-08-23T21:16:00.000Z",
  "author": "usr_01J2XK2M7P3Q4R5S6T7U8V9W0X",
  "device": "dev_01J2XK2N8Q4R5S6T7U8V9W0X1",
  "server": "srv_01J2XK1L6P2Q3R4S5T6U7V8W0X",
  "sig": "base64url(ed25519_signature)",
  "payload": {
    "member": "usr_01J2XK7V3W9X0Y1Z2A3B4C5D6E"
  }
}
```

## 4. Synchronisation

### 4.1 `sync.request` (client → serveur)

```json
{
  "v": 1,
  "id": "evt_01J2XK8W4X0Y1Z2A3B4C5D6E7F",
  "type": "sync.request",
  "ts": "2026-08-23T21:17:00.000Z",
  "author": "usr_01J2XK2M7P3Q4R5S6T7U8V9W0X",
  "device": "dev_01J2XK2N8Q4R5S6T7U8V9W0X1",
  "server": "srv_01J2XK1L6P2Q3R4S5T6U7V8W0X",
  "sig": "base64url(ed25519_signature)",
  "payload": {
    "cursor": null,
    "limit": 500,
    "protocol_versions": [1]
  }
}
```

- `cursor: null` = synchronisation complète (première connexion).
- `protocol_versions` permet la négociation de version (voir [`versioning.md`](versioning.md)).

### 4.2 `sync.response` (serveur → client)

```json
{
  "v": 1,
  "id": "evt_01J2XK9X5Y1Z2A3B4C5D6E7F8G",
  "type": "sync.response",
  "ts": "2026-08-23T21:17:00.100Z",
  "author": "usr_01J2XK2M7P3Q4R5S6T7U8V9W0X",
  "device": "dev_01J2XK2N8Q4R5S6T7U8V9W0X1",
  "server": "srv_01J2XK1L6P2Q3R4S5T6U7V8W0X",
  "sig": "base64url(ed25519_signature)",
  "payload": {
    "cursor": "evt_01J2XK3Q8Z9W4Y5R6T7U8V9W0X",
    "complete": true,
    "protocol_version": 1,
    "events": [
      {
        "v": 1,
        "id": "evt_01J2XK3Q8Z9W4Y5R6T7U8V9W0X",
        "type": "message.create",
        "ts": "2026-08-23T21:15:00.000Z",
        "author": "usr_01J2XK2M7P3Q4R5S6T7U8V9W0X",
        "device": "dev_01J2XK2N8Q4R5S6T7U8V9W0X1",
        "server": "srv_01J2XK1L6P2Q3R4S5T6U7V8W0X",
        "channel": "chn_01J2XK4R9S5T6U7V8W9X0Y1Z2",
        "sig": "base64url(ed25519_signature)",
        "payload": {}
      }
    ]
  }
}
```

- `complete: false` = il reste des pages (500 événements max par page, voir [`sync.md`](sync.md)).

## 5. Erreur

Format standardisé (voir [`errors.md`](errors.md)) :

```json
{
  "error": {
    "code": 401,
    "reason": "invalid_signature",
    "message": "La signature Ed25519 ne correspond pas à l'auteur déclaré."
  }
}
```

## 6. Poignée de main de fédération (serveur → serveur)

Étape 1 — présentation de l'identité (voir [`federation.md`](federation.md)) :

```json
{
  "v": 1,
  "id": "evt_01J2XKA6Z2A3B4C5D6E7F8G9H0",
  "type": "federation.hello",
  "ts": "2026-08-23T21:18:00.000Z",
  "author": "srv_01J2XK1L6P2Q3R4S5T6U7V8W0X",
  "device": null,
  "server": "srv_01J2XK1L6P2Q3R4S5T6U7V8W0X",
  "sig": "base64url(ed25519_signature)",
  "payload": {
    "public_key": "base64url(ed25519_public_key)",
    "protocol_versions": [1]
  }
}
```

Étape 2 — réponse au challenge (nonce signé) :

```json
{
  "v": 1,
  "id": "evt_01J2XKB7A3B4C5D6E7F8G9H0I1",
  "type": "federation.challenge",
  "ts": "2026-08-23T21:18:00.200Z",
  "author": "srv_01J2XK1L6P2Q3R4S5T6U7V8W0X",
  "device": null,
  "server": "srv_01J2XK1L6P2Q3R4S5T6U7V8W0X",
  "sig": "base64url(ed25519_signature)",
  "payload": {
    "nonce": "base64url(32_octets)",
    "signature": "base64url(ed25519_signature_du_nonce)"
  }
}
```

## 7. Règles de validation rapide

1. `v` doit être supporté (sinon 400 `unsupported_protocol_version`).
2. `id` doit être un ULID valide (sinon 400 `invalid_identifier`).
3. `type` doit exister (sinon 400 `unknown_event_type`).
4. `sig` doit être valide pour l'auteur déclaré (sinon 401 `invalid_signature`).
5. Taille totale ≤ 64 Ko, payload E2EE ≤ 60 Ko (sinon 413 `payload_too_large`).
6. Le serveur réécrit `ts` : le client ne doit jamais se fier à l'ordre d'arrivée.