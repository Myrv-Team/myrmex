# Chiffrement et déchiffrement des messages

> **Statut :** brouillon — phase de conception / architecture
> **Responsable :** @Nixo
> **Référence :** [docs/protocol/events.md](../protocol/events.md)

## Flux de chiffrement (émetteur)

1. Le client construit le message en clair (texte, pièces jointes chiffrées, etc.).
2. Il chiffre avec la session E2EE du destinataire (Double Ratchet, voir [signal.md](signal.md)).
3. Il construit le payload E2EE :

```json
{
  "ciphertext": "base64url(...)",
  "nonce": "base64url(...)",
  "session_id": "sess_01HZ8K2Q3X...",
  "sender_key_id": "k_01HZ8K2Q3X...",
  "prekey": false
}
```

4. Il signe l'enveloppe d'événement avec sa clé d'appareil (Ed25519) et l'envoie au serveur.

> Format exact de l'enveloppe : [docs/protocol/events.md](../protocol/events.md). Le champ `prekey` vaut `true` uniquement pour le premier message d'une session (X3DH).

## Flux de déchiffrement (destinataire)

1. Le client reçoit l'événement `message.create` et vérifie la signature Ed25519 de l'enveloppe.
2. Il retrouve la session locale `sess_` correspondante.
3. Il déchiffre `ciphertext` avec le `nonce` et la clé de session.
4. En cas de premier message (`"prekey": true`), il exécute la partie X3DH côté destinataire avant le Double Ratchet.

## Gestion des messages hors ordre

Le Double Ratchet permet de déchiffrer des messages reçus dans le désordre (clés de chaîne conservées en mémoire tampon). Les messages trop anciens ou issus d'une session révoquée sont rejetés.

## Limites

- Payload E2EE : **60 Ko maximum** → erreur `413` (voir [docs/protocol/events.md](../protocol/events.md)).
- Les pièces jointes volumineuses sont chiffrées séparément (clé de fichier) et référencées dans le message chiffré — le serveur ne voit que des blobs chiffrés.

## Règles

- Le serveur ne déchiffre jamais : il vérifie la signature de l'enveloppe et relaie.
- `401` : signature d'enveloppe invalide.
- `403` : appareil révoqué ou session non autorisée.