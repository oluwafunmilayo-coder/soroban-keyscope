use crate::types::{KeyType, StorageDomain, StorageKeyDefinition};
use serde::{Deserialize, Serialize};
use syn::{Attribute, Fields, Item};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractedKeyVariant {
    pub enum_name: String,
    pub variant_name: String,
    pub arity: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractedContractSchema {
    pub contract_name: String,
    pub contracttype_enums: Vec<ExtractedKeyVariant>,
    pub detected_domains: Vec<StorageDomain>,
}

/// Real Rust AST static analyzer for Soroban contract source files using `syn`.
pub struct RustAstExtractor;

impl RustAstExtractor {
    /// Parse a Soroban contract `.rs` source string and extract `#[contracttype]` enum key
    /// definitions along with detected storage domains (`persistent`, `temporary`, `instance`).
    pub fn extract_from_source(
        contract_name: &str,
        rust_source: &str,
    ) -> Result<ExtractedContractSchema, String> {
        let ast = syn::parse_file(rust_source).map_err(|e| format!("Rust syntax parse error: {e}"))?;
        let mut variants = Vec::new();

        for item in &ast.items {
            if let Item::Enum(item_enum) = item {
                if has_contracttype_attr(&item_enum.attrs) {
                    let enum_name = item_enum.ident.to_string();
                    for v in &item_enum.variants {
                        let arity = match &v.fields {
                            Fields::Unit => 0,
                            Fields::Unnamed(u) => u.unnamed.len(),
                            Fields::Named(n) => n.named.len(),
                        };
                        variants.push(ExtractedKeyVariant {
                            enum_name: enum_name.clone(),
                            variant_name: v.ident.to_string(),
                            arity,
                        });
                    }
                }
            }
        }

        let mut detected_domains = Vec::new();
        if rust_source.contains(".persistent()") {
            detected_domains.push(StorageDomain::Persistent);
        }
        if rust_source.contains(".temporary()") {
            detected_domains.push(StorageDomain::Temporary);
        }
        if rust_source.contains(".instance()") {
            detected_domains.push(StorageDomain::Instance);
        }

        Ok(ExtractedContractSchema {
            contract_name: contract_name.to_string(),
            contracttype_enums: variants,
            detected_domains,
        })
    }

    /// Convert extracted `#[contracttype]` unit variants into `StorageKeyDefinition` entries
    /// for a given storage domain so they can be checked for exact XDR `ScVal` collisions.
    pub fn to_key_definitions(
        schema: &ExtractedContractSchema,
        domain: StorageDomain,
    ) -> Vec<StorageKeyDefinition> {
        schema
            .contracttype_enums
            .iter()
            .filter(|v| v.arity == 0)
            .map(|v| StorageKeyDefinition {
                domain,
                key: KeyType::EnumVariant {
                    enum_name: v.enum_name.clone(),
                    variant: v.variant_name.clone(),
                },
                source_contract: schema.contract_name.clone(),
            })
            .collect()
    }
}

fn has_contracttype_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path()
            .segments
            .last()
            .map(|seg| seg.ident == "contracttype")
            .unwrap_or(false)
    })
}
