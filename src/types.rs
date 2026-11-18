use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageDomain {
    Instance,
    Persistent,
    Temporary,
}

impl std::fmt::Display for StorageDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Instance => write!(f, "Instance"),
            Self::Persistent => write!(f, "Persistent"),
            Self::Temporary => write!(f, "Temporary"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyType {
    Symbol(String),
    EnumVariant { enum_name: String, variant: String },
    Tuple(Vec<KeyType>),
    Address(String),
    U32(u32),
}

impl KeyType {
    pub fn to_serialized_bytes(&self) -> Vec<u8> {
        match self {
            Self::Symbol(s) => {
                let mut b = vec![0x01];
                b.extend_from_slice(s.as_bytes());
                b
            }
            Self::EnumVariant { enum_name, variant } => {
                let mut b = vec![0x02];
                b.extend_from_slice(enum_name.as_bytes());
                b.push(0x00);
                b.extend_from_slice(variant.as_bytes());
                b
            }
            Self::Tuple(items) => {
                let mut b = vec![0x03, items.len() as u8];
                for item in items {
                    b.extend(item.to_serialized_bytes());
                }
                b
            }
            Self::Address(a) => {
                let mut b = vec![0x04];
                b.extend_from_slice(a.as_bytes());
                b
            }
            Self::U32(n) => {
                let mut b = vec![0x05];
                b.extend_from_slice(&n.to_be_bytes());
                b
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageKeyDefinition {
    pub domain: StorageDomain,
    pub key: KeyType,
    pub source_contract: String,
}
