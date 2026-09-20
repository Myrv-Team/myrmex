//! Types métier partagés du projet Myrmex.
//!
//! Ce crate est le socle commun de tout le projet : identifiants (ULID
//! préfixés), identité et appareils, serveurs, canaux, membres et
//! permissions. Il ne contient ni réseau ni cryptographie : ce sont des
//! types purs, sérialisables, partagés par le serveur, le client et le
//! protocole.
//!
//! Spécifications : `docs/protocol/identifiers.md`, `docs/glossary.md`.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod identifiers;
pub mod identity;
pub mod permissions;
pub mod server;

pub use identifiers::{
    ChannelId, DeviceId, EventId, Identifier, IdentifierError, KeyId, MessageId, ServerId,
    SessionId, Ulid, UserId,
};
pub use identity::{Device, DeviceStatus, Identity};
pub use permissions::Permission;
pub use server::{Channel, ChannelKind, Member, Role, Server};