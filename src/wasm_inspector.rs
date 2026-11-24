use serde::{Deserialize, Serialize};
use wasmparser::{Parser, Payload};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmCustomSectionSummary {
    pub has_contract_spec: bool,
    pub has_contract_meta: bool,
    pub spec_section_bytes: usize,
    pub custom_section_names: Vec<String>,
}

/// Real WebAssembly binary inspector extracting Soroban `contractspecv0` and `contractmetav0`
/// custom sections directly from compiled `.wasm` modules.
pub struct WasmInspector;

impl WasmInspector {
    pub fn inspect_wasm_bytes(wasm_bytes: &[u8]) -> Result<WasmCustomSectionSummary, String> {
        let mut summary = WasmCustomSectionSummary {
            has_contract_spec: false,
            has_contract_meta: false,
            spec_section_bytes: 0,
            custom_section_names: Vec::new(),
        };

        for payload in Parser::new(0).parse_all(wasm_bytes) {
            let payload = payload.map_err(|e| format!("invalid wasm binary: {e}"))?;
            if let Payload::CustomSection(reader) = payload {
                let name = reader.name().to_string();
                if name == "contractspecv0" {
                    summary.has_contract_spec = true;
                    summary.spec_section_bytes += reader.data().len();
                } else if name == "contractmetav0" {
                    summary.has_contract_meta = true;
                }
                summary.custom_section_names.push(name);
            }
        }
        Ok(summary)
    }
}
