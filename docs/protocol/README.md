# Protocole Myrmex

> **Statut :** brouillon — phase de conception / architecture
> **Responsables :** @Nixo et @notkiwwy
> **Règle n°5 du README :** toute modification du protocole doit être documentée ici.

Le protocole Myrmex est le **composant central** du projet : il conditionne la
fédération et l'E2EE (voir [`README.txt`](../../README.txt)). Il doit être défini
**avant** de multiplier les implémentations, et le format des événements ainsi que
leur synchronisation doivent être **communs au serveur et au client**.

## Objectifs

- Définir un format d'échange unique, versionné et extensible.
- Garantir la compatibilité entre versions (serveur ↔ client, serveur ↔ serveur).
- Permettre la fédération : n'importe quel opérateur peut déployer son propre serveur.
- Ne jamais compromettre l'E2EE : le serveur ne doit jamais voir le contenu en clair.
- Fonctionner sans dépendance permanente au serveur principal (bootstrap uniquement).
- Être utilisable via Tor (services onion), sans présupposer une connexion Internet classique.

## Documents

| Document | Contenu |
|----------|---------|
| [`events.md`](events.md) | Format des événements (enveloppe, types, payload, signature) |
| [`identifiers.md`](identifiers.md) | Schéma d'identifiants (utilisateurs, appareils, serveurs, canaux, messages) |
| [`versioning.md`](versioning.md) | Versionnement du protocole et compatibilité entre versions |
| [`sync.md`](sync.md) | Synchronisation (journal d'événements, curseurs, reprise après déconnexion) |
| [`federation.md`](federation.md) | Fédération entre serveurs (découverte, authentification, relais, blocage) |
| [`errors.md`](errors.md) | Codes d'erreur normalisés (format, raisons, règles transverses) |
| [`examples.md`](examples.md) | Exemples JSON concrets (enveloppe, E2EE, sync, fédération) |

## Principes directeurs

1. **Le serveur ne fait jamais confiance au client** (règle n°1 du README).
2. **Aucune clé privée n'est envoyée au serveur** (règle n°2 du README).
3. **Aucune cryptographie maison** : Signal Protocol, Ed25519, X25519 (règle n°3).
4. **Pas de données personnelles inutiles** (règle n°4).
5. **Toute modification du protocole est documentée** (règle n°5).
6. **Le format des événements et la synchronisation sont communs** au serveur et au client.
7. **Toute modification de l'API est documentée** pour ne pas casser le frontend.

## Règles de modification

- Toute évolution du protocole passe par une mise à jour de ce dossier **et** de
  [`docs/DEVELOPMENT.md`](../DEVELOPMENT.md).
- Une modification **cassante** (MAJOR) doit être annoncée et documentée avant
  d'être déployée (voir [`versioning.md`](versioning.md)).
- Les changements sont discutés avec @notkiwwy (réseau, sync, fédération) avant
  implémentation.