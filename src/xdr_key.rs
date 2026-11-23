use crate::types::{KeyType, StorageDomain, StorageKeyDefinition};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use stellar_xdr::{
    BytesM, Int128Parts, Limits, ScBytes, ScString, ScSymbol, ScVal, ScVec, StringM, WriteXdr,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XdrCollisionFinding {
    pub contract: String,
    pub domain: StorageDomain,
    pub xdr_hex: String,
    pub xdr_bytes_len: usize,
    pub colliding_descriptions: Vec<String>,
}

/// Real Soroban XDR `ScVal` encoder for storage key collision & footprint verification.
pub struct XdrKeyEncoder;

impl XdrKeyEncoder {
    /// Convert a `KeyType` into its canonical Stellar `ScVal` representation.
    pub fn to_scval(key: &KeyType) -> Result<ScVal, String> {
        match key {
            KeyType::Symbol(s) => {
                let sym = StringM::try_from(s.as_str()).map_err(|e| format!("invalid symbol: {e}"))?;
                Ok(ScVal::Symbol(ScSymbol(sym)))
            }
            KeyType::U32(n) => Ok(ScVal::U32(*n)),
            KeyType::I128(n) => Ok(ScVal::I128(Int128Parts {
                hi: (*n >> 64) as i64,
                lo: *n as u64,
            })),
            KeyType::Bytes(b) => {
                let bytes_m = BytesM::try_from(b.as_slice())
                    .map_err(|e| format!("invalid bytes: {e}"))?;
                Ok(ScVal::Bytes(ScBytes(bytes_m)))
            }
            KeyType::Address(addr) => {
                let s = StringM::try_from(addr.as_str())
                    .map_err(|e| format!("invalid address string: {e}"))?;
                Ok(ScVal::String(ScString(s)))
            }
            KeyType::EnumVariant { variant, .. } => {
                // In Soroban `#[contracttype]` enums, unit variants encode as `ScVec([ScSymbol(variant)])`.
                let sym = StringM::try_from(variant.as_str())
                    .map_err(|e| format!("invalid enum variant symbol: {e}"))?;
                let vec = ScVec::try_from(vec![ScVal::Symbol(ScSymbol(sym))])
                    .map_err(|e| format!("scvec error: {e}"))?;
                Ok(ScVal::Vec(Some(vec)))
            }
            KeyType::Tuple(items) => {
                let mut sc_items = Vec::with_capacity(items.len());
                for item in items {
                    sc_items.push(Self::to_scval(item)?);
                }
                let vec = ScVec::try_from(sc_items).map_err(|e| format!("scvec error: {e}"))?;
                Ok(ScVal::Vec(Some(vec)))
            }
        }
    }

    /// Encode a `KeyType` into canonical Soroban XDR bytes.
    pub fn to_xdr_bytes(key: &KeyType) -> Result<Vec<u8>, String> {
        let scval = Self::to_scval(key)?;
        scval
            .to_xdr(Limits::none())
            .map_err(|e| format!("xdr encode error: {e}"))
    }

    /// Detect exact XDR `ScVal` byte collisions strictly within the same contract and storage domain.
    /// Notably, two distinct Rust `#[contracttype]` enums in the same contract that both define
    /// a unit variant named `Config` encode to the EXACT same Soroban XDR `ScVec([ScSymbol("Config")])`!
    pub fn detect_intra_contract_xdr_collisions(
        keys: &[StorageKeyDefinition],
    ) -> Result<Vec<XdrCollisionFinding>, String> {
        let mut grouped: HashMap<(String, StorageDomain, Vec<u8>), Vec<String>> = HashMap::new();

        for def in keys {
            let xdr_bytes = Self::to_xdr_bytes(&def.key)?;
            let desc = format!("{}", def.key);
            grouped
                .entry((def.source_contract.clone(), def.domain, xdr_bytes))
                .or_default()
                .push(desc);
        }

        let mut findings = Vec::new();
        for ((contract, domain, bytes), descs) in grouped {
            if descs.len() > 1 {
                let xdr_hex: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
                findings.push(XdrCollisionFinding {
                    contract,
                    domain,
                    xdr_hex,
                    xdr_bytes_len: bytes.len(),
                    colliding_descriptions: descs,
                });
            }
        }
        Ok(findings)
    }
}
