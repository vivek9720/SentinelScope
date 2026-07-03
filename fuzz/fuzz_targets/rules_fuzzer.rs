#![no_main]

fn run(data: &[u8]) {
    let parsed = sentinel_rules::parse_rules_bytes(data);
    let _ = sentinel_rules::validate_rules(&parsed.rules);
    for rule in parsed.rules.iter().take(32) {
        let _ = sentinel_rules::normalize_rule_text(rule);
    }
}

#[no_mangle]
pub extern "C" fn LLVMFuzzerTestOneInput(data: *const u8, size: usize) -> i32 {
    if data.is_null() {
        return 0;
    }
    // libFuzzer passes a raw pointer/length pair through its C ABI.
    let bytes = unsafe { std::slice::from_raw_parts(data, size) };
    run(bytes);
    0
}
