use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

use crate::encrypt::Encrypted;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(u8)]
pub enum KeyData {
    Public {
        #[serde_as(as = "Bytes")]
        master_pk: [u8; 48],
    },
    Secret {
        #[serde_as(as = "Bytes")]
        master_pk: [u8; 48],
        entropy: bool,
        encrypted: Encrypted,
    },
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretKeyData(#[serde_as(as = "Bytes")] pub Vec<u8>);
