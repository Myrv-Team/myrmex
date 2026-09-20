# Récupération cryptographique

> **Statut :** brouillon — phase de conception
> **Responsable :** @Nixo

## Principe

Sans e-mail ni numéro de téléphone, la récupération d'un compte ne peut pas
passer par le serveur : **le serveur ne détient pas les clés privées** et ne
peut pas réinitialiser un compte (règle n°2 du README). La récupération est
donc **cryptographique** : elle repose sur des secrets générés et conservés
par l'utilisateur.

## Mécanismes de récupération

### 1. Phrase de récupération (seed phrase)

- À la création du compte, l'application génère une **phrase de récupération**
  (liste de mots, par exemple 12 ou 24 mots, standard BIP-39 ou équivalent
  éprouvé — pas de crypto maison).
- La phrase permet de **re-dériver la clé d'identité** (et donc l'identifiant
  `usr_...`) sur un nouvel appareil.
- L'utilisateur doit la **noter et la conserver hors ligne** (papier, coffre).
- La phrase n'est **jamais** envoyée au serveur ni stockée en clair sur
  l'appareil.

### 2. Sauvegarde chiffrée des clés

- L'utilisateur peut exporter un **coffre chiffré** contenant ses clés
  (chiffré par un secret dérivé de la phrase de récupération).
- Le coffre peut être stocké où l'utilisateur le souhaite (disque local,
  clé USB, stockage cloud de son choix) — Myrmex ne l'héberge pas.

### 3. Appareils multiples

- Tant qu'au moins **un appareil actif** existe, un nouvel appareil peut être
  ajouté normalement (voir [`devices.md`](devices.md)).
- La récupération par phrase n'est nécessaire qu'en cas de **perte de tous les
  appareils**.

## Flux de récupération

1. L'utilisateur installe Myrmex sur un nouvel appareil et choisit
   « Récupérer un compte ».
2. Il saisit sa **phrase de récupération**.
3. L'application re-dérive la clé d'identité et l'identifiant `usr_...`.
4. L'application s'authentifie auprès du serveur par preuve de possession de la
   clé privée (voir [`sessions.md`](sessions.md)).
5. Le nouvel appareil est enregistré via `device.add` ; les anciens appareils
   peuvent être révoqués par l'utilisateur.

## Règles

- **Perte de la phrase + perte de tous les appareils = perte du compte.**
  Aucun mécanisme serveur ne peut le restaurer (c'est un choix de conception :
  le serveur ne détient aucune donnée permettant de reconstruire l'identité).
- La phrase de récupération est affichée **une seule fois** à la création du
  compte ; l'utilisateur doit confirmer qu'il l'a notée.
- La phrase n'est jamais transmise sur le réseau, jamais stockée par le
  serveur, jamais journalisée.
- La récupération ne révèle **aucune donnée personnelle** au serveur : elle
  n'utilise que la preuve cryptographique de possession de la clé.