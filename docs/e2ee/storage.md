# Sécurité du stockage local

> **Statut :** brouillon — phase de conception / architecture
> **Responsable :** @Nixo
> **Référence :** [docs/identity/keys.md](../identity/keys.md), [docs/identity/recovery.md](../identity/recovery.md)

## Principes

- Les clés privées (identité, appareil, sessions E2EE) sont stockées **chiffrées au repos**.
- Deux options selon la plateforme :
  - **Trousseau / keychain du système** (Windows Credential Manager, macOS Keychain, Linux Secret Service) ;
  - **Coffre local chiffré** dérivé de la phrase de récupération (voir [docs/identity/recovery.md](../identity/recovery.md)).
- Les clés ne sont **jamais** écrites en clair dans les logs, le cloud, les exports ou les sauvegardes.

## Données protégées

| Donnée | Protection |
|--------|-----------|
| Clé d'identité privée (Ed25519) | Trousseau ou coffre chiffré |
| Clé d'appareil privée (Ed25519) | Trousseau ou coffre chiffré |
| Clés de session E2EE (`sess_`) | Coffre chiffré, mémoire uniquement pendant l'utilisation |
| Prekeys privées (X25519) | Coffre chiffré |
| Phrase de récupération | Jamais stockée en clair ; affichée une seule fois à la création |

## Règles

- Le déchiffrement des clés se fait **en mémoire**, jamais persisté en clair.
- Les opérations sensibles (identité, crypto) restent dans le code **Rust** (crate `myrmex-crypto`), jamais dans le frontend.
- En cas de perte du trousseau et de la phrase de récupération, les clés sont irrécupérables — c'est un choix de conception (privacy by design).