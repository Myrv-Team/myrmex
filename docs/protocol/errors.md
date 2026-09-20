# Codes d'erreur

> **Statut** : brouillon
> **Responsables** : @Nixo et @notkiwwy

Ce document centralise **tous** les codes d'erreur du protocole Myrmex. Il sert de référence unique pour le client (@Shoppixx), le serveur et le réseau (@notkiwwy). Les codes étaient auparavant dispersés dans [`events.md`](events.md), [`versioning.md`](versioning.md), [`sessions.md`](../identity/sessions.md) et [`messages.md`](../e2ee/messages.md) : ils restent décrits dans leur contexte, mais la liste normative est ici.

## Format d'une erreur

Toute erreur est renvoyée sous forme d'objet JSON :

```json
{
  "error": {
    "code": 400,
    "reason": "invalid_envelope",
    "message": "L'enveloppe est invalide ou mal formée."
  }
}
```

| Champ | Type | Description |
|---|---|---|
| `code` | entier | Code HTTP de l'erreur |
| `reason` | chaîne | Identifiant machine **stable** de l'erreur (snake_case) |
| `message` | chaîne | Description lisible, destinée aux logs et au débogage |

> **Règle client** : ne jamais parser `message`, uniquement `reason`. Le `message` peut évoluer sans préavis ; le `reason` est garanti stable au sein d'une version majeure (voir [`versioning.md`](versioning.md)).

## Tableau récapitulatif

| Code | Signification | Raisons possibles | Référence |
|---|---|---|---|
| 400 | Requête invalide | `invalid_envelope`, `unknown_event_type`, `invalid_identifier`, `unsupported_protocol_version`, `invalid_cursor` | [`events.md`](events.md), [`identifiers.md`](identifiers.md), [`versioning.md`](versioning.md) |
| 401 | Non authentifié | `invalid_signature`, `unauthenticated_author`, `expired_challenge`, `invalid_session` | [`events.md`](events.md), [`sessions.md`](../identity/sessions.md) |
| 403 | Interdit | `forbidden`, `revoked_device`, `unauthorized_session` | [`events.md`](events.md), [`messages.md`](../e2ee/messages.md) |
| 404 | Introuvable | `channel_not_found`, `server_not_found`, `target_not_found` | [`events.md`](events.md) |
| 409 | Conflit | `id_already_used`, `target_already_deleted` | [`events.md`](events.md) |
| 413 | Payload trop volumineux | `payload_too_large` | [`events.md`](events.md), [`messages.md`](../e2ee/messages.md) |

## Détail par code

### 400 — Requête invalide

La requête est mal formée ou viole les règles du protocole. Le serveur **ne doit jamais** traiter une requête 400.

| Raison | Condition de déclenchement |
|---|---|
| `invalid_envelope` | Enveloppe absente, champs manquants, types incorrects, JSON invalide |
| `unknown_event_type` | Le champ `type` de l'événement n'existe pas dans le protocole |
| `invalid_identifier` | Identifiant ne respectant pas le format (préfixe + ULID, voir [`identifiers.md`](identifiers.md)) |
| `unsupported_protocol_version` | Version majeure du protocole non supportée (voir [`versioning.md`](versioning.md)) |
| `invalid_cursor` | Curseur de synchronisation invalide ou inconnu (voir [`sync.md`](sync.md)) |

### 401 — Non authentifié

L'identité de l'auteur n'a pas pu être établie. Le serveur refuse de relayer l'événement.

| Raison | Condition de déclenchement |
|---|---|
| `invalid_signature` | La signature Ed25519 ne correspond pas à l'auteur déclaré |
| `unauthenticated_author` | L'auteur n'a pas de session d'authentification valide |
| `expired_challenge` | Le challenge (nonce) a expiré ou a déjà été utilisé (anti-rejeu) |
| `invalid_session` | Jeton de session inconnu, expiré, révoqué ou non lié à l'appareil |

### 403 — Interdit

L'auteur est authentifié mais n'a pas le droit d'effectuer l'action.

| Raison | Condition de déclenchement |
|---|---|
| `forbidden` | Permissions insuffisantes (rôle, modération, etc.) |
| `revoked_device` | L'appareil `dev_` a été révoqué (voir [`devices.md`](../identity/devices.md)) |
| `unauthorized_session` | La session E2EE n'est pas autorisée pour ce canal |

### 404 — Introuvable

La cible de l'opération n'existe pas.

| Raison | Condition de déclenchement |
|---|---|
| `channel_not_found` | Canal `chn_` inconnu ou inaccessible |
| `server_not_found` | Serveur `srv_` inconnu ou inaccessible |
| `target_not_found` | Cible générique (message, membre, etc.) introuvable |

### 409 — Conflit

L'opération entre en conflit avec l'état actuel. L'événement n'est pas appliqué.

| Raison | Condition de déclenchement |
|---|---|
| `id_already_used` | L'identifiant ULID de l'événement a déjà été utilisé (rejeu détecté) |
| `target_already_deleted` | La cible a déjà été supprimée (tombstone déjà posé) |

### 413 — Payload trop volumineux

L'événement dépasse les limites du protocole.

| Raison | Condition de déclenchement |
|---|---|
| `payload_too_large` | Événement > 64 Ko, ou payload E2EE > 60 Ko |

## Règles transverses

1. **Un seul code par erreur** : le serveur renvoie le code le plus spécifique applicable (ex. 401 avant 403 si la signature est invalide).
2. **`reason` stable** : toute nouvelle raison doit être ajoutée à ce document et au journal de [`versioning.md`](versioning.md).
3. **Pas de fuite d'information** : le `message` ne doit jamais révéler de données personnelles ni de secrets.
4. **Le client ne dépend pas du `message`** : uniquement `code` + `reason`.
5. **Toute modification** (ajout, suppression, changement de sémantique) est documentée ici **et** dans [`versioning.md`](versioning.md), et discutée avec @notkiwwy avant publication.