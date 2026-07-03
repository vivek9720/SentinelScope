use std::env;

use std::fs;

use std::process;



fn read_file_arg(program: &str) -> Vec<u8> {

    let mut args = env::args().skip(1);

    let Some(path) = args.next() else { eprintln!("usage: {} <file>", program); process::exit(2); };

    match fs::read(&path) { Ok(bytes) => bytes, Err(err) => { eprintln!("{}: {}", path, err); process::exit(1); } }

}



fn main() {

    let data = read_file_arg("packetscan");

    match sentinel_packet::summarize_pcap(&data) {

        Ok(summary) => println!("{}", summary.render_text()),

        Err(report) => { eprintln!("{}", report.summary()); for line in report.diagnostics.render_lines() { eprintln!("{}", line); } std::process::exit(1); }

    }

}
