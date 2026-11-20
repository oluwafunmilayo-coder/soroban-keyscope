use soroban_keyscope::{
    KeyType, LayoutReporter, StorageAnalyzer, StorageDomain, StorageKeyDefinition,
};

#[test]
fn test_ascii_layout_visualization() {
    let keys = vec![StorageKeyDefinition {
        domain: StorageDomain::Instance,
        key: KeyType::EnumVariant {
            enum_name: "AdminKey".into(),
            variant: "Owner".into(),
        },
        source_contract: "Vault".into(),
    }];

    let summary = StorageAnalyzer::summarize(&keys);
    let ascii = LayoutReporter::format_ascii(&keys, &summary);
    assert!(ascii.contains("Soroban Storage Key Memory Layout"));
    assert!(ascii.contains("[Instance] Vault"));
    assert!(ascii.contains("End of Report"));
}

#[test]
fn test_csv_layout_export() {
    let keys = vec![
        StorageKeyDefinition {
            domain: StorageDomain::Instance,
            key: KeyType::Symbol("Config".into()),
            source_contract: "Pool".into(),
        },
        StorageKeyDefinition {
            domain: StorageDomain::Persistent,
            key: KeyType::U32(100),
            source_contract: "Token".into(),
        },
    ];

    let csv = LayoutReporter::format_csv_footprint(&keys);
    assert!(csv.starts_with("index,contract,domain,key_size_bytes\n"));
    assert!(csv.contains("1,Pool,Instance,"));
    assert!(csv.contains("2,Token,Persistent,"));
}
