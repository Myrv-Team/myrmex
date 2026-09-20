//! Permissions.
//!
//! Les permissions ne sont pas finalisées dans la spécification : seul le
//! code d'erreur `forbidden` (« Permissions insuffisantes (rôle,
//! modération, etc.) ») est défini dans `docs/protocol/errors.md`. Cette
//! liste est donc une base minimale et lisible, à affiner lors du
//! développement du serveur.
//!
//! La liste est marquée `#[non_exhaustive]` : ajouter une permission ne
//! cassera pas les consommateurs du crate.

use serde::{Deserialize, Serialize};

/// Permission accordée à un rôle ou à un membre.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    /// Lire les messages d'un canal.
    ReadMessages,
    /// Envoyer des messages dans un canal.
    SendMessages,
    /// Modérer les messages (suppression, épinglage).
    ManageMessages,
    /// Créer, renommer ou supprimer des canaux.
    ManageChannels,
    /// Gérer les membres (invitations, rôles).
    ManageMembers,
    /// Expulser un membre.
    KickMembers,
    /// Bannir un membre.
    BanMembers,
    /// Administrer le serveur (paramètres, suppression).
    ManageServer,
}