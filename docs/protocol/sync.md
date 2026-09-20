# Synchronisation Myrmex

> Statut : **brouillon** — phase de conception / architecture
> Responsables : @Nixo et @notkiwwy
> Référence : le format des événements et leur synchronisation doivent être **communs au serveur et au client**

## 1. Objectif

La synchronisation garantit qu'un client retrouve **l'état exact** de ses conversations, serveurs et canaux, même après :

- une **déconnexion** (réseau instable, Tor, veille) ;
- un **changement d'appareil** (nouveau device) ;
- une **réinstallation** du client.

## 2. Principe : journal d'événements append-only

Chaque canal (`chn_...`) et chaque serveur (`srv_...`) possède un **journal d'événements** :

- les événements sont **ajoutés** au journal dans l'ordre de leur horodatage serveur ;
- un événement n'est **jamais modifié ni supprimé** : les modifications et suppressions émettent de **nouveaux événements** qui référencent la cible (voir [`events.md`](events.md)) ;
- le journal est **tronçonnable** : les anciens événements peuvent être compactés en **tombstones** (voir §6).

## 3. Curseurs (cursors)

La synchronisation incrémentale repose sur des **curseurs** :

- un curseur est l'identifiant **ULID** (`evt_...`) du dernier événement reçu ;
- les ULID étant **triables chronologiquement** (ordre lexicographique = ordre temporel, voir [`identifiers.md`](identifiers.md)), le serveur peut renvoyer tous les événements **strictement postérieurs** au curseur ;
- un curseur `null` signifie « synchronisation complète depuis le début ».

## 4. Flux de synchronisation

### 4.1 Connexion initiale (nouvel appareil)

1. Le client envoie `sync.request` avec `cursor: null` et la liste des canaux/serveurs concernés.
2. Le serveur répond par `sync.response` contenant :
   - la **version de protocole** négociée (voir [`versioning.md`](versioning.md)) ;
   - le **nouveau curseur** ;
   - les événements manquants (paginés, voir §5).
3. Le client **rejoue** les événements dans l'ordre pour reconstruire l'état local.

### 4.2 Reconnexion après déconnexion

1. Le client se reconnecte (WebSocket) et envoie `sync.request` avec le **dernier curseur connu**.
2. Le serveur renvoie uniquement les événements postérieurs au curseur.
3. Le client applique les événements et met à jour son curseur.

### 4.3 Temps réel

- Pendant une connexion WebSocket active, les événements sont **poussés** en temps réel par le serveur.
- Le client met à jour son curseur à chaque événement reçu.
- En cas de coupure, la reprise se fait par le mécanisme §4.2.

## 5. Pagination

- Une réponse `sync.response` est limitée à **500 événements** par page.
- Si le volume dépasse la limite, le serveur renvoie le curseur de fin de page ; le client enchaîne avec un nouveau `sync.request` à partir de ce curseur.
- Le client **ne doit pas** supposer que la synchronisation est terminée tant que le serveur n'a pas signalé la fin (champ `complete: true`).

## 6. Conflits et tombstones

### 6.1 Modifications et suppressions

- `message.edit` : le nouveau contenu est un **nouvel événement** qui référence le message d'origine (`target: msg_...`). L'état local applique la dernière version.
- `message.delete` : émet un événement de suppression ; l'état local remplace le message par un **tombstone** (contenu effacé, métadonnées conservées).
- Les tombstones sont conservés pour la **cohérence entre appareils** : un appareil qui se resynchronise doit savoir qu'un message a été supprimé.

### 6.2 Conflit d'édition

- Deux éditions concurrentes sont résolues par **ordre chronologique** (horodatage serveur, puis ULID en cas d'égalité).
- La dernière édition **gagne** ; les éditions antérieures restent dans le journal (audit) mais ne sont pas appliquées.

### 6.3 Horloges

- L'horodatage `ts` est émis par le **client** (ISO 8601 UTC) mais **réécrit par le serveur** à la réception (l'horloge du serveur fait foi pour l'ordre).
- Le client ne doit **jamais** se fier à l'ordre d'arrivée pour trier : il trie par `ts` serveur puis par ULID.

## 7. Synchronisation E2EE

- Le contenu des messages est chiffré (Signal Protocol) : le serveur **relaie** les événements chiffrés sans pouvoir les lire (voir [`events.md`](events.md)).
- La synchronisation d'un **nouvel appareil** nécessite la récupération des **clés de session** via un canal sécurisé (voir l'architecture E2EE, à définir).
- Les événements de gestion (création de canal, permissions, présence) ne sont **pas** chiffrés : ils sont signés et vérifiés par le serveur.

## 8. Garanties

| Garantie | Mécanisme |
|---|---|
| Aucun événement perdu | Journal append-only + curseurs ULID |
| Ordre déterministe | Tri par `ts` serveur puis ULID |
| Reprise après coupure | `sync.request` avec dernier curseur |
| Cohérence multi-appareils | Tombstones + rejeu complet des événements |
| Confidentialité | Contenu E2EE, le serveur ne relaie que du chiffré |