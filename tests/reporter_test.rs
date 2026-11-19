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
