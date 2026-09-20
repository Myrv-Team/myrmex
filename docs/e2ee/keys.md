# Gestion des clés E2EE

> **Statut :** brouillon — phase de conception / architecture
> **Responsable :** @Nixo
> **Référence :** [docs/identity/keys.md](../identity/keys.md)

## Hiérarchie des clés

| Clé | Algorithme | Identifiant | Durée de vie |
|-----|-----------|-------------|--------------|
| Clé d'identité | Ed25519 | `k_` (dérivé de l'empreinte) | Vie du compte (pas de rotation au MVP) |
| Clé d'appareil | Ed25519 | `k_` | Vie de l'appareil |
| Clé de session E2EE | X25519 / Double Ratchet | `sess_` / `k_` | Durée de la session |
| Signed prekey | X25519 | `k_` | Rotation périodique |
| One-time prekeys | X25519 | `k_` | Consommées à chaque nouvelle session |

## Prekey bundles

Chaque appareil publie sur le serveur un **bundle de clés publiques** :

- clé d'identité publique (Ed25519) ;
- signed prekey publique (X25519) + sa signature par la clé d'identité ;
- un lot de one-time prekeys publiques (X25519).

Le serveur stocke ces clés publiques et les sert aux autres appareils lors de l'établissement d'une session. Il ne peut rien en déduire sur le contenu des messages.

## Rotation des clés

- **One-time prekeys** : consommées à chaque nouvelle session X3DH ; le client doit en re-publier dès que le lot descend sous un seuil (ex. 10 restantes).
- **Signed prekey** : rotation périodique (ex. 7 jours) ; la nouvelle signed prekey est signée par la clé d'identité.
- **Clé d'identité** : pas de rotation au MVP — une rotation équivaut à un nouveau compte (voir [docs/identity/keys.md](../identity/keys.md)).
- **Clés de session** : renouvelées en continu par le Double Ratchet (ratchet DH à chaque message).

## Révocation

La révocation d'un appareil (`device.revoke`, voir [docs/identity/devices.md](../identity/devices.md)) :

- invalide ses sessions E2EE (`sess_`) ;
- retire ses prekeys du serveur ;
- empêche tout nouvel établissement de session vers cet appareil.

## Règles

- `400` : clé publique invalide ou bundle mal formé.
- `401` : signature invalide (signed prekey non signée par la clé d'identité).
- Le payload E2EE est limité à 60 Ko (voir [docs/protocol/events.md](../protocol/events.md)) — les bundles et messages doivent rester sous cette limite.