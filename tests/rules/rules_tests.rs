use sentinel_rules::{parse_rule_line, validate_rule};
#[test]
fn parses_basic_rule() { let rule = parse_rule_line("alert tcp any any -> any 80 (msg:\"test\"; content:\"Host\"; sid:1001; rev:1;)").unwrap(); assert!(validate_rule(&rule).diagnostics.len() == 0); }
