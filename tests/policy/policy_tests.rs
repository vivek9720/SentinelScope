use sentinel_policy::{parse_policy_text, summarize_policy};
#[test]
fn parses_iptables_rule() { let out = parse_policy_text("-A INPUT -p tcp --dport 22 -j ACCEPT\n-A INPUT -j DROP\n"); let summary = summarize_policy(&out.policy); assert_eq!(summary.rules, 2); }
