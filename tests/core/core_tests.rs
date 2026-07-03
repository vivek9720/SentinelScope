use sentinel_core::{parse_ipv4, parse_cidr, parse_port_range, internet_checksum};
#[test]
fn parses_ipv4_and_cidr() { let ip = parse_ipv4("192.0.2.10").unwrap(); let cidr = parse_cidr("192.0.2.0/24").unwrap(); assert!(cidr.contains(ip)); }
#[test]
fn rejects_bad_port_range() { assert!(parse_port_range("2000:1000").is_err()); }
#[test]
fn checksum_empty_is_ffff() { assert_eq!(internet_checksum(&[]), 0xffff); }
