# Fédération Myrmex

> Statut : **brouillon** — phase de conception / architecture
> Responsables : @Nixo et @notkiwwy
> Référence : *« n'importe quel opérateur peut déployer son propre serveur »* — le serveur principal n'est qu'un **bootstrap**, pas une dépendance permanente

## 1. Objectif

La fédération permet à des **serveurs Myrmex indépendants** d'échanger des événements :

- n'importe quel opérateur peut déployer son propre serveur ;
- le serveur principal (`myrmex.dev`) sert uniquement de **bootstrap / découverte**, pas de dépendance permanente ;
- le réseau doit **survivre** à l'indisponibilité du serveur principal ;
- le protocole doit être utilisable via **Tor** (services onion), sans présupposer une connexion Internet classique.

## 2. Identité des serveurs

- Chaque serveur possède un identifiant `srv_...` (ULID, voir [`identifiers.md`](identifiers.md)) et une **paire de clés Ed25519**.
- L'identifiant du serveur est **dérivé de sa clé publique** : deux serveurs qui se présentent avec le même `srv_...` doivent avoir la même clé publique, sinon l'échange est rejeté.
- La clé privée du serveur sert à **signer** les échanges inter-serveurs ; elle ne quitte jamais le serveur.

## 3. Découverte

### 3.1 Bootstrap

- Le serveur principal maintient un **annuaire** de serveurs publics (identifiant, clé publique, adresse de contact).
- Un nouveau serveur s'**enregistre** auprès du bootstrap en présentant son identifiant et sa clé publique, signés par sa clé privée.
- Le bootstrap **ne stocke pas** de contenu de conversation : il ne fait que de la découverte.

### 3.2 Indépendance

- Une fois la découverte faite, les serveurs communiquent **directement** entre eux.
- L'indisponibilité du bootstrap **n'interrompt pas** les échanges déjà établis.
- Chaque serveur peut **mettre en cache** les informations de découverte pour fonctionner sans bootstrap.

## 4. Authentification inter-serveurs

### 4.1 Poignée de main

1. Le serveur A contacte le serveur B (adresse directe ou via Tor).
2. A présente son identifiant `srv_...` et sa clé publique.
3. B vérifie que l'identifiant correspond bien à la clé publique (dérivation).
4. A signe un **challenge** (nonce) fourni par B : B vérifie la signature Ed25519.
5. Les deux serveurs négocient la **version de protocole** commune (voir [`versioning.md`](versioning.md)).
6. La connexion est établie ; chaque échange ultérieur est signé.

### 4.2 Signature des échanges

- Chaque événement relayé entre serveurs est signé par le serveur émetteur (en plus de la signature de l'auteur, voir [`events.md`](events.md)).
- Le serveur récepteur vérifie **les deux signatures** : celle de l'auteur (intégrité) et celle du serveur émetteur (authenticité de la provenance).

## 5. Relais d'événements

- Un événement créé sur le serveur A et destiné à des membres hébergés sur le serveur B est **relayé** de A vers B.
- Le relais respecte le **format d'événement commun** : aucun champ n'est ajouté ni retiré, seule la signature du serveur émetteur est ajoutée.
- Le contenu E2EE reste **chiffré de bout en bout** : les serveurs intermédiaires ne peuvent pas le lire.
- Les événements de gestion (membres, permissions, présence) sont relayés **signés** mais non chiffrés.

## 6. Synchronisation fédérée

- La synchronisation entre serveurs suit les mêmes principes que [`sync.md`](sync.md) : journal append-only, curseurs ULID, pagination.
- Chaque serveur maintient un **curseur** par serveur distant pour ne demander que les événements manquants.
- En cas de coupure entre deux serveurs, la reprise se fait par `sync.request` avec le dernier curseur connu.

## 7. Modération et blocage

- Un serveur peut **bloquer** un autre serveur : les événements entrants sont rejetés, les événements sortants ne sont plus relayés.
- Un serveur peut **bloquer un utilisateur** hébergé ailleurs : ses événements sont rejetés (liste de blocage locale).
- Le blocage est **local** à chaque serveur : il n'est pas propagé automatiquement (chaque opérateur garde la main sur sa modération).

## 8. Tor et réseaux alternatifs

- Chaque serveur peut exposer une **adresse onion** en plus de son adresse classique.
- La découverte peut référencer des adresses onion ; la connexion se fait alors via le réseau Tor.
- Le protocole **ne suppose pas** de connexion Internet classique : toutes les étapes (découverte, poignée de main, relais, synchronisation) fonctionnent de manière identique sur Tor.
- Tor réduit la traçabilité mais **n'est pas une anonymisation absolue** : cette limite est documentée et assumée.

## 9. Garanties

| Garantie | Mécanisme |
|---|---|
| Aucune dépendance permanente au serveur principal | Bootstrap = découverte uniquement, cache local |
| Authenticité des serveurs | Identifiant dérivé de la clé publique + challenge signé |
| Intégrité des événements | Double signature (auteur + serveur émetteur) |
| Confidentialité | E2EE de bout en bout, les serveurs ne voient que du chiffré |
| Survie aux coupures | Curseurs + reprise incrémentale |
| Utilisable hors Internet classique | Support natif des services onion |