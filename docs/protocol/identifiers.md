# Identifiants

> **Statut :** brouillon — phase de conception
> **Responsables :** @Nixo et @notkiwwy

## Principes

- **Aucun e-mail, aucun numéro de téléphone** : l'identité est basée sur des
  clés cryptographiques générées localement (voir README, section MVP — Identité).
- Les identifiants sont **opaques** : ils ne révèlent aucune information
  personnelle.
- Un identifiant est **unique globalement** (pas seulement par serveur) pour
  permettre la fédération.

## Format : ULID

Tous les identifiants utilisent le format **ULID** (128 bits, encodage Crockford
Base32, 26 caractères) :

- **Triables par temps** : l'ordre lexicographique = ordre chronologique.
- **Uniques** sans coordination centralisée (48 bits de timestamp + 80 bits
  aléatoires).
- Compatibles avec la synchronisation incrémentale (voir [`sync.md`](sync.md)).

Exemple : `01HZ8K2Q3X4Y5Z6A7B8C9D0E1F`

## Préfixes

Chaque type d'entité porte un préfixe lisible pour éviter les collisions et
faciliter le débogage :

| Préfixe | Entité | Exemple |
|---------|--------|---------|
| `usr_` | Utilisateur | `usr_01HZ8K2Q3X4Y5Z6A7B8C9D0E1F` |
| `dev_` | Appareil | `dev_01HZ8K2Q3X4Y5Z6A7B8C9D0E1F` |
| `srv_` | Serveur (instance fédérée) | `srv_01HZ8K2Q3X4Y5Z6A7B8C9D0E1F` |
| `chn_` | Canal / conversation | `chn_01HZ8K2Q3X4Y5Z6A7B8C9D0E1F` |
| `msg_` | Message | `msg_01HZ8K2Q3X4Y5Z6A7B8C9D0E1F` |
| `evt_` | Événement | `evt_01HZ8K2Q3X4Y5Z6A7B8C9D0E1F` |
| `sess_` | Session E2EE | `sess_01HZ8K2Q3X4Y5Z6A7B8C9D0E1F` |
| `k_` | Clé (identifiant de clé) | `k_01HZ8K2Q3X4Y5Z6A7B8C9D0E1F` |

## Identité utilisateur

- L'identité est ancrée sur une **paire de clés Ed25519** générée localement.
- L'identifiant `usr_...` est dérivé de l'empreinte de la clé publique
  (hash SHA-256 tronqué, encodé ULID).
- La clé privée **ne quitte jamais l'appareil** (règle n°2 du README).
- Un utilisateur peut avoir **plusieurs appareils** (`dev_...`), chacun avec sa
  propre paire de clés, liée à l'identité principale.

## Appareils

- Chaque appareil possède un identifiant `dev_...` et une paire de clés.
- Les appareils sont **gérés par l'utilisateur** : ajout, liste, révocation
  (événements `device.add` / `device.revoke`, voir [`events.md`](events.md)).
- La révocation d'un appareil invalide ses clés de session E2EE.

## Serveurs (fédération)

- Chaque serveur fédéré possède un identifiant `srv_...` et une paire de clés
  **Ed25519** servant à signer les échanges inter-serveurs
  (voir [`federation.md`](federation.md)).
- L'identifiant est dérivé de la clé publique du serveur, comme pour les
  utilisateurs.

## Canaux et conversations

- `chn_...` identifie un canal (dans un serveur) **ou** une conversation privée
  (1:1 ou de groupe).
- Pour une conversation privée, l'identifiant est dérivé de façon déterministe
  des identifiants des participants (ordre canonique), ce qui permet aux deux
  parties de retrouver la même conversation sans coordination.

## Règles de validation

- Longueur : préfixe (4) + ULID (26) = **30 caractères**.
- Caractères autorisés : `[0-9A-Z]` (Base32 Crockford, sans `I`, `L`, `O`, `U`).
- Les identifiants sont **sensibles à la casse** (majuscules uniquement).
- Toute entité reçue avec un identifiant invalide est rejetée (`400`).