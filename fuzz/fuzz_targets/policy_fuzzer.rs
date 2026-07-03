#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let parsed = sentinel_policy::parse_policy_bytes(data);
    let _ = sentinel_policy::summarize_policy(&parsed.policy);
    let _ = sentinel_policy::validate_policy(&parsed.policy);
    let _ = sentinel_policy::analyze_ordering(&parsed.policy);
});
