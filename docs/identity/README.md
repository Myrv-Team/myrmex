# Modèle d'identité

> **Statut :** brouillon — phase de conception / architecture
> **Responsable :** @Nixo

Ce dossier définit le **modèle d'identité** de Myrmex : comment un compte est
créé, comment les clés cryptographiques sont générées et gérées, comment les
appareils sont liés à une identité, comment l'authentification fonctionne sans
e-mail ni numéro de téléphone, et comment la récupération cryptographique est
assurée.

## Objectifs

- Créer un compte **sans e-mail, sans numéro de téléphone, sans nom réel**
  (voir README, section *Pas d'identité réelle obligatoire*).
- Ancrer l'identité sur des **clés cryptographiques générées localement**.
- Permettre la **gestion des appareils** (ajout, liste, révocation).
- Assurer la **récupération cryptographique** en cas de perte d'appareil.
- Ne **jamais** envoyer de clé privée au serveur (règle n°2 du README).

## Principes

1. **Pas d'identité réelle obligatoire** : ni e-mail, ni téléphone, ni nom
   réel. L'identité est créée localement à partir de données cryptographiques.
2. **La clé privée ne quitte jamais l'appareil** (règle n°2). Le serveur ne
   reçoit que des clés publiques et des signatures.
3. **Pas de cryptographie maison** (règle n°3) : Ed25519, X25519, SHA-256,
   Signal Protocol — uniquement des primitives éprouvées.
4. **Données minimales** (règle n°4) : le serveur ne stocke que ce qui est
   strictement nécessaire au fonctionnement (identifiants, clés publiques,
   métadonnées d'appareil minimales).
5. **Privacy by design** : pour chaque fonctionnalité, se demander
   *« Quelles informations cette fonctionnalité révèle-t-elle au serveur ou à
   un observateur réseau ? »* et réduire au minimum.
6. **L'authentification ne dépend d'aucune donnée personnelle** : elle repose
   sur la preuve de possession d'une clé privée.

## Documents

| Document | Contenu |
|----------|---------|
| [`account.md`](account.md) | Création de compte sans e-mail ni téléphone, identité ancrée sur une paire de clés Ed25519 |
| [`keys.md`](keys.md) | Génération locale des clés, hiérarchie, stockage sécurisé, gestion des clés publiques |
| [`devices.md`](devices.md) | Gestion des appareils : ajout, liste, révocation |
| [`sessions.md`](sessions.md) | Sessions d'authentification et sessions E2EE |
| [`recovery.md`](recovery.md) | Récupération cryptographique en cas de perte d'appareil |

## Liens avec le protocole

- Les identifiants (`usr_`, `dev_`, `sess_`, `k_`) sont définis dans
  [`docs/protocol/identifiers.md`](../protocol/identifiers.md).
- Les événements `device.add` / `device.revoke` sont définis dans
  [`docs/protocol/events.md`](../protocol/events.md).
- Le chiffrement de bout en bout (Signal Protocol) est défini dans
  [`docs/protocol/events.md`](../protocol/events.md) (payload E2EE) et sera
  détaillé dans le dossier `docs/e2ee/` (priorité n°3).