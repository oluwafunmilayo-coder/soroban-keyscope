use soroban_keyscope::{KeyRulesEngine, KeyType, StorageDomain, StorageKeyDefinition};

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
