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
}
