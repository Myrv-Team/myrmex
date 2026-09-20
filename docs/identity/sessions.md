# Sessions

> **Statut :** brouillon — phase de conception
> **Responsable :** @Nixo

## Deux types de sessions

Myrmex distingue deux notions de session :

| Type | Identifiant | Rôle |
|------|-------------|------|
| Session d'authentification | (jeton opaque, non persistant) | Prouve au serveur que le client possède la clé privée du compte |
| Session E2EE | `sess_...` | Chiffre les messages de bout en bout (Signal Protocol) |

## Session d'authentification

L'authentification ne repose **ni sur un mot de passe, ni sur un e-mail, ni sur
un numéro de téléphone** : elle repose sur la **preuve de possession de la clé
privée**.

### Handshake

1. Le client se connecte au serveur et présente son identifiant `usr_...`.
2. Le serveur répond avec un **challenge** : un nonce aléatoire à usage unique,
   horodaté.
3. Le client signe le challenge avec sa clé privée (Ed25519) et renvoie la
   signature.
4. Le serveur vérifie la signature avec la clé publique enregistrée. Si elle
   est valide, il émet un **jeton de session** (opaque, à durée de vie limitée)
   et l'associe à l'appareil `dev_...` émetteur.

### Règles

- Le challenge est **à usage unique** et expire après un court délai (anti-rejeu).
- Le jeton de session est lié à un **appareil** : un jeton émis pour
  `dev_...` ne peut pas être utilisé par un autre appareil.
- Un jeton émis pour un appareil **révoqué** est invalidé immédiatement
  (voir [`devices.md`](devices.md)).
- Le jeton est renouvelé périodiquement ; le renouvellement exige une nouvelle
  preuve de possession de la clé privée.
- Une signature invalide ou un challenge expiré est rejeté avec une erreur
  `401`.

## Session E2EE

- Les sessions E2EE (`sess_...`) sont établies entre deux appareils via le
  **Signal Protocol** (échange de prekeys, Double Ratchet).
- Le serveur **relaie** les messages chiffrés mais ne peut pas les lire
  (voir [`docs/protocol/events.md`](../protocol/events.md), payload E2EE).
- La **révocation d'un appareil** invalide ses sessions E2EE : les autres
  appareils arrêtent de lui envoyer des messages chiffrés.
- Le détail de l'architecture E2EE sera défini dans le dossier `docs/e2ee/`
  (priorité n°3 du README).

## Règles

- Le serveur ne stocke **jamais** de clé privée ni de jeton en clair
  (règle n°2).
- Les jetons de session sont stockés côté serveur sous forme de **hash**
  (jamais en clair).
- Toute session expirée, révoquée ou invalide est rejetée avec une erreur
  `401`.