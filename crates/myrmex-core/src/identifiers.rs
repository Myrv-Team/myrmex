//! Identifiants Myrmex (ULID préfixés).
//!
//! Chaque entité du système porte un identifiant unique composé d'un préfixe
//! lisible (ex. `usr_`, `srv_`) suivi d'un ULID de 26 caractères. L'ULID est
//! un identifiant de 128 bits, triable chronologiquement, encodé en Base32
//! Crockford (alphabet `0-9A-Z` sans `I`, `L`, `O`, `U`), en majuscules.
//!
//! Spécification : `docs/protocol/identifiers.md`.
//!
//! # Exemple
//!
//! ```
//! use myrmex_core::identifiers::{UserId, Identifier};
//!
//! let id = UserId::generate();
//! let text = id.to_string();
//! let parsed: UserId = text.parse().expect("un identifiant valide");
//! assert_eq!(id, parsed);
//! ```

use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::de::{self, Deserializer};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

pub use ulid::Ulid;

/// Préfixe des identifiants d'utilisateur.
pub const USR: &str = "usr_";
/// Préfixe des identifiants d'appareil.
pub const DEV: &str = "dev_";
/// Préfixe des identifiants de serveur.
pub const SRV: &str = "srv_";
/// Préfixe des identifiants de canal.
pub const CHN: &str = "chn_";
/// Préfixe des identifiants de message.
pub const MSG: &str = "msg_";
/// Préfixe des identifiants d'événement.
pub const EVT: &str = "evt_";
/// Préfixe des identifiants de session.
pub const SESS: &str = "sess_";
/// Préfixe des identifiants de clé.
pub const KEY: &str = "k_";

/// Marqueur de préfixe : associe un préfixe constant à un type.
///
/// Rust stable n'autorise pas `&str` comme paramètre const générique (seuls
/// entiers, `bool` et `char` le sont). Le préfixe est donc porté par une
/// constante associée à un type-marqueur (pattern typestate), ce qui
/// conserve le même niveau de sûreté : impossible de confondre deux types
/// d'identifiants à la compilation.
pub trait Prefix {
    /// La chaîne de préfixe (ex. `"usr_"`).
    const VALUE: &'static str;
}

/// Marqueur du préfixe `usr_` (identifiants d'utilisateur).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserPrefix;
impl Prefix for UserPrefix {
    const VALUE: &'static str = USR;
}

/// Marqueur du préfixe `dev_` (identifiants d'appareil).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DevicePrefix;
impl Prefix for DevicePrefix {
    const VALUE: &'static str = DEV;
}

/// Marqueur du préfixe `srv_` (identifiants de serveur).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServerPrefix;
impl Prefix for ServerPrefix {
    const VALUE: &'static str = SRV;
}

/// Marqueur du préfixe `chn_` (identifiants de canal).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChannelPrefix;
impl Prefix for ChannelPrefix {
    const VALUE: &'static str = CHN;
}

/// Marqueur du préfixe `msg_` (identifiants de message).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessagePrefix;
impl Prefix for MessagePrefix {
    const VALUE: &'static str = MSG;
}

/// Marqueur du préfixe `evt_` (identifiants d'événement).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventPrefix;
impl Prefix for EventPrefix {
    const VALUE: &'static str = EVT;
}

/// Marqueur du préfixe `sess_` (identifiants de session).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SessionPrefix;
impl Prefix for SessionPrefix {
    const VALUE: &'static str = SESS;
}

/// Marqueur du préfixe `k_` (identifiants de clé).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyPrefix;
impl Prefix for KeyPrefix {
    const VALUE: &'static str = KEY;
}

/// Identifiant d'utilisateur (`usr_` + ULID).
pub type UserId = Identifier<UserPrefix>;
/// Identifiant d'appareil (`dev_` + ULID).
pub type DeviceId = Identifier<DevicePrefix>;
/// Identifiant de serveur (`srv_` + ULID).
pub type ServerId = Identifier<ServerPrefix>;
/// Identifiant de canal (`chn_` + ULID).
pub type ChannelId = Identifier<ChannelPrefix>;
/// Identifiant de message (`msg_` + ULID).
pub type MessageId = Identifier<MessagePrefix>;
/// Identifiant d'événement (`evt_` + ULID).
pub type EventId = Identifier<EventPrefix>;
/// Identifiant de session (`sess_` + ULID).
pub type SessionId = Identifier<SessionPrefix>;
/// Identifiant de clé (`k_` + ULID).
pub type KeyId = Identifier<KeyPrefix>;

/// Alphabet Base32 Crockford, sans `I`, `L`, `O` et `U`.
const CROCKFORD: &[u8] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// Longueur d'un ULID encodé en Base32 Crockford.
const ULID_LEN: usize = 26;

/// Identifiant typé par un préfixe constant.
///
/// Le préfixe est porté par le type lui-même (via un type-marqueur
/// [`Prefix`]), ce qui rend impossible de confondre un identifiant
/// d'utilisateur avec un identifiant de serveur à la compilation. La
/// plupart des identifiants font 30 caractères (préfixe de 4 + ULID de 26) ;
/// le préfixe `k_` (2 caractères) est une exception assumée de la
/// spécification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier<P: Prefix>(Ulid, PhantomData<P>);

impl<P: Prefix> Identifier<P> {
    /// Génère un nouvel identifiant unique, triable chronologiquement.
    pub fn generate() -> Self {
        Self(Ulid::new(), PhantomData)
    }

    /// Construit un identifiant depuis ses 16 octets bruts.
    ///
    /// Utile pour dériver un identifiant d'une empreinte (ex. `usr_` dérivé
    /// de la clé publique Ed25519) ou pour reconstruire un identifiant
    /// stocké en binaire.
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(Ulid::from_bytes(bytes), PhantomData)
    }

    /// Retourne les 16 octets bruts de l'identifiant.
    pub fn as_bytes(&self) -> [u8; 16] {
        self.0.to_bytes()
    }

    /// Retourne l'ULID sous-jacent.
    pub fn as_ulid(&self) -> Ulid {
        self.0
    }

    /// Horodatage de création (millisecondes depuis l'époque Unix).
    pub fn timestamp_ms(&self) -> u64 {
        self.0.timestamp_ms()
    }
}

/// Erreur de validation d'un identifiant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentifierError {
    /// Le préfixe ne correspond pas à celui attendu par le type.
    WrongPrefix {
        /// Préfixe attendu.
        expected: &'static str,
        /// Préfixe trouvé dans la chaîne.
        found: String,
    },
    /// La partie ULID n'a pas la bonne longueur (26 caractères).
    InvalidLength(usize),
    /// Un caractère n'appartient pas à l'alphabet Crockford.
    InvalidCharacter(char),
    /// La partie ULID n'est pas un ULID valide.
    InvalidUlid(String),
}

impl fmt::Display for IdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongPrefix { expected, found } => write!(
                f,
                "préfixe invalide : attendu « {expected} », trouvé « {found} »"
            ),
            Self::InvalidLength(len) => write!(
                f,
                "longueur invalide : {len} caractères au lieu de {ULID_LEN}"
            ),
            Self::InvalidCharacter(c) => write!(
                f,
                "caractère invalide « {c} » : l'alphabet Crockford exclut I, L, O et U"
            ),
            Self::InvalidUlid(s) => write!(f, "ULID invalide : « {s} »"),
        }
    }
}

impl std::error::Error for IdentifierError {}

impl<P: Prefix> FromStr for Identifier<P> {
    type Err = IdentifierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ulid_part = s
            .strip_prefix(P::VALUE)
            .ok_or_else(|| IdentifierError::WrongPrefix {
                expected: P::VALUE,
                found: s
                    .chars()
                    .take(P::VALUE.len())
                    .collect::<String>(),
            })?;

        validate_ulid_part(ulid_part)?;

        let ulid =
            Ulid::from_string(ulid_part).map_err(|e| IdentifierError::InvalidUlid(e.to_string()))?;
        Ok(Self(ulid, PhantomData))
    }
}

impl<P: Prefix> fmt::Display for Identifier<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // L'ULID est toujours affiché en majuscules, conformément à la
        // spécification, quelle que soit la casse produite par la crate.
        write!(f, "{}{}", P::VALUE, self.0.to_string().to_uppercase())
    }
}

impl<P: Prefix> Serialize for Identifier<P> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, P: Prefix> Deserialize<'de> for Identifier<P> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(de::Error::custom)
    }
}

/// Valide la partie ULID d'un identifiant (26 caractères Crockford).
fn validate_ulid_part(s: &str) -> Result<(), IdentifierError> {
    if s.len() != ULID_LEN {
        return Err(IdentifierError::InvalidLength(s.len()));
    }
    for c in s.chars() {
        if !c.is_ascii() || !CROCKFORD.contains(&(c as u8)) {
            return Err(IdentifierError::InvalidCharacter(c));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ULID valide issu de la spécification (exemple de la crate `ulid`).
    const VALID_ULID: &str = "01ARZ3NDEKTSV4RRFFQ69G5FAV";

    #[test]
    fn parse_identifiant_valide() {
        let id: UserId = format!("usr_{VALID_ULID}").parse().unwrap();
        assert_eq!(id.to_string(), format!("usr_{VALID_ULID}"));
    }

    #[test]
    fn rejette_un_mauvais_prefixe() {
        let err = format!("srv_{VALID_ULID}").parse::<UserId>().unwrap_err();
        assert!(matches!(
            err,
            IdentifierError::WrongPrefix {
                expected: "usr_",
                ..
            }
        ));
    }

    #[test]
    fn rejette_les_minuscules() {
        let err = format!("usr_{}", VALID_ULID.to_lowercase())
            .parse::<UserId>()
            .unwrap_err();
        assert!(matches!(err, IdentifierError::InvalidCharacter(_)));
    }

    #[test]
    fn rejette_les_lettres_exclues() {
        // I, L, O et U sont exclus de l'alphabet Crockford.
        for c in ['I', 'L', 'O', 'U'] {
            let mut s = VALID_ULID.to_string();
            s.replace_range(0..1, &c.to_string());
            let err = format!("usr_{s}").parse::<UserId>().unwrap_err();
            assert!(
                matches!(err, IdentifierError::InvalidCharacter(_)),
                "« {c} » aurait dû être rejeté"
            );
        }
    }

    #[test]
    fn rejette_une_mauvaise_longueur() {
        let err = "usr_01ARZ3NDEKTSV4RRFFQ69G5FA".parse::<UserId>().unwrap_err();
        assert!(matches!(err, IdentifierError::InvalidLength(25)));
    }

    #[test]
    fn accepte_le_prefixe_court_k() {
        let id: KeyId = format!("k_{VALID_ULID}").parse().unwrap();
        assert_eq!(id.to_string(), format!("k_{VALID_ULID}"));
    }

    #[test]
    fn genere_des_identifiants_uniques() {
        let a = UserId::generate();
        let b = UserId::generate();
        assert_ne!(a, b);
    }

    #[test]
    fn roundtrip_octets() {
        let id = UserId::generate();
        let bytes = id.as_bytes();
        assert_eq!(UserId::from_bytes(bytes), id);
    }

    #[test]
    fn roundtrip_serde() {
        let id = UserId::generate();
        let json = serde_json::to_string(&id).unwrap();
        let back: UserId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, back);
        assert_eq!(json, format!("\"{id}\""));
    }

    #[test]
    fn tri_chronologique() {
        let ancien = UserId::from_bytes([0; 16]);
        let recent = UserId::from_bytes([1; 16]);
        assert!(ancien < recent);
    }
}