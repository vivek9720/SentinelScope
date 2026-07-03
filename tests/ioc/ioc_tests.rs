use sentinel_ioc::{parse_ioc_text, IocCollection};
#[test]
fn parses_mixed_iocs() { let out = parse_ioc_text("192.0.2.4 block severity=high\nexample.com allow\n"); let mut c = IocCollection::new(); assert_eq!(c.extend(out.entries), 2); }
