# Vérification des identités

> **Statut :** brouillon — phase de conception / architecture
> **Responsable :** @Nixo
> **Référence :** [docs/identity/keys.md](../identity/keys.md)

## Objectif

Protéger contre l'attaque de l'homme du milieu (MITM) : s'assurer que la clé d'identité publique d'un contact est bien la sienne, même si le serveur est compromis.

## Safety numbers

Chaque paire d'utilisateurs peut calculer un **safety number** (empreinte dérivée des clés d'identité publiques des deux parties, via SHA-256). Il est affiché sous forme de groupe de chiffres lisibles.

## Vérification hors-bande

1. Les deux utilisateurs comparent leur safety number **hors-bande** (en personne, appel vidéo, QR code, etc.).
2. Si les empreintes correspondent, chaque client marque l'identité du contact comme **vérifiée**.
3. Toute divergence (clé d'identité changée) déclenche un avertissement explicite.

## États de vérification

| État | Description |
|------|-------------|
| `unverified` | Aucune vérification effectuée (par défaut) |
| `verified` | Safety number confirmé hors-bande |
| `changed` | La clé d'identité du contact a changé depuis la dernière vérification |

## Règles

- La vérification est **locale à l'appareil** : elle n'est jamais envoyée au serveur.
- Un changement de clé d'identité (nouveau compte) réinitialise l'état à `changed` et bloque l'envoi tant que l'utilisateur n'a pas confirmé.
- La vérification ne repose sur aucune autorité centrale (pas de PKI serveur).