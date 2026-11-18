use soroban_keyscope::{
    CollisionDetector, KeyType, KeyViolation, StorageDomain, StorageKeyDefinition,
};

#[test]
fn test_clean_namespaced_storage_keys() {
    let keys = vec![
        StorageKeyDefinition {
            domain: StorageDomain::Instance,
            key: KeyType::EnumVariant {
                enum_name: "AdminKey".to_string(),
                variant: "Owner".to_string(),
            },
            source_contract: "VaultContract".to_string(),
        },
        StorageKeyDefinition {
            domain: StorageDomain::Persistent,
            key: KeyType::Tuple(vec![
                KeyType::EnumVariant {
                    enum_name: "BalanceKey".to_string(),
                    variant: "User".to_string(),
                },
                KeyType::Address("G_USER_1".to_string()),
            ]),
            source_contract: "VaultContract".to_string(),
        },
    ];

    let report = CollisionDetector::analyze(&keys);
    assert!(report.is_clean());
    assert_eq!(report.total_keys_analyzed, 2);
    assert!(report.total_footprint_bytes > 0);
}

#[test]
fn test_storage_collision_detection() {
    // Both contracts use identical enum variant name and variant in Persistent domain
    let keys = vec![
        StorageKeyDefinition {
            domain: StorageDomain::Persistent,
            key: KeyType::EnumVariant {
                enum_name: "SharedKey".to_string(),
                variant: "Config".to_string(),
            },
            source_contract: "ProtocolV1".to_string(),
        },
        StorageKeyDefinition {
            domain: StorageDomain::Persistent,
            key: KeyType::EnumVariant {
                enum_name: "SharedKey".to_string(),
                variant: "Config".to_string(),
            },
            source_contract: "ExternalLibrary".to_string(),
        },
    ];

    let report = CollisionDetector::analyze(&keys);
    assert!(!report.is_clean());
    assert!(report
        .violations
        .iter()
        .any(|v| matches!(v, KeyViolation::DirectCollision { .. })));
}

#[test]
fn test_unscoped_symbol_flagging() {
    let keys = vec![StorageKeyDefinition {
        domain: StorageDomain::Temporary,
        key: KeyType::Symbol("NONCE".to_string()),
        source_contract: "AuthContract".to_string(),
    }];

    let report = CollisionDetector::analyze(&keys);
    assert!(!report.is_clean());
    assert!(report
        .violations
        .iter()
        .any(|v| matches!(v, KeyViolation::UnscopedSymbol { .. })));
}
