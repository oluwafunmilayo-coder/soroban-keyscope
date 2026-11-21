use soroban_keyscope::{KeyRulesEngine, KeyType, StorageDomain, StorageKeyDefinition, TtlAlertLevel};

#[test]
fn test_lint_tuple_discriminants() {
    let bad_tuple = vec![
        StorageKeyDefinition {
            domain: StorageDomain::Persistent,
            key: KeyType::Tuple(vec![]), // empty tuple
            source_contract: "ContractBad1".into(),
        },
        StorageKeyDefinition {
            domain: StorageDomain::Persistent,
            // First item is an Address instead of a discriminant symbol/enum/u32
            key: KeyType::Tuple(vec![KeyType::Address("G_USER".into()), KeyType::U32(42)]),
            source_contract: "ContractBad2".into(),
        },
        StorageKeyDefinition {
            domain: StorageDomain::Persistent,
            // Correct tuple with enum prefix
            key: KeyType::Tuple(vec![
                KeyType::EnumVariant {
                    enum_name: "Key".into(),
                    variant: "Balance".into(),
                },
                KeyType::Address("G_USER".into()),
            ]),
            source_contract: "ContractGood".into(),
        },
    ];

    let warnings = KeyRulesEngine::lint_tuple_discriminants(&bad_tuple);
    assert_eq!(warnings.len(), 2);
    assert!(warnings[0].contains("empty tuple"));
    assert!(warnings[1].contains("lacks a discriminant"));
}

#[test]
fn test_lint_instance_storage_key_size() {
    let normal_key = StorageKeyDefinition {
        domain: StorageDomain::Instance,
        key: KeyType::Symbol("Admin".into()),
        source_contract: "Vault".into(),
    };
    let oversized_key = StorageKeyDefinition {
        domain: StorageDomain::Instance,
        key: KeyType::Bytes(vec![0u8; 100]),
        source_contract: "Vault".into(),
    };

    let warnings = KeyRulesEngine::lint_instance_storage_keys(&[normal_key, oversized_key], 64);
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("exceeds recommended max threshold of 64 bytes"));
}

#[test]
fn test_ttl_alert_level_classifier() {
    let min_threshold = 10_000u32;

    // Remaining 15,000 >= 10,000 => Safe
    assert_eq!(
        KeyRulesEngine::classify_ttl_alert(115_000, 100_000, min_threshold),
        TtlAlertLevel::Safe
    );

    // Remaining 7,000 < 10,000 and >= 5,000 => Warning
    assert_eq!(
        KeyRulesEngine::classify_ttl_alert(107_000, 100_000, min_threshold),
        TtlAlertLevel::Warning
    );

    // Remaining 3,000 < 5,000 => Critical
    assert_eq!(
        KeyRulesEngine::classify_ttl_alert(103_000, 100_000, min_threshold),
        TtlAlertLevel::Critical
    );

    // Expired or equal => Critical
    assert_eq!(
        KeyRulesEngine::classify_ttl_alert(100_000, 100_000, min_threshold),
        TtlAlertLevel::Critical
    );
    assert_eq!(
        KeyRulesEngine::classify_ttl_alert(99_000, 100_000, min_threshold),
        TtlAlertLevel::Critical
    );
}
