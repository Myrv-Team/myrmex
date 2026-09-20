# Sessions cryptographiques E2EE

> **Statut :** brouillon — phase de conception / architecture
> **Responsable :** @Nixo
> **Référence :** [docs/identity/sessions.md](../identity/sessions.md)

## Définition

Une session E2EE (`sess_` + ULID) est un canal chiffré entre **deux appareils** (émetteur → destinataire). Elle est indépendante de la session d'authentification (token opaque, voir [docs/identity/sessions.md](../identity/sessions.md)).

## États d'une session

| État | Description |
|------|-------------|
| `pending` | Session en cours d'établissement (X3DH initié, premier message non confirmé) |
| `established` | Session active, Double Ratchet en cours |
| `expired` | Session arrivée en fin de vie (rotation, inactivité prolongée) |
| `revoked` | Session invalidée par la révocation d'un appareil |

## Cycle de vie

1. **Initiation** : l'émetteur récupère le prekey bundle du destinataire et exécute X3DH (voir [signal.md](signal.md)). Le premier message porte `"prekey": true`.
2. **Établissement** : le destinataire déchiffre le premier message, répond, et la session passe à `established`.
3. **Utilisation** : chaque message renouvelle les clés de chaîne (ratchet symétrique) et, périodiquement, les clés de session (ratchet DH).
4. **Fin de vie** : la session expire ou est révoquée (voir [keys.md](keys.md)).

## Stockage local

Les sessions sont stockées chiffrées localement (voir [storage.md](storage.md)). Elles ne sont **jamais** envoyées au serveur — seul l'identifiant `sess_` apparaît dans le payload E2EE (voir [docs/protocol/events.md](../protocol/events.md)).

## Règles

- Une session est liée à un couple (appareil émetteur, appareil destinataire).
- La révocation d'un appareil invalide toutes ses sessions (`revoked`).
- `401` : tentative d'utilisation d'une session expirée ou révoquée.
- Le serveur relaie les messages chiffrés sans connaître l'état des sessions.