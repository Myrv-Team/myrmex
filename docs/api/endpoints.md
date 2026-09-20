# Endpoints API client ↔ serveur

> **Statut** : brouillon
> **Responsables** : @Nixo et @notkiwwy

## Conventions générales

- **Base URL** : `https://<serveur>/api/v1` (et `wss://<serveur>/api/v1/ws` pour le WebSocket).
- **Format** : JSON UTF-8 compact. Les événements utilisent l'enveloppe commune [`events.md`](../protocol/events.md).
- **Authentification** : en-tête `Authorization: Bearer <token>` (token opaque de session, voir [`sessions.md`](../identity/sessions.md)). Seuls les endpoints d'authentification y échappent.
- **Erreurs** : format `{error: {code, reason, message}}` ([`errors.md`](../protocol/errors.md)).
- **Limites** : événement ≤ 64 Ko, payload E2EE ≤ 60 Ko (erreur `413`).

## Authentification

Le flux complet (challenge à usage unique, signature Ed25519, token lié à `dev_`) est décrit dans [`sessions.md`](../identity/sessions.md).

| Méthode | Endpoint | Auth | Description |
| --- | --- | --- | --- |
| POST | `/auth/challenge` | non | Le client présente son `usr_` et son `dev_` ; le serveur répond avec un nonce à usage unique horodaté. |
| POST | `/auth/session` | non | Le client signe le nonce (Ed25519) ; le serveur vérifie et émet un token de session opaque lié à `dev_`. |

**`POST /auth/challenge`** — corps :

```json
{ "usr": "usr_01J2XK2M7P3Q4R5S6T7U8V9W0X", "dev": "dev_01J2XK2N8Q4R5S6T7U8V9W0X1" }
```

Réponse : `{ "nonce": "<nonce base64>", "expires_at": "<ISO 8601>" }`.

**`POST /auth/session`** — corps :

```json
{ "usr": "usr_01J2XK2M7P3Q4R5S6T7U8V9W0X", "dev": "dev_01J2XK2N8Q4R5S6T7U8V9W0X1", "nonce": "<nonce reçu>", "signature": "<signature Ed25519 base64>" }
```

Réponse : `{ "token": "<token opaque>", "expires_at": "<ISO 8601>" }`. Le token est ensuite envoyé dans l'en-tête `Authorization` de toutes les requêtes.

## Événements

| Méthode | Endpoint | Auth | Description |
| --- | --- | --- | --- |
| POST | `/events` | oui | Publie un événement (message, réaction, présence…). Le serveur vérifie la signature, réécrit `ts`, journalise et relaie. |
| GET | `/events/{id}` | oui | Récupère un événement précis par son `evt_`. |

**`POST /events`** — corps : l'enveloppe commune complète (voir [`examples.md`](../protocol/examples.md)). Réponse : `202 Accepted` avec l'`id` de l'événement journalisé.

## Synchronisation

| Méthode | Endpoint | Auth | Description |
| --- | --- | --- | --- |
| GET | `/sync?cursor=<evt_>&limit=500` | oui | Récupère les événements depuis le curseur (ou tout si `cursor` absent). Réponse : contenu d'un `sync.response`. |
| GET | `/sync/channels/{chn_}` | oui | Synchronisation ciblée sur un canal. |

La sémantique des curseurs et la pagination (500 événements/page, `complete: true`) sont définies dans [`sync.md`](../protocol/sync.md). En WebSocket, la synchronisation se fait par événements `sync.request` / `sync.response`.

## Serveurs et canaux

| Méthode | Endpoint | Auth | Description |
| --- | --- | --- | --- |
| GET | `/servers` | oui | Liste des serveurs accessibles. |
| POST | `/servers` | oui | Crée un serveur (événement `server.create`). |
| GET | `/servers/{srv_}` | oui | Détail d'un serveur. |
| GET | `/servers/{srv_}/channels` | oui | Liste des canaux d'un serveur. |
| POST | `/servers/{srv_}/channels` | oui | Crée un canal (événement `channel.create`). |
| GET | `/channels/{chn_}/events` | oui | Historique d'un canal (paginé par curseur). |

## WebSocket temps réel

- **URL** : `wss://<serveur>/api/v1/ws`
- **Connexion** : token de session dans l'en-tête `Authorization` (ou paramètre `?token=` pour les clients qui ne peuvent pas poser d'en-tête).
- **Sens serveur → client** : push des événements dans l'ordre du journal (ordre ULID) : messages, réactions, présence, `sync.response`.
- **Sens client → serveur** : publication d'événements et `sync.request` (mêmes formats qu'en HTTP).

En cas de coupure, le client se resynchronise via `GET /sync` avec son dernier curseur connu.

## Tableau récapitulatif

| Endpoint | Méthode | Auth | Description |
| --- | --- | --- | --- |
| `/auth/challenge` | POST | non | Obtention du nonce |
| `/auth/session` | POST | non | Obtention du token de session |
| `/events` | POST | oui | Publication d'événement |
| `/events/{id}` | GET | oui | Lecture d'un événement |
| `/sync` | GET | oui | Synchronisation par curseur |
| `/sync/channels/{chn_}` | GET | oui | Synchronisation ciblée |
| `/servers` | GET / POST | oui | Liste / création de serveurs |
| `/servers/{srv_}` | GET | oui | Détail d'un serveur |
| `/servers/{srv_}/channels` | GET / POST | oui | Canaux d'un serveur |
| `/channels/{chn_}/events` | GET | oui | Historique d'un canal |
| `/ws` | WebSocket | oui | Flux temps réel |