# Format des événements

> **Statut :** brouillon — phase de conception
> **Responsables :** @Nixo et @notkiwwy

L'**événement** est l'unité atomique d'échange entre le client et le serveur, et
entre serveurs. Tout ce qui se passe dans Myrmex (message, réaction, modification,
adhésion, etc.) est représenté par un événement.

## Règles générales

- Encodage : **JSON UTF-8**, compact, sans commentaires.
- Chaque événement est **signé** par son auteur (intégrité) et, si nécessaire,
  **chiffré** de bout en bout (confidentialité).
- Le serveur **relaie** les événements mais ne peut pas lire le contenu chiffré.
- Un événement est **immutable** : on ne le modifie jamais, on émet un nouvel
  événement (édition, suppression) qui le référence.

## Enveloppe commune

Tous les événements partagent la même enveloppe :

```json
{
  "v": 1,
  "id": "evt_01HZ8K2Q3X...",
  "type": "message.create",
  "ts": "2026-08-21T10:00:00.000Z",
  "author": "usr_01HZ8K2Q3X...",
  "device": "dev_01HZ8K2Q3X...",
  "server": "srv_01HZ8K2Q3X...",
  "channel": "chn_01HZ8K2Q3X...",
  "sig": "base64url(Ed25519 signature)",
  "payload": { }
}
```

| Champ | Type | Description |
|-------|------|-------------|
| `v` | entier | Version du protocole (voir [`versioning.md`](versioning.md)) |
| `id` | chaîne | Identifiant unique de l'événement (ULID, voir [`identifiers.md`](identifiers.md)) |
| `type` | chaîne | Type d'événement, format `domaine.action` |
| `ts` | ISO 8601 | Horodatage UTC de création |
| `author` | chaîne | Identifiant de l'utilisateur auteur |
| `device` | chaîne | Identifiant de l'appareil émetteur |
| `server` | chaîne | Identifiant du serveur d'origine |
| `channel` | chaîne | Identifiant du canal/conversation (optionnel selon le type) |
| `sig` | chaîne | Signature Ed25519 de l'enveloppe + payload (hors `sig`) |
| `payload` | objet | Contenu spécifique au type |

## Types d'événements

### Messagerie

| Type | Payload |
|------|---------|
| `message.create` | Contenu E2EE (ciphertext, nonce, clé de session, etc.) |
| `message.edit` | `target` (id du message), nouveau contenu E2EE |
| `message.delete` | `target` (id du message), `reason` (optionnel) |
| `message.reaction.add` | `target`, `emoji` |
| `message.reaction.remove` | `target`, `emoji` |
| `message.reply` | `target`, contenu E2EE |

### Canaux et serveurs

| Type | Payload |
|------|---------|
| `channel.create` | `name`, `kind` (text/voice), `parent` (optionnel) |
| `channel.edit` | `target`, champs modifiés |
| `channel.delete` | `target` |
| `server.create` | `name`, `description` |
| `server.edit` | `target`, champs modifiés |
| `server.delete` | `target` |

### Membres et permissions

| Type | Payload |
|------|---------|
| `member.join` | `user`, `roles` |
| `member.leave` | `user` |
| `member.kick` | `user`, `reason` (optionnel) |
| `member.role` | `user`, `role` ajouté/retiré |

### Appareils et présence

| Type | Payload |
|------|---------|
| `device.add` | `device`, clé publique, métadonnées |
| `device.revoke` | `device` |
| `presence.update` | `status` (online/offline/away), `ts` |

### Synchronisation (interne)

| Type | Payload |
|------|---------|
| `sync.request` | `cursor`, `limit` |
| `sync.response` | `cursor`, liste d'événements |

## Payload E2EE (message.create)

Le contenu d'un message n'est **jamais** visible par le serveur :

```json
{
  "ciphertext": "base64url(...)",
  "nonce": "base64url(...)",
  "session_id": "sess_01HZ8K2Q3X...",
  "sender_key_id": "k_01HZ8K2Q3X...",
  "prekey": false
}
```

- Chiffrement : **Signal Protocol** (Double Ratchet) — aucune cryptographie maison.
- Les clés de session sont échangées hors-bande via le protocole d'identité
  (voir [`identifiers.md`](identifiers.md)).

## Signature

- Algorithme : **Ed25519**.
- La signature couvre l'enveloppe complète (hors champ `sig`) sérialisée en JSON
  canonique (clés triées, pas d'espaces superflus).
- Le serveur vérifie la signature avant de relayer ; il rejette les événements
  non signés ou mal signés.

## Limites

- Taille maximale d'un événement : **64 Ko** (payload compris).
- Taille maximale d'un payload E2EE : **60 Ko** (le reste est réservé à l'enveloppe).
- Les événements au-delà de ces limites sont rejetés avec une erreur `413`.

## Erreurs

| Code | Signification |
|------|---------------|
| `400` | Enveloppe invalide ou type inconnu |
| `401` | Signature invalide ou auteur non authentifié |
| `403` | Auteur non autorisé (permissions) |
| `404` | Canal/serveur cible introuvable |
| `413` | Événement trop volumineux |
| `409` | Conflit (id déjà utilisé, cible déjà supprimée) |