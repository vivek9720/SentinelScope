use crate::types::{FirewallRule, PolicyAction};

use sentinel_core::{Diagnostic, ParseResult};



pub fn parse_iptables_line(line: &str) -> ParseResult<Option<FirewallRule>> {

    let clean = line.trim();

    if clean.is_empty() || clean.starts_with('#') { return Ok(None); }

    if clean.starts_with(':') || clean.starts_with('*') || clean == "COMMIT" { return Ok(None); }

    let tokens = shell_words(clean);

    if tokens.is_empty() { return Ok(None); }

    let mut rule = FirewallRule { raw: clean.to_string(), ..Default::default() };

    let mut idx = 0usize;

    while idx < tokens.len() {

        match tokens[idx].as_str() {

            "-A" | "--append" | "-I" | "--insert" => { idx += 1; rule.chain = tokens.get(idx).cloned().unwrap_or_default(); }

            "-p" | "--protocol" => { idx += 1; rule.protocol = tokens.get(idx).map(|s| s.to_ascii_lowercase()); }

            "-s" | "--source" => { idx += 1; rule.source = tokens.get(idx).cloned(); }

            "-d" | "--destination" => { idx += 1; rule.destination = tokens.get(idx).cloned(); }

            "--sport" | "--source-port" => { idx += 1; rule.source_port = tokens.get(idx).cloned(); }

            "--dport" | "--destination-port" => { idx += 1; rule.destination_port = tokens.get(idx).cloned(); }

            "-i" | "--in-interface" => { idx += 1; rule.in_interface = tokens.get(idx).cloned(); }

            "-o" | "--out-interface" => { idx += 1; rule.out_interface = tokens.get(idx).cloned(); }

            "--state" | "--ctstate" => { idx += 1; if let Some(v) = tokens.get(idx) { rule.state = v.split(',').map(|s| s.to_ascii_uppercase()).collect(); } }

            "-j" | "--jump" => { idx += 1; if let Some(v) = tokens.get(idx) { rule.action = Some(PolicyAction::from_word(v)); } }

            _ => {}

        }

        idx += 1;

    }

    if rule.chain.is_empty() { return Err(Diagnostic::medium("iptables rule missing chain")); }

    Ok(Some(rule))

}



fn shell_words(line: &str) -> Vec<String> {

    let mut out = Vec::new(); let mut current = String::new(); let mut quote = None;

    for ch in line.chars() {

        match (quote, ch) { (Some(q), c) if c == q => quote = None, (Some(_), c) => current.push(c), (None, '\'' | '"') => quote = Some(ch), (None, c) if c.is_ascii_whitespace() => { if !current.is_empty() { out.push(std::mem::take(&mut current)); } }, (None, c) => current.push(c) }

    }

    if !current.is_empty() { out.push(current); }

    out

}
