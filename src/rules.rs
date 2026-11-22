use crate::types::{KeyType, StorageKeyDefinition};

pub struct KeyRulesEngine;

impl KeyRulesEngine {
    pub fn lint_tuple_discriminants(keys: &[StorageKeyDefinition]) -> Vec<String> {
        let mut warnings = Vec::new();
        for def in keys {
            if let KeyType::Tuple(ref items) = def.key {
                if items.is_empty() {
                    warnings.push(format!(
                        "Contract '{}' defines an empty tuple storage key",
                        def.source_contract
                    ));
                } else if !matches!(
                    items[0],
                    KeyType::Symbol(_) | KeyType::EnumVariant { .. } | KeyType::U32(_)
                ) {
                    warnings.push(format!(
                        "Contract '{}' tuple key lacks a discriminant prefix as first element: {:?}",
                        def.source_contract, items[0]
                    ));
                }
            }
        }
        warnings
    }

    pub fn lint_instance_storage_keys(keys: &[StorageKeyDefinition], max_bytes: usize) -> Vec<String> {
        let mut warnings = Vec::new();
        for def in keys {
            if def.domain == crate::types::StorageDomain::Instance && def.key.footprint_size() > max_bytes {
                warnings.push(format!(
                    "Contract '{}' instance key size ({} bytes) exceeds recommended max threshold of {} bytes",
                    def.source_contract,
                    def.key.footprint_size(),
                    max_bytes
                ));
            }
        }
        warnings
    }

    pub fn lint_temporary_storage_hazards(keys: &[StorageKeyDefinition]) -> Vec<String> {
        let mut warnings = Vec::new();
        for def in keys {
            if def.domain == crate::types::StorageDomain::Temporary {
                let name = def.key.to_string();
                if name.contains("Admin") || name.contains("Owner") || name.contains("Config") {
                    warnings.push(format!(
                        "Contract '{}' stores core protocol state '{}' in Temporary storage (eviction hazard)",
                        def.source_contract, name
                    ));
                }
            }
        }
        warnings
    }

    pub fn classify_ttl_alert(live_until_ledger: u32, current_ledger: u32, min_threshold: u32) -> TtlAlertLevel {
        if live_until_ledger <= current_ledger {
            return TtlAlertLevel::Critical;
        }
        let remaining = live_until_ledger - current_ledger;
        if remaining < min_threshold / 2 {
            TtlAlertLevel::Critical
        } else if remaining < min_threshold {
            TtlAlertLevel::Warning
        } else {
            TtlAlertLevel::Safe
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TtlAlertLevel {
    Safe,
    Warning,
    Critical,
}
