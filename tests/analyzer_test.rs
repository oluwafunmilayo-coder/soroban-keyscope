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
