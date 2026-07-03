#![no_main]

fn run(data: &[u8]) {
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
