#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(pcap) = sentinel_packet::parse_pcap(data) {
        for (idx, record) in pcap.records.iter().take(32).enumerate() {
            let _ = sentinel_packet::metadata_from_frame(idx, record.data);
        }
        let _ = sentinel_packet::summarize_pcap(data);
    } else {
        let _ = sentinel_packet::parse_ethernet_frame(data);
        let _ = sentinel_packet::parse_ipv4_packet(data);
        let _ = sentinel_packet::parse_dns_message(data);
    }
});
