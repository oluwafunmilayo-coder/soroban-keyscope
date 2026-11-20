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
    Bytes(Vec<u8>),
    I128(i128),
    U32(u32),
}

impl std::fmt::Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Symbol(s) => write!(f, "Symbol(\"{}\")", s),
            Self::EnumVariant { enum_name, variant } => write!(f, "{}::{}", enum_name, variant),
            Self::Tuple(items) => {
                let inner = items.iter().map(|k| k.to_string()).collect::<Vec<_>>().join(", ");
                write!(f, "({})", inner)
            }
            Self::Address(a) => write!(f, "Address({})", a),
            Self::Bytes(b) => write!(f, "Bytes(len={})", b.len()),
            Self::I128(n) => write!(f, "I128({})", n),
            Self::U32(n) => write!(f, "U32({})", n),
        }
    }
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
            Self::Bytes(bytes) => {
                let mut b = vec![0x05, bytes.len() as u8];
                b.extend_from_slice(bytes);
                b
            }
            Self::I128(val) => {
                let mut b = vec![0x06];
                b.extend_from_slice(&val.to_be_bytes());
                b
            }
            Self::U32(n) => {
                let mut b = vec![0x07];
                b.extend_from_slice(&n.to_be_bytes());
                b
            }
        }
    }

    pub fn footprint_size(&self) -> usize {
        self.to_serialized_bytes().len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageKeyDefinition {
    pub domain: StorageDomain,
    pub key: KeyType,
    pub source_contract: String,
}
