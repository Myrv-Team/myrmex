//! Serveurs, canaux et membres.
//!
//! Un serveur regroupe des canaux et des membres. Un canal peut appartenir
//! à un serveur (canal de serveur) ou être une conversation privée entre
//! deux utilisateurs (canal sans serveur).
//!
//! Spécifications : `docs/glossary.md`, `docs/protocol/events.md`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::identifiers::{ChannelId, ServerId, UserId};

/// Serveur (guild) : espace de discussion fédéré.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Server {
    /// Identifiant du serveur.
    pub id: ServerId,
    /// Nom du serveur.
    pub name: String,
    /// Description facultative du serveur.
    pub description: Option<String>,
    /// Utilisateur propriétaire du serveur.
    pub owner: UserId,
    /// Date de création du serveur.
    pub created_at: DateTime<Utc>,
}

/// Nature d'un canal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelKind {
    /// Canal de discussion textuelle.
    Text,
    /// Canal vocal.
    Voice,
}

/// Canal de discussion.
///
/// Un canal est rattaché à un serveur (`server`), sauf pour les
/// conversations privées où `server` est `None` et `parent` peut porter
/// l'identifiant de la conversation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Channel {
    /// Identifiant du canal.
    pub id: ChannelId,
    /// Serveur parent, ou `None` pour une conversation privée.
    pub server: Option<ServerId>,
    /// Nom du canal.
    pub name: String,
    /// Nature du canal (texte ou vocal).
    pub kind: ChannelKind,
    /// Canal parent éventuel (sous-canaux, conversations privées).
    pub parent: Option<ChannelId>,
    /// Date de création du canal.
    pub created_at: DateTime<Utc>,
}

/// Rôle d'un membre dans un serveur.
///
/// La liste n'est pas figée par la spécification : elle est volontairement
/// marquée `#[non_exhaustive]` pour pouvoir évoluer sans casser les
/// consommateurs du crate.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Role {
    /// Propriétaire du serveur.
    Owner,
    /// Administrateur.
    Admin,
    /// Modérateur.
    Moderator,
    /// Membre simple.
    Member,
}

/// Membre d'un serveur.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Member {
    /// Serveur auquel le membre appartient.
    pub server: ServerId,
    /// Utilisateur membre.
    pub user: UserId,
    /// Rôles du membre (peut être vide pour un simple membre).
    pub roles: Vec<Role>,
    /// Date d'arrivée du membre sur le serveur.
    pub joined_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canal_de_serveur() {
        let server = ServerId::generate();
        let channel = Channel {
            id: ChannelId::generate(),
            server: Some(server),
            name: "général".to_string(),
            kind: ChannelKind::Text,
            parent: None,
            created_at: Utc::now(),
        };
        assert_eq!(channel.server, Some(server));
        assert_eq!(channel.kind, ChannelKind::Text);
    }

    #[test]
    fn conversation_privee_sans_serveur() {
        let channel = Channel {
            id: ChannelId::generate(),
            server: None,
            name: "privé".to_string(),
            kind: ChannelKind::Text,
            parent: None,
            created_at: Utc::now(),
        };
        assert_eq!(channel.server, None);
    }

    #[test]
    fn roles_du_membre() {
        let member = Member {
            server: ServerId::generate(),
            user: UserId::generate(),
            roles: vec![Role::Moderator],
            joined_at: Utc::now(),
        };
        assert!(member.roles.contains(&Role::Moderator));
        assert!(!member.roles.contains(&Role::Admin));
    }
}