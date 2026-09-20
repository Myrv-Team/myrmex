# Glossaire commun

> **Statut** : brouillon
> **Responsables** : @Nixo, @Shoppixx, @notkiwwy

Termes partagés par toute l'équipe. Ce glossaire est la référence unique : si un terme est ambigu, il est défini ici.

## Identité et appareils

| Terme | Définition |
| --- | --- |
| **Identité (`usr_`)** | Identifiant opaque dérivé de l'empreinte de la clé publique Ed25519 (SHA-256 tronqué, encodé ULID). Aucun email ni téléphone. Voir [`identifiers.md`](protocol/identifiers.md). |
| **Appareil (`dev_`)** | Instance du client (ordinateur, téléphone). Chaque appareil possède sa propre paire de clés et est lié à l'identité. Voir [`devices.md`](identity/devices.md). |
| **Serveur (`srv_`)** | Nœud du réseau fédéré. Son identité est dérivée de sa clé publique Ed25519. Voir [`federation.md`](protocol/federation.md). |
| **Canal (`chn_`)** | Canal ou conversation. Pour une conversation privée, dérivé déterministiquement des identifiants des participants. |
| **ULID** | Identifiant 128 bits, encodé Crockford Base32 (26 caractères), triable chronologiquement. Format : préfixe (4) + ULID (26) = 30 caractères. Voir [`identifiers.md`](protocol/identifiers.md). |

## Protocole et événements

| Terme | Définition |
| --- | --- |
| **Enveloppe commune** | Format JSON unique de tout événement : `{v, id, type, ts, author, device, server, channel, sig, payload}`. Voir [`events.md`](protocol/events.md). |
| **Événement** | Unité atomique d'échange (message, réaction, présence…). Immuable, signé (Ed25519), éventuellement chiffré E2EE. |
| **Journal (append-only)** | Historique d'événements d'un canal ou serveur : on ne modifie ni ne supprime jamais, on ajoute. Voir [`sync.md`](protocol/sync.md). |
| **Curseur** | ULID `evt_` du dernier événement reçu ; permet de reprendre une synchronisation. `null` = synchronisation complète. |
| **Tombstone** | Marqueur de suppression : l'événement d'origine reste dans le journal, un événement de suppression le masque. |
| **Double signature** | En fédération, un événement est signé par l'auteur **et** par le serveur émetteur. Voir [`federation.md`](protocol/federation.md). |
| **Bootstrap** | Serveur d'annuaire initial (myrmex.dev) permettant la découverte d'autres serveurs. |

## Cryptographie et E2EE

| Terme | Définition |
| --- | --- |
| **E2EE** | Chiffrement de bout en bout : le serveur ne peut jamais déchiffrer le contenu. Voir [`e2ee/README.md`](e2ee/README.md). |
| **Signal Protocol** | Protocole de chiffrement utilisé (X3DH + Double Ratchet). Aucune cryptographie maison. Voir [`signal.md`](e2ee/signal.md). |
| **X3DH** | Établissement de clé initial entre deux appareils (clé pré-partagée + éphémère). |
| **Double Ratchet** | Dérivation de clés de session après l'établissement initial ; gère les messages hors ordre. |
| **Prekey** | Clé pré-partagée publiée par un appareil ; utilisée uniquement pour le premier message d'une session (`"prekey": true`). |
| **Session E2EE (`sess_`)** | Session de chiffrement entre deux appareils, distincte de la session d'authentification. |
| **Nonce** | Valeur à usage unique (challenge d'authentification ou IV de chiffrement). |
| **Ed25519** | Signature numérique (clés de signature). |
| **X25519** | Échange de clés Diffie-Hellman sur Curve25519. |
| **SHA-256 / HKDF** | Hachage et dérivation de clés. |
| **AES-256 AEAD** | Chiffrement authentifié des messages. |

## Sessions et authentification

| Terme | Définition |
| --- | --- |
| **Session d'authentification** | Token opaque, non persistant, lié à un appareil (`dev_`), obtenu après signature d'un challenge. Voir [`sessions.md`](identity/sessions.md). |
| **Challenge** | Nonce à usage unique horodaté envoyé par le serveur ; le client le signe pour prouver la possession de sa clé privée. |
| **Token opaque** | Jeton de session stocké côté serveur sous forme de hash, jamais en clair. |

## Réseau et fédération

| Terme | Définition |
| --- | --- |
| **Fédération** | Interconnexion de serveurs indépendants qui échangent des événements. Voir [`federation.md`](protocol/federation.md). |
| **Serveur d'origine** | Serveur qui reçoit un événement d'un client et le relaie au réseau. |
| **Onion / Tor** | Support des adresses `.onion` : le protocole fonctionne à l'identique, sans anonymat absolu garanti. |
| **WebSocket** | Canal temps réel client ↔ serveur (push d'événements). Voir [`endpoints.md`](api/endpoints.md). |