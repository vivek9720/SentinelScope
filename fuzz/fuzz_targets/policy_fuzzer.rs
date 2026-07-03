#![no_main]

fn run(data: &[u8]) {
    let parsed = sentinel_policy::parse_policy_bytes(data);
    let _ = sentinel_policy::summarize_policy(&parsed.policy);
    let _ = sentinel_policy::validate_policy(&parsed.policy);
    let _ = sentinel_policy::analyze_ordering(&parsed.policy);
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
