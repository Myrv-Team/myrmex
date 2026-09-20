# Clés

> **Statut :** brouillon — phase de conception
> **Responsable :** @Nixo

## Principes

- Toutes les clés sont **générées localement** sur l'appareil.
- La clé privée **ne quitte jamais l'appareil** (règle n°2 du README).
- **Aucune cryptographie maison** (règle n°3) : uniquement des primitives
  éprouvées (Ed25519, X25519, SHA-256, Signal Protocol).
- Le serveur ne reçoit que des **clés publiques** et des **signatures**.

## Hiérarchie des clés

| Clé | Algorithme | Rôle | Identifiant |
|-----|------------|------|-------------|
| Clé d'identité | Ed25519 | Signe les événements de l'utilisateur ; ancre l'identité `usr_...` | `k_...` (dérivé de l'empreinte) |
| Clé d'appareil | Ed25519 | Signe les événements émis par un appareil donné | `k_...` |
| Clé de session E2EE | Signal Protocol (X25519, Double Ratchet) | Chiffre les messages de bout en bout | `sess_...` / `k_...` |
| Clé de pré-signature (prekey) | Signal Protocol | Permet d'initier une session E2EE | `k_...` |

## Génération locale

- La clé d'identité est générée **une seule fois**, à la création du compte
  (voir [`account.md`](account.md)).
- Chaque appareil génère sa **propre paire de clés** à son ajout
  (voir [`devices.md`](devices.md)).
- Les clés de session E2EE sont générées à la demande, lors de l'établissement
  d'une session (Signal Protocol).

## Stockage local sécurisé

- Les clés privées sont stockées dans le **stockage sécurisé de l'OS**
  (keychain / trousseau) ou dans un **coffre chiffré localement** (dérivé d'une
  phrase de récupération, voir [`recovery.md`](recovery.md)).
- Le stockage est chiffré au repos ; l'accès est protégé par le mécanisme de
  déverrouillage de l'appareil (si disponible).
- Les clés privées ne sont **jamais** écrites en clair dans les journaux, les
  sauvegardes cloud ou les exports.

## Gestion des clés publiques

- Les clés publiques sont **publiées** sur le serveur (enregistrement du
  compte, événement `device.add`).
- La **rotation** d'une clé d'identité n'est pas prévue au MVP : l'identité
  `usr_...` est dérivée de la clé publique initiale. Une rotation d'identité
  équivaut à la création d'un nouveau compte.
- La **révocation** d'une clé d'appareil se fait via l'événement
  `device.revoke` (voir [`devices.md`](devices.md)) et invalide les sessions
  E2EE associées.

## Règles

- Toute clé publique reçue est validée (format, courbe, longueur) avant d'être
  acceptée ; une clé invalide est rejetée avec une erreur `400`.
- Une signature invalide est rejetée avec une erreur `401`.
- Les identifiants de clés (`k_...`) suivent le format ULID défini dans
  [`docs/protocol/identifiers.md`](../protocol/identifiers.md).