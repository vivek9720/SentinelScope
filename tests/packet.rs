use sentinel_packet::parse_pcap;

#[test]
fn rejects_short_pcap() {
    assert!(parse_pcap(&[0, 1, 2]).is_err());
}
