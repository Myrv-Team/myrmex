# Myrmex

> Messagerie décentralisée, fédérée et orientée confidentialité.

Myrmex est une plateforme de communication instantanée inspirée des fonctionnalités communautaires de Discord, mais conçue autour de la confidentialité, du chiffrement de bout en bout et de la décentralisation.

L'objectif n'est pas de créer simplement un « Discord privé », mais de construire un réseau de communication dans lequel les utilisateurs peuvent communiquer sans dépendre d'une infrastructure centrale unique.

---

## Fonctionnalités

### Messagerie

* Messages privés
* Conversations de groupe
* Serveurs
* Salons textuels
* Réactions
* Réponses aux messages
* Mentions
* Partage de fichiers

### Confidentialité

* Comptes sans adresse e-mail
* Comptes sans numéro de téléphone
* Identité basée sur la cryptographie
* Chiffrement de bout en bout (E2EE)
* Minimisation des métadonnées
* Clés privées conservées côté client
* Support du réseau Tor

### Décentralisation

* Auto-hébergement
* Serveurs indépendants
* Fédération entre serveurs
* Communication serveur-à-serveur
* Possibilité de bloquer des serveurs
* Fonctionnement ne dépendant pas en permanence du serveur principal
* Support potentiel des services onion

---

# Architecture

Myrmex est divisé en trois grandes parties :

```text
                         MYRMEX
                            │
             ┌──────────────┴──────────────┐
             │                             │
          CLIENT                         SERVEUR
             │                             │
      Tauri 2 / React                  Rust / Axum
             │                             │
             └──────────────┬──────────────┘
                            │
                     MYRMEX PROTOCOL
                            │
                  ┌─────────┴─────────┐
                  │                   │
             E2EE / Crypto       Fédération
                  │                   │
                  └─────────┬─────────┘
                            │
                       PostgreSQL
```

Le client est responsable de l'interface, de l'identité utilisateur et des opérations cryptographiques nécessaires au fonctionnement de l'E2EE.

Le serveur est principalement responsable du routage, de la synchronisation, de la fédération et du stockage des données nécessaires au fonctionnement du réseau.

Le serveur ne doit jamais avoir accès aux clés privées permettant de déchiffrer les conversations.

---

# Philosophie technique

## Privacy by design

La confidentialité doit être prise en compte dès la conception.

Nous ne devons pas ajouter la sécurité après avoir construit le produit.

Chaque nouvelle fonctionnalité doit répondre à la question :

> Quelles informations cette fonctionnalité révèle-t-elle au serveur ou à un observateur réseau ?

---

## Pas d'identité réelle obligatoire

Un compte Myrmex ne nécessite :

* ni e-mail ;
* ni numéro de téléphone ;
* ni nom réel.

L'identité doit être créée localement à partir de données cryptographiques.

La clé privée d'un utilisateur ne doit jamais quitter son appareil.

---

## E2EE par défaut

Les conversations privées doivent être chiffrées de bout en bout.

Le serveur doit uniquement transporter et/ou stocker des données chiffrées.

Nous ne devons pas développer notre propre algorithme cryptographique.

Le projet utilisera des protocoles et bibliothèques cryptographiques éprouvés, notamment le Signal Protocol lorsque son modèle correspond aux besoins de Myrmex.

---

# Fédération

Myrmex doit permettre à n'importe quel opérateur de déployer son propre serveur.

Exemple :

```text
                 ┌──────────────┐
                 │ Serveur A    │
                 └──────┬───────┘
                        │
             ┌──────────┼──────────┐
             │          │          │
             ▼          ▼          ▼
       ┌──────────┐ ┌──────────┐ ┌──────────┐
       │Serveur B │ │Serveur C │ │Serveur D │
       └──────────┘ └──────────┘ └──────────┘
```

Les serveurs doivent pouvoir communiquer directement entre eux.

Le serveur principal pourra initialement servir au :

* bootstrap ;
* démarrage du réseau ;
* enregistrement initial ;
* mécanisme de découverte.

Il ne doit cependant pas devenir une dépendance permanente.

À terme, le réseau doit pouvoir continuer à fonctionner même si le serveur principal est indisponible.

---

# Tor

Myrmex devra être utilisable via Tor.

L'architecture doit notamment permettre aux utilisateurs et aux opérateurs de serveurs d'utiliser des services onion lorsque cela est pertinent.

L'intégration Tor doit être conçue sans supposer que l'utilisateur possède une connexion Internet classique.

La confidentialité réseau doit cependant être décrite avec précision : Tor réduit certaines possibilités de traçage, mais ne constitue pas une garantie d'anonymat absolu.

---

# Stack technique

## Client

* Tauri v2
* Rust
* React
* TypeScript
* Vite
* HTML
* CSS

Tauri constitue la couche desktop entre l'interface React et le cœur Rust.

Les opérations sensibles, notamment celles liées à l'identité et à la cryptographie, doivent rester autant que possible dans le code Rust.

```text
React / TypeScript
        │
        ▼
     Tauri 2
        │
        ▼
   Rust Core
        │
 ┌──────┼──────┐
 ▼      ▼      ▼
Crypto Protocol Network
```

---

## Backend

* Rust
* Tokio
* Axum
* WebSocket
* PostgreSQL

Le backend doit être modulaire afin de faciliter la fédération et l'évolution du protocole.

---

## Cryptographie

* Signal Protocol / protocole adapté aux besoins de Myrmex
* Bibliothèques cryptographiques reconnues
* Génération locale des clés
* Signatures cryptographiques
* Gestion des sessions
* Rotation et révocation des clés

**Aucune cryptographie maison.**

---

## Base de données

PostgreSQL est utilisé pour les données persistantes côté serveur.

La base ne doit pas être utilisée comme moyen de contourner l'E2EE.

Les contenus de conversations qui doivent rester privés doivent être stockés sous une forme chiffrée ou ne pas être stockés côté serveur lorsque cela est possible.

---

# Organisation du projet

L'organisation cible du repository est :

```text
myrmex/
│
├── apps/
│   ├── desktop/
│   │   ├── src/
│   │   └── src-tauri/
│   │
│   └── server/
│
├── crates/
│   ├── myrmex-core/
│   ├── myrmex-crypto/
│   ├── myrmex-protocol/
│   ├── myrmex-network/
│   ├── myrmex-federation/
│   └── myrmex-storage/
│
├── docs/
│   ├── api/
│   ├── e2ee/
│   ├── identity/
│   ├── protocol/
│   └── glossary.md
│
└── README.md
```

Cette organisation pourra évoluer pendant le développement.

---

# Répartition des tâches

## @*Nixo*

### Backend

Responsable du cœur serveur de Myrmex :

* Architecture backend
* API
* Axum
* Tokio
* Gestion des utilisateurs côté serveur
* Serveurs Myrmex
* Gestion des serveurs et salons
* Permissions
* Modération
* PostgreSQL
* Migrations
* Stockage
* Déploiement du serveur

### Authentification / identité

* Création des identités
* Gestion des clés publiques
* Sessions
* Gestion des appareils
* Révocation
* Récupération cryptographique

L'authentification ne doit pas dépendre d'un e-mail ou d'un numéro de téléphone.

### E2EE / cryptographie

Responsable de l'intégration cryptographique :

* Architecture E2EE
* Intégration du Signal Protocol ou protocole retenu
* Gestion des clés
* Sessions cryptographiques
* Chiffrement/déchiffrement
* Vérification des identités
* Rotation des clés
* Sécurité du stockage local

Les primitives cryptographiques existantes doivent être privilégiées.

### Protocole

Responsable avec @notkiwwy de la définition du protocole Myrmex :

* Format des événements
* Identifiants
* Synchronisation
* Versionnement
* Compatibilité entre versions
* Fédération

---

## @Shoppixx

### Frontend

Responsable du client Myrmex :

* React
* TypeScript
* Tauri v2
* Architecture frontend
* Gestion de l'état
* Navigation
* Composants
* Affichage des conversations
* Affichage des serveurs
* Affichage des salons
* Gestion des fichiers côté interface
* Paramètres utilisateur

### UI / UX

Responsable de l'identité visuelle et de l'expérience utilisateur :

* Design system
* Interface de messagerie
* Serveurs
* Salons
* Profils
* Paramètres
* États de connexion
* États de synchronisation
* Erreurs
* Accessibilité
* Responsive design

L'interface doit avoir sa propre identité et ne pas être une copie visuelle de Discord.

---

## @notkiwwy

### Réseau

Responsable de la couche réseau :

* Connexions client ↔ serveur
* Connexions serveur ↔ serveur
* Gestion des connexions
* Reconnexion
* Routage
* Gestion des erreurs réseau
* Support Tor

### WebSocket

* Connexions temps réel
* Gestion des événements
* Heartbeat
* Reconnexion
* Gestion des déconnexions
* Synchronisation des événements

### Synchronisation

* Synchronisation des messages
* Synchronisation des conversations
* Synchronisation multi-appareils
* Gestion des événements manquants
* Reprise après déconnexion
* Ordonnancement
* Détection des doublons

### Fédération

Travail en collaboration avec @*Nixo* sur :

* Protocole serveur ↔ serveur
* Authentification des serveurs
* Échange d'événements
* Découverte
* Gestion des serveurs bloqués
* Résilience du réseau

---

# Travail en équipe

Certaines parties ne doivent pas être développées indépendamment.

Les trois développeurs doivent notamment se coordonner sur :

### Protocole

Le protocole doit être défini avant de multiplier les implémentations.

### E2EE

Le frontend, le backend et le réseau doivent tous respecter les mêmes hypothèses cryptographiques.

### Fédération

Le format des événements et leur synchronisation doivent être communs au serveur et au client.

### API

Toute modification de l'API doit être documentée afin d'éviter de casser le frontend.

---

# Règles de développement

## 1. Ne jamais faire confiance au client

Toutes les permissions importantes doivent être vérifiées côté serveur.

## 2. Ne jamais transmettre une clé privée au serveur

Une clé privée appartient exclusivement à son appareil.

## 3. Ne pas inventer de cryptographie

Utiliser des protocoles et bibliothèques reconnus.

## 4. Ne pas stocker inutilement de données personnelles

Chaque donnée persistée doit avoir une justification technique.

## 5. Documenter le protocole

Toute modification du protocole doit être documentée.

## 6. Tester les fonctionnalités critiques

Les éléments suivants doivent disposer de tests :

* authentification ;
* permissions ;
* E2EE ;
* synchronisation ;
* fédération ;
* reconnexion ;
* gestion des appareils.

---

# MVP

La première version doit se concentrer sur le fonctionnement fondamental du réseau.

### Identité

* [ ] Création d'un compte sans e-mail
* [ ] Création d'un compte sans numéro
* [ ] Génération locale des clés
* [ ] Gestion des appareils
* [ ] Révocation

### Messagerie

* [ ] Conversations privées
* [ ] Messages temps réel
* [ ] Modification des messages
* [ ] Suppression
* [ ] Réactions
* [ ] Réponses
* [ ] E2EE

### Serveurs

* [ ] Création d'un serveur
* [ ] Salons
* [ ] Membres
* [ ] Permissions
* [ ] Modération basique

### Fédération

* [ ] Protocole serveur ↔ serveur
* [ ] Découverte
* [ ] Authentification des serveurs
* [ ] Synchronisation
* [ ] Blocage d'un serveur

### Réseau

* [ ] WebSocket
* [ ] Reconnexion
* [ ] Synchronisation après déconnexion
* [ ] Support Tor

### Client

* [ ] Application Tauri 2
* [ ] Interface React
* [ ] Gestion des conversations
* [ ] Gestion des serveurs
* [ ] Gestion des paramètres
* [ ] Gestion locale de l'identité

---

# Priorité de développement

L'ordre recommandé est :

```text
1. Architecture du protocole
          ↓
2. Identité cryptographique
          ↓
3. E2EE
          ↓
4. Communication client ↔ serveur
          ↓
5. Synchronisation
          ↓
6. Serveurs fédérés
          ↓
7. Support Tor
          ↓
8. Interface complète
          ↓
9. Fonctionnalités communautaires
```

Il est important de ne pas construire toute l'interface avant d'avoir stabilisé les concepts fondamentaux du protocole.

---

# Objectif final

Myrmex doit permettre à plusieurs serveurs indépendants de former un réseau commun sans qu'un serveur central soit nécessaire pour chaque communication.

```text
             ┌──────────────┐
             │  Serveur A   │
             └──────┬───────┘
                    │
          ┌─────────┼─────────┐
          │         │         │
          ▼         ▼         ▼
     Serveur B  Serveur C  Serveur D
          │         │         │
          └─────────┼─────────┘
                    │
               Serveur E
```

Les conversations restent E2EE.

Les clés privées restent côté utilisateur.

Les serveurs peuvent être auto-hébergés.

Les serveurs peuvent communiquer entre eux.

Tor peut être utilisé pour réduire l'exposition réseau.

Le serveur principal constitue un point de bootstrap, pas une autorité indispensable.

À terme, Myrmex doit être capable de fonctionner comme un **réseau de communication fédéré et privacy-first**, plutôt que comme une simple application de messagerie centralisée.

---

## État du projet

**Phase actuelle : conception / architecture**

Priorités immédiates :

1. Définir le protocole Myrmex.
2. Définir le modèle d'identité.
3. Définir l'architecture E2EE.
4. Définir le protocole de fédération.
5. Définir la synchronisation.
6. Mettre en place le workspace Rust/Tauri 2.
7. Construire un prototype minimal client ↔ serveur.
