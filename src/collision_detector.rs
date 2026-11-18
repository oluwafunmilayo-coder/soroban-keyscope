use crate::types::{KeyType, StorageDomain, StorageKeyDefinition};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyViolation {
    #[error("Storage key collision detected in {domain} domain between '{contract_a}' and '{contract_b}'")]
    DirectCollision {
        domain: StorageDomain,
        contract_a: String,
        contract_b: String,
        serialized_hex: String,
    },

    #[error("Un-namespaced raw symbol key detected in contract '{contract}': '{symbol}'. Recommend enum-scoped prefix")]
    UnscopedSymbol { contract: String, symbol: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnalysisReport {
    pub total_keys_analyzed: usize,
    pub domain_counts: HashMap<String, usize>,
    pub total_footprint_bytes: usize,
    pub violations: Vec<KeyViolation>,
}

impl AnalysisReport {
    pub fn is_clean(&self) -> bool {
        self.violations.is_empty()
    }
}

pub struct CollisionDetector;

impl CollisionDetector {
    pub fn analyze(keys: &[StorageKeyDefinition]) -> AnalysisReport {
        let mut report = AnalysisReport::default();
        let mut byte_map: HashMap<(StorageDomain, Vec<u8>), &StorageKeyDefinition> = HashMap::new();

        for key_def in keys {
            report.total_keys_analyzed += 1;
            let bytes = key_def.key.to_serialized_bytes();
            report.total_footprint_bytes += bytes.len();

            *report
                .domain_counts
                .entry(key_def.domain.to_string())
                .or_insert(0) += 1;

            // Check for raw unscoped symbols
            if let KeyType::Symbol(ref sym) = key_def.key {
                report.violations.push(KeyViolation::UnscopedSymbol {
                    contract: key_def.source_contract.clone(),
                    symbol: sym.clone(),
                });
            }

            // Check for collision within domain
            if let Some(existing) = byte_map.get(&(key_def.domain, bytes.clone())) {
                if existing.source_contract != key_def.source_contract {
                    report.violations.push(KeyViolation::DirectCollision {
                        domain: key_def.domain,
                        contract_a: existing.source_contract.clone(),
                        contract_b: key_def.source_contract.clone(),
                        serialized_hex: hex_string(&bytes),
                    });
                }
            } else {
                byte_map.insert((key_def.domain, bytes), key_def);
            }
        }

        report
    }
}

fn hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
