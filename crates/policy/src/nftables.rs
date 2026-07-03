use crate::types::{FirewallRule, PolicyAction};

use sentinel_core::{Diagnostic, ParseResult};



pub fn parse_nft_line(line: &str) -> ParseResult<Option<FirewallRule>> {

    let clean = line.trim();

    if clean.is_empty() || clean.starts_with('#') || clean.ends_with('{') || clean == "}" { return Ok(None); }

    let tokens: Vec<&str> = clean.split_whitespace().collect();

    if tokens.is_empty() { return Ok(None); }

    let mut rule = FirewallRule { raw: clean.to_string(), ..Default::default() };

    if tokens.first() == Some(&"add") && tokens.get(1) == Some(&"rule") { rule.family = tokens.get(2).map(|s| s.to_string()); rule.table = tokens.get(3).map(|s| s.to_string()); rule.chain = tokens.get(4).map(|s| s.to_string()).unwrap_or_default(); } else { rule.chain = "inline".to_string(); }

    let mut idx = 0usize;

    while idx < tokens.len() {

        match tokens[idx] {

            "ip" if tokens.get(idx + 1) == Some(&"saddr") => { rule.source = tokens.get(idx + 2).map(|s| s.to_string()); idx += 2; }

            "ip" if tokens.get(idx + 1) == Some(&"daddr") => { rule.destination = tokens.get(idx + 2).map(|s| s.to_string()); idx += 2; }

            "tcp" | "udp" => { rule.protocol = Some(tokens[idx].to_string()); if tokens.get(idx + 1) == Some(&"sport") { rule.source_port = tokens.get(idx + 2).map(|s| s.to_string()); idx += 2; } else if tokens.get(idx + 1) == Some(&"dport") { rule.destination_port = tokens.get(idx + 2).map(|s| s.to_string()); idx += 2; } }

            "ct" if tokens.get(idx + 1) == Some(&"state") => { if let Some(v) = tokens.get(idx + 2) { rule.state = v.trim_matches('{').trim_matches('}').split(',').map(|s| s.trim().to_ascii_uppercase()).collect(); } idx += 2; }

            "accept" | "drop" | "reject" | "log" | "return" => { rule.action = Some(PolicyAction::from_word(tokens[idx])); }

            _ => {}

        }

        idx += 1;

    }

    if rule.chain.is_empty() { return Err(Diagnostic::medium("nftables rule missing chain")); }

    Ok(Some(rule))

}
