use soroban_keyscope::{RustAstExtractor, StorageDomain, XdrKeyEncoder};

#[test]
fn test_ast_extractor_parses_real_soroban_contract_and_catches_enum_xdr_collision() {
    let sample_contract = r#"
        use soroban_sdk::{contracttype, Address, Env};

        #[contracttype]
        pub enum VaultKey {
            Admin,
            Balance(Address),
        }

        #[contracttype]
        pub enum ModuleKey {
            Admin,
            Paused,
        }

        pub fn touch(env: Env) {
            let _ = env.storage().instance();
            let _ = env.storage().persistent();
        }
    "#;

    let schema = RustAstExtractor::extract_from_source("MultiModuleVault", sample_contract).unwrap();
    assert_eq!(schema.contracttype_enums.len(), 4);
    assert!(schema.detected_domains.contains(&StorageDomain::Instance));
    assert!(schema.detected_domains.contains(&StorageDomain::Persistent));

    let defs = RustAstExtractor::to_key_definitions(&schema, StorageDomain::Instance);
    let collisions = XdrKeyEncoder::detect_intra_contract_xdr_collisions(&defs).unwrap();
    assert_eq!(collisions.len(), 1);
    assert_eq!(collisions[0].colliding_descriptions.len(), 2);
}
