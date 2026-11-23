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
fn test_intra_contract_storage_collision_detection() {
    // Two storage sites in the SAME contract and SAME domain serializing to identical bytes -> real collision
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
            source_contract: "ProtocolV1".to_string(),
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
fn test_cross_contract_identical_keys_do_not_collide() {
    // Soroban isolates storage by contract ID; identical keys in different contracts are NOT collisions
    let keys = vec![
        StorageKeyDefinition {
            domain: StorageDomain::Persistent,
            key: KeyType::EnumVariant {
                enum_name: "DataKey".to_string(),
                variant: "Admin".to_string(),
            },
            source_contract: "VaultA".to_string(),
        },
        StorageKeyDefinition {
            domain: StorageDomain::Persistent,
            key: KeyType::EnumVariant {
                enum_name: "DataKey".to_string(),
                variant: "Admin".to_string(),
            },
            source_contract: "VaultB".to_string(),
        },
    ];

    let report = CollisionDetector::analyze(&keys);
    assert!(
        report.is_clean(),
        "identical keys in distinct Soroban contracts must never be flagged as collisions"
    );
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

#[test]
fn test_oversized_key_detection() {
    let huge_bytes = vec![0xaa; 128]; // 128 bytes
    let keys = vec![StorageKeyDefinition {
        domain: StorageDomain::Persistent,
        key: KeyType::Bytes(huge_bytes),
        source_contract: "BlobStorageContract".to_string(),
    }];

    // Analyze with 64 byte budget
    let report = CollisionDetector::analyze_with_budget(&keys, 64);
    assert!(!report.is_clean());
    assert!(report.violations.iter().any(|v| matches!(
        v,
        KeyViolation::OversizedKey { size_bytes, max_recommended: 64, .. } if *size_bytes > 64
    )));
}

#[test]
fn test_nested_tuple_and_i128_keys() {
    let keys = vec![StorageKeyDefinition {
        domain: StorageDomain::Instance,
        key: KeyType::Tuple(vec![
            KeyType::EnumVariant {
                enum_name: "OraclePool".to_string(),
                variant: "TickData".to_string(),
            },
            KeyType::I128(-54200),
            KeyType::Bytes(vec![0x01, 0x02, 0x03]),
        ]),
        source_contract: "DexContract".to_string(),
    }];

    let report = CollisionDetector::analyze(&keys);
    assert!(report.is_clean());
    assert_eq!(report.total_keys_analyzed, 1);
    assert!(report.total_footprint_bytes > 0);
}

#[test]
fn test_cross_domain_isolation() {
    // Exact same key in Instance vs Persistent domains must NOT collide
    let keys = vec![
        StorageKeyDefinition {
            domain: StorageDomain::Instance,
            key: KeyType::EnumVariant {
                enum_name: "ConfigKey".to_string(),
                variant: "Admin".to_string(),
            },
            source_contract: "ContractA".to_string(),
        },
        StorageKeyDefinition {
            domain: StorageDomain::Persistent,
            key: KeyType::EnumVariant {
                enum_name: "ConfigKey".to_string(),
                variant: "Admin".to_string(),
            },
            source_contract: "ContractB".to_string(),
        },
    ];

    let report = CollisionDetector::analyze(&keys);
    assert!(report.is_clean(), "Different storage domains should not collide");
}

#[test]
fn test_key_type_display_formatting() {
    let symbol = KeyType::Symbol("Admin".into());
    assert_eq!(symbol.to_string(), "Symbol(\"Admin\")");

    let enum_v = KeyType::EnumVariant {
        enum_name: "Status".into(),
        variant: "Active".into(),
    };
    assert_eq!(enum_v.to_string(), "Status::Active");

    let tuple = KeyType::Tuple(vec![KeyType::U32(1), KeyType::I128(200)]);
    assert_eq!(tuple.to_string(), "(U32(1), I128(200))");
}

#[test]
fn test_cli_guide_exists_and_contains_sections() {
    let guide_path = std::path::Path::new("docs/CLI_GUIDE.md");
    assert!(guide_path.exists());
    let content = std::fs::read_to_string(guide_path).expect("read CLI_GUIDE.md");
    assert!(content.contains("# Soroban Keyscope CLI Guide"));
    assert!(content.contains("--min-ttl-ledgers"));
    assert!(content.contains("Storage Domains & Quota Guidance"));
    assert!(content.contains("TTL Alert Levels"));
}
