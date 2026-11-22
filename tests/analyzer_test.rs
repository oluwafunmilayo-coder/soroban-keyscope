use soroban_keyscope::{KeyType, StorageAnalyzer, StorageDomain, StorageKeyDefinition};

#[test]
fn test_domain_footprint_breakdown() {
    let keys = vec![
        StorageKeyDefinition {
            domain: StorageDomain::Instance,
            key: KeyType::Symbol("ADMIN".into()),
            source_contract: "Contract1".into(),
        },
        StorageKeyDefinition {
            domain: StorageDomain::Persistent,
            key: KeyType::Tuple(vec![KeyType::U32(1), KeyType::Address("G_USER".into())]),
            source_contract: "Contract1".into(),
        },
        StorageKeyDefinition {
            domain: StorageDomain::Temporary,
            key: KeyType::Bytes(vec![1, 2, 3, 4]),
            source_contract: "Contract2".into(),
        },
    ];

    let summary = StorageAnalyzer::summarize(&keys);
    assert_eq!(summary.total_keys, 3);
    assert!(summary.instance_bytes > 0);
    assert!(summary.persistent_bytes > 0);
    assert!(summary.temporary_bytes > 0);
    assert_eq!(
        summary.total_bytes,
        summary.instance_bytes + summary.persistent_bytes + summary.temporary_bytes
    );
    assert_eq!(summary.contract_distribution.get("Contract1"), Some(&2));
    assert_eq!(summary.contract_distribution.get("Contract2"), Some(&1));

    let p_inst = summary.domain_percentage(StorageDomain::Instance);
    let p_pers = summary.domain_percentage(StorageDomain::Persistent);
    let p_temp = summary.domain_percentage(StorageDomain::Temporary);
    let total_pct = p_inst + p_pers + p_temp;
    assert!((total_pct - 100.0).abs() < 1e-4);
}

#[test]
fn test_storage_rent_estimation() {
    // 500 bytes for 1000 ledgers, persistent: 1 KB * 10 * 1000 = 10,000 stroops
    let pers_rent = StorageAnalyzer::estimate_rent_stroops(500, 1000, true);
    assert_eq!(pers_rent, 10_000);

    // 500 bytes for 1000 ledgers, temporary: 1 KB * 3 * 1000 = 3,000 stroops
    let temp_rent = StorageAnalyzer::estimate_rent_stroops(500, 1000, false);
    assert_eq!(temp_rent, 3_000);

    // 2500 bytes for 100 ledgers, persistent: 3 KB * 10 * 100 = 3,000 stroops
    let multi_kb = StorageAnalyzer::estimate_rent_stroops(2500, 100, true);
    assert_eq!(multi_kb, 3_000);
}
