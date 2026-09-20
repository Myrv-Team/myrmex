# Comptes

> **Statut :** brouillon — phase de conception
> **Responsable :** @Nixo

## Principes

- Un compte est créé **sans e-mail, sans numéro de téléphone, sans nom réel**.
- L'identité est ancrée sur une **paire de clés Ed25519** générée localement
  (voir [`keys.md`](keys.md)).
- L'identifiant `usr_...` est dérivé de l'empreinte de la clé publique
  (hash SHA-256 tronqué, encodé ULID — voir
  [`docs/protocol/identifiers.md`](../protocol/identifiers.md)).
- Le serveur ne connaît que la **clé publique** et l'identifiant dérivé. Il ne
  peut pas créer, usurper ou réinitialiser un compte à la place de l'utilisateur.

## Flux de création de compte

1. **Génération locale** : l'application génère une paire de clés Ed25519
   (clé d'identité) sur l'appareil.
2. **Dérivation de l'identifiant** : `usr_...` est calculé à partir de
   l'empreinte de la clé publique.
3. **Enregistrement** : le client envoie au serveur une demande
   d'enregistrement contenant uniquement :
   - l'identifiant `usr_...` ;
   - la clé publique Ed25519 ;
   - un pseudonyme **optionnel** (affiché aux autres utilisateurs) ;
   - la clé publique de l'appareil initial (voir [`devices.md`](devices.md)).
4. **Validation serveur** : le serveur vérifie que l'identifiant est dérivé
   correctement de la clé publique, que l'identifiant est libre, puis enregistre
   le compte. Il répond avec un **challenge signé** que le client doit signer
   pour prouver la possession de la clé privée (voir [`sessions.md`](sessions.md)).
5. **Premier appareil** : l'appareil initial est enregistré via l'événement
   `device.add` (voir [`devices.md`](devices.md)).

## Données stockées par le serveur

Le serveur stocke, pour chaque compte :

| Donnée | Obligatoire | Notes |
|--------|-------------|-------|
| `usr_...` | oui | Identifiant dérivé de la clé publique |
| Clé publique Ed25519 | oui | Utilisée pour vérifier les signatures |
| Pseudonyme | non | Affiché aux autres utilisateurs ; modifiable |
| Appareils (`dev_...`) | oui | Liste des appareils actifs et révoqués |
| Horodatage de création | oui | Métadonnée technique |

**Aucune donnée personnelle** (e-mail, téléphone, nom réel, adresse IP
persistante, etc.) n'est stockée (règle n°4 du README).

## Règles

- Le serveur **ne peut pas** réinitialiser un compte : sans la clé privée,
  aucune récupération par le serveur n'est possible (voir
  [`recovery.md`](recovery.md)).
- Le serveur **ne peut pas** créer un compte au nom d'un utilisateur : la
  preuve de possession de la clé privée est exigée à l'enregistrement.
- Un identifiant `usr_...` déjà enregistré est rejeté avec une erreur `409`
  (conflit).
- Une clé publique invalide ou un identifiant mal dérivé est rejeté avec une
  erreur `400`.