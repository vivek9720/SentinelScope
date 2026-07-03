use std::env;

use std::fs;

use std::process;



fn read_file_arg(program: &str) -> Vec<u8> {

    let mut args = env::args().skip(1);

    let Some(path) = args.next() else { eprintln!("usage: {} <file>", program); process::exit(2); };

    match fs::read(&path) { Ok(bytes) => bytes, Err(err) => { eprintln!("{}: {}", path, err); process::exit(1); } }

}



fn main() {

    let data = read_file_arg("rulecheck");

    let parsed = sentinel_rules::parse_rules_bytes(&data);

    for line in parsed.diagnostics.render_lines() { eprintln!("parse: {}", line); }

    let report = sentinel_rules::validate_rules(&parsed.rules);

    println!("rules: {}", parsed.rules.len());

    println!("{}", report.summary());

    for rule in &parsed.rules { println!("{}", sentinel_rules::normalize_rule_text(rule)); }

    for line in report.diagnostics.render_lines() { eprintln!("{}", line); }

    if !report.ok() { std::process::exit(1); }

}
