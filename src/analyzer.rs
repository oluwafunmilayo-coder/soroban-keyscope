use crate::types::{StorageDomain, StorageKeyDefinition};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FootprintSummary {
    pub total_keys: usize,
    pub total_bytes: usize,
    pub instance_bytes: usize,
    pub persistent_bytes: usize,
    pub temporary_bytes: usize,
    pub average_key_size_bytes: f64,
    pub largest_key_bytes: usize,
    pub contract_distribution: HashMap<String, usize>,
}

impl FootprintSummary {
    pub fn domain_percentage(&self, domain: StorageDomain) -> f64 {
        if self.total_bytes == 0 {
            return 0.0;
        }
        let bytes = match domain {
            StorageDomain::Instance => self.instance_bytes,
            StorageDomain::Persistent => self.persistent_bytes,
            StorageDomain::Temporary => self.temporary_bytes,
        };
        ((bytes as f64) / (self.total_bytes as f64)) * 100.0
    }
}

pub struct StorageAnalyzer;

impl StorageAnalyzer {
    pub fn summarize(keys: &[StorageKeyDefinition]) -> FootprintSummary {
        let mut summary = FootprintSummary::default();
        summary.total_keys = keys.len();

        for k in keys {
            let size = k.key.footprint_size();
            summary.total_bytes += size;
            if size > summary.largest_key_bytes {
                summary.largest_key_bytes = size;
            }
            match k.domain {
                StorageDomain::Instance => summary.instance_bytes += size,
                StorageDomain::Persistent => summary.persistent_bytes += size,
                StorageDomain::Temporary => summary.temporary_bytes += size,
            }
            *summary
                .contract_distribution
                .entry(k.source_contract.clone())
                .or_insert(0) += 1;
        }

        if summary.total_keys > 0 {
            summary.average_key_size_bytes =
                (summary.total_bytes as f64) / (summary.total_keys as f64);
        }

        summary
    }

    pub fn estimate_rent_stroops(bytes: usize, ledgers: u32, is_persistent: bool) -> u64 {
        let rate_per_kb_ledger = if is_persistent { 10u64 } else { 3u64 };
        let kb = ((bytes as u64) + 1023) / 1024;
        kb * rate_per_kb_ledger * (ledgers as u64)
    }
}
