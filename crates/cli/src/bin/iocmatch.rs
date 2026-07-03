use std::env;

use std::fs;

use std::process;

use sentinel_ioc::{IocCollection, parse_ioc_bytes, match_packet_metadata};

use sentinel_packet::{parse_pcap, metadata_from_frame};



fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 { eprintln!("usage: iocmatch <ioc-file> <pcap-file>"); process::exit(2); }

    let ioc_data = fs::read(&args[1]).unwrap_or_else(|err| { eprintln!("{}: {}", args[1], err); process::exit(1); });

    let pcap_data = fs::read(&args[2]).unwrap_or_else(|err| { eprintln!("{}: {}", args[2], err); process::exit(1); });

    let parsed = parse_ioc_bytes(&ioc_data);

    for line in parsed.diagnostics.render_lines() { eprintln!("ioc: {}", line); }

    let mut collection = IocCollection::new();

    collection.extend(parsed.entries);

    let pcap = parse_pcap(&pcap_data).unwrap_or_else(|err| { eprintln!("pcap: {}", err); process::exit(1); });

    let mut total = 0usize;

    for (idx, record) in pcap.records.iter().enumerate() {

        let (meta, diag) = metadata_from_frame(idx, record.data);

        for line in diag.render_lines() { eprintln!("packet {}: {}", idx, line); }

        for hit in match_packet_metadata(&collection, &meta) {

            total += 1;

            println!("frame={} field={} value={} key={} disposition={:?} severity={}", idx, hit.field, hit.value, hit.key, hit.disposition, hit.severity.as_str());

        }

    }

    eprintln!("matches: {}", total);

}
