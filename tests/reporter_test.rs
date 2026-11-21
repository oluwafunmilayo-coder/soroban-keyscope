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

#[test]
fn test_mock_storage_dump_fixture_deserialization() {
    let fixture_path = std::path::Path::new("examples/fixtures/mock_storage_dump.json");
    assert!(fixture_path.exists());
    let raw = std::fs::read_to_string(fixture_path).expect("read mock storage dump");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("parse json");
    let arr = json.as_array().expect("dump is array");
    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0]["domain"], "Instance");
    assert_eq!(arr[1]["domain"], "Persistent");
    assert_eq!(arr[2]["domain"], "Temporary");
}

#[test]
fn test_html_layout_visualization() {
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

    let summary = StorageAnalyzer::summarize(&keys);
    let html = LayoutReporter::format_html(&keys, &summary);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Soroban Storage Footprint"));
    assert!(html.contains("class=\"instance\""));
    assert!(html.contains("class=\"persistent\""));
    assert!(html.contains("[Instance] Pool"));
}
