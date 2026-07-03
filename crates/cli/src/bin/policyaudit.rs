use std::env;

use std::fs;

use std::process;



fn read_file_arg(program: &str) -> Vec<u8> {

    let mut args = env::args().skip(1);

    let Some(path) = args.next() else { eprintln!("usage: {} <file>", program); process::exit(2); };

    match fs::read(&path) { Ok(bytes) => bytes, Err(err) => { eprintln!("{}: {}", path, err); process::exit(1); } }

}



fn main() {

    let data = read_file_arg("policyaudit");

    let parsed = sentinel_policy::parse_policy_bytes(&data);

    for line in parsed.diagnostics.render_lines() { eprintln!("parse: {}", line); }

    let summary = sentinel_policy::summarize_policy(&parsed.policy);

    println!("{}", summary.render_text());

    let report = sentinel_policy::validate_policy(&parsed.policy);

    println!("{}", report.summary());

    for line in report.diagnostics.render_lines() { eprintln!("{}", line); }

    if !report.ok() { std::process::exit(1); }

}
