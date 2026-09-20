# Versionnement du protocole Myrmex

> Statut : **brouillon** — phase de conception / architecture
> Responsables : @Nixo et @notkiwwy
> Référence : Règle n°5 — *Toute modification du protocole doit être documentée*

## 1. Objectif

Le protocole Myrmex évolue. Le versionnement garantit que :

- un **serveur** et un **client** de versions différentes peuvent dialoguer sans casser ;
- un **serveur** et un autre **serveur** (fédération) de versions différentes restent compatibles ;
- toute évolution est **documentée** et **annoncée** avant déploiement.

## 2. Schéma de version : `MAJEUR.MINEUR.PATCH`

Le protocole suit un versionnage sémantique simplifié :

| Composant | Règle | Exemple |
|---|---|---|
| **MAJEUR** | Changement **incompatible** (casse le format, l'E2EE, la signature, la fédération) | `2.0.0` |
| **MINEUR** | Ajout **rétrocompatible** (nouveau type d'événement, nouveau champ optionnel) | `1.3.0` |
| **PATCH** | Correction **rétrocompatible** (clarification, correction d'erreur de spec) | `1.3.1` |

La version courante du protocole est **1.0.0** (brouillon de conception).

## 3. Champ `v` dans l'enveloppe

Chaque événement transporte sa version de protocole dans le champ `v` de l'enveloppe (voir [`events.md`](events.md)) :

```json
{
  "v": 1,
  "id": "evt_01HZ8K2Q3X...",
  "type": "message.create",
  ...
}
```

Règles :

- `v` est un **entier** correspondant à la version **MAJEURE** du protocole.
- Un événement est toujours émis avec la version majeure supportée par l'émetteur.
- Un récepteur qui ne supporte pas la version majeure de l'événement **rejette** l'événement avec l'erreur `400` (enveloppe invalide) ou négocie une version commune (voir §4).

## 4. Négociation de version

### 4.1 Client ↔ Serveur

À la connexion (WebSocket ou HTTP), le client annonce les versions qu'il supporte :

```json
{
  "type": "sync.request",
  "payload": {
    "protocol_versions": [1, 2],
    "cursor": null
  }
}
```

Le serveur répond avec la version commune la plus élevée :

```json
{
  "type": "sync.response",
  "payload": {
    "protocol_version": 2,
    "cursor": "evt_01HZ8K2Q3X..."
  }
}
```

- Si aucune version commune n'existe, le serveur répond avec l'erreur `400` et un message explicite (`unsupported_protocol_version`).
- Le client doit alors être **mis à jour** avant de pouvoir se connecter.

### 4.2 Serveur ↔ Serveur (fédération)

La négociation se fait lors de l'établissement de la connexion fédérée (voir [`federation.md`](federation.md)). Les deux serveurs échangent leurs versions supportées et retiennent la version commune la plus élevée.

## 5. Règles de compatibilité

| Situation | Version | Comportement |
|---|---|---|
| Même version majeure, MINEUR différent | `1.2` ↔ `1.5` | Compatible : les champs/événements inconnus sont **ignorés** (tolérance) |
| Version majeure différente | `1.x` ↔ `2.x` | **Incompatible** : négociation ou rejet |
| PATCH différent | `1.3.0` ↔ `1.3.1` | Toujours compatible |

Règles impératives :

1. **Ne jamais supprimer** un champ ou un type d'événement existant sans passer en version majeure.
2. **Ne jamais changer** la sémantique d'un champ existant sans passer en version majeure.
3. Un **nouveau champ** doit être **optionnel** (ou avoir une valeur par défaut) pour rester en version mineure.
4. Un récepteur **ignore** les champs inconnus : il ne doit jamais rejeter un événement valide à cause d'un champ qu'il ne connaît pas.
5. Toute modification **doit** être documentée dans ce dossier **et** dans [`docs/DEVELOPMENT.md`](../DEVELOPMENT.md).

## 6. Politique de dépréciation

- Un champ, un type d'événement ou une règle peut être **déprécié** (marqué `deprecated`) dans la documentation.
- La dépréciation **n'entraîne pas** la suppression immédiate : le support est maintenu au minimum jusqu'à la **prochaine version majeure**.
- La suppression effective se fait uniquement lors d'un passage en version majeure, avec annonce préalable.

## 7. Journal des versions

| Version | Date | Changements |
|---|---|---|
| 1.0.0 | 2026-08-21 | Version initiale (brouillon) : format des événements, identifiants ULID, versionnage, synchronisation, fédération |