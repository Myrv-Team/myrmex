//! Identité et appareils.
//!
//! L'identité d'un utilisateur est ancrée par une clé publique Ed25519 :
//! l'identifiant `usr_` en est dérivé (empreinte SHA-256). Les appareils
//! portent chacun leur propre paire de clés et peuvent être révoqués.
//!
//! Spécifications : `docs/identity/keys.md`, `docs/identity/devices.md`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::identifiers::{DeviceId, UserId};

/// Identité publique d'un utilisateur.
///
/// Ne contient jamais de clé privée : les clés privées ne quittent pas
/// l'appareil (règle n° 2 du projet).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identity {
    /// Identifiant dérivé de l'empreinte de la clé publique.
    pub id: UserId,
    /// Clé publique Ed25519 (32 octets) qui ancre l'identité.
    pub public_key: [u8; 32],
    /// Date de création du compte.
    pub created_at: DateTime<Utc>,
}

/// Appareil enregistré pour un utilisateur.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Device {
    /// Identifiant de l'appareil.
    pub id: DeviceId,
    /// Utilisateur propriétaire de l'appareil.
    pub owner: UserId,
    /// Clé publique Ed25519 de l'appareil (32 octets).
    pub public_key: [u8; 32],
    /// Nom libre choisi par l'utilisateur (ex. « Téléphone »).
    pub name: Option<String>,
    /// Date d'enregistrement de l'appareil.
    pub created_at: DateTime<Utc>,
    /// Date de révocation, si l'appareil a été révoqué.
    pub revoked_at: Option<DateTime<Utc>>,
}

/// État d'un appareil.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceStatus {
    /// L'appareil est actif et peut signer des événements.
    Active,
    /// L'appareil a été révoqué et ne peut plus signer.
    Revoked,
}

impl Device {
    /// État courant de l'appareil.
    pub fn status(&self) -> DeviceStatus {
        if self.revoked_at.is_some() {
            DeviceStatus::Revoked
        } else {
            DeviceStatus::Active
        }
    }

    /// L'appareil est-il actif ?
    pub fn is_active(&self) -> bool {
        self.status() == DeviceStatus::Active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn device(revoked_at: Option<DateTime<Utc>>) -> Device {
        Device {
            id: DeviceId::generate(),
            owner: UserId::generate(),
            public_key: [7; 32],
            name: Some("Test".to_string()),
            created_at: Utc::now(),
            revoked_at,
        }
    }

    #[test]
    fn appareil_actif() {
        let d = device(None);
        assert!(d.is_active());
        assert_eq!(d.status(), DeviceStatus::Active);
    }

    #[test]
    fn appareil_revoque() {
        let d = device(Some(Utc::now()));
        assert!(!d.is_active());
        assert_eq!(d.status(), DeviceStatus::Revoked);
    }
}