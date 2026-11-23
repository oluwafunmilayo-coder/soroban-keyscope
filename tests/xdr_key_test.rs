use soroban_keyscope::{KeyType, StorageDomain, StorageKeyDefinition, XdrKeyEncoder};

#[test]
fn test_real_xdr_scval_encoding_detects_same_variant_collision_across_enums_in_same_contract() {
    // In Soroban, two distinct #[contracttype] enums in the SAME contract with the same
    // unit variant name (`Admin`) serialize to the EXACT same XDR ScVal bytes!
    let keys = vec![
        StorageKeyDefinition {
            domain: StorageDomain::Instance,
            key: KeyType::EnumVariant {
                enum_name: "CoreKey".to_string(),
                variant: "Admin".to_string(),
            },
            source_contract: "LendingPool".to_string(),
        },
        StorageKeyDefinition {
            domain: StorageDomain::Instance,
            key: KeyType::EnumVariant {
                enum_name: "GovernanceKey".to_string(),
                variant: "Admin".to_string(),
            },
            source_contract: "LendingPool".to_string(),
        },
    ];

    let findings = XdrKeyEncoder::detect_intra_contract_xdr_collisions(&keys).unwrap();
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].contract, "LendingPool");
    assert!(findings[0].xdr_bytes_len > 0);
}

#[test]
fn test_real_xdr_scval_encoding_allows_same_variant_in_different_contracts() {
    let keys = vec![
        StorageKeyDefinition {
            domain: StorageDomain::Instance,
            key: KeyType::EnumVariant {
                enum_name: "DataKey".to_string(),
                variant: "Admin".to_string(),
            },
            source_contract: "ContractA".to_string(),
        },
        StorageKeyDefinition {
            domain: StorageDomain::Instance,
            key: KeyType::EnumVariant {
                enum_name: "DataKey".to_string(),
                variant: "Admin".to_string(),
            },
            source_contract: "ContractB".to_string(),
        },
    ];

    let findings = XdrKeyEncoder::detect_intra_contract_xdr_collisions(&keys).unwrap();
    assert!(findings.is_empty());
}
