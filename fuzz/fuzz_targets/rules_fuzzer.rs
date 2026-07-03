#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let parsed = sentinel_rules::parse_rules_bytes(data);
    let _ = sentinel_rules::validate_rules(&parsed.rules);
    for rule in parsed.rules.iter().take(32) {
        let _ = sentinel_rules::normalize_rule_text(rule);
    }
});
