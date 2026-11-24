use soroban_keyscope::WasmInspector;

#[test]
fn test_wasm_inspector_extracts_soroban_contractspecv0_custom_section() {
    // Minimal valid WASM binary header (\0asm v1) + custom section named "contractspecv0"
    let mut wasm = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
    let name = b"contractspecv0";
    let payload = b"\x00\x00\x00\x01";
    let section_len = 1 + name.len() + payload.len();
    wasm.push(0x00); // Custom section ID
    wasm.push(section_len as u8);
    wasm.push(name.len() as u8);
    wasm.extend_from_slice(name);
    wasm.extend_from_slice(payload);

    let summary = WasmInspector::inspect_wasm_bytes(&wasm).unwrap();
    assert!(summary.has_contract_spec);
    assert!(!summary.has_contract_meta);
    assert_eq!(summary.spec_section_bytes, 4);
    assert_eq!(summary.custom_section_names, vec!["contractspecv0"]);

    // Corrupted bytes must fail cleanly
    assert!(WasmInspector::inspect_wasm_bytes(b"not-wasm").is_err());
}
