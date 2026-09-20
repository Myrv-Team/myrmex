# Appareils

> **Statut :** brouillon — phase de conception
> **Responsable :** @Nixo

## Principes

- Un utilisateur peut utiliser **plusieurs appareils** (ordinateur, téléphone,
  etc.), chacun identifié par `dev_...`.
- Chaque appareil possède sa **propre paire de clés** (voir [`keys.md`](keys.md)).
- Les appareils sont **gérés par l'utilisateur** : ajout, liste, révocation.
- La **révocation** d'un appareil invalide ses clés de session E2EE.

## Ajout d'un appareil

1. **Génération locale** : le nouvel appareil génère sa paire de clés.
2. **Liaison** : l'utilisateur authentifie le nouvel appareil depuis un appareil
   déjà enregistré (par exemple via un code de liaison à usage unique ou une
   signature croisée hors-bande).
3. **Enregistrement** : l'appareil déjà enregistré émet l'événement
   `device.add` avec :
   - l'identifiant `dev_...` du nouvel appareil ;
   - sa clé publique ;
   - des métadonnées minimales (nom d'appareil optionnel, type).
4. **Validation** : le serveur vérifie la signature de l'événement (auteur =
   propriétaire du compte) et enregistre l'appareil.

## Liste des appareils

- Le client peut demander la liste des appareils du compte (actifs et
  révoqués) via une requête de synchronisation (voir
  [`docs/protocol/sync.md`](../protocol/sync.md)).
- Chaque appareil affiche son nom, sa date d'ajout et son état
  (actif / révoqué).

## Révocation

1. L'utilisateur émet l'événement `device.revoke` depuis un appareil
   encore actif, en ciblant l'appareil à révoquer.
2. Le serveur vérifie la signature et marque l'appareil comme révoqué.
3. **Conséquences** :
   - l'appareil révoqué ne peut plus émettre d'événements signés avec sa clé ;
   - ses **sessions E2EE sont invalidées** (les autres appareils ne lui
     envoient plus de messages chiffrés) ;
   - ses sessions d'authentification sont révoquées (voir
     [`sessions.md`](sessions.md)).

## Métadonnées minimales

| Donnée | Obligatoire | Notes |
|--------|-------------|-------|
| `dev_...` | oui | Identifiant de l'appareil |
| Clé publique | oui | Clé Ed25519 de l'appareil |
| Nom d'appareil | non | Libellé affiché (ex. « Portable », « Bureau ») |
| Type | non | `desktop`, `mobile`, etc. |
| Date d'ajout | oui | Métadonnée technique |
| État | oui | `active` / `revoked` |

**Aucune donnée personnelle** n'est associée à un appareil (règle n°4).

## Règles

- Un appareil révoqué ne peut pas être réactivé : il faut en ajouter un
  nouveau.
- Un événement signé par un appareil révoqué est rejeté avec une erreur `401`.
- L'événement `device.revoke` doit être signé par un appareil **actif** du
  même compte ; sinon erreur `403`.