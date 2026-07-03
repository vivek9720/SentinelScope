use crate::types::{IdsRule, RuleOption};

pub fn normalize_rule(rule: &IdsRule) -> IdsRule {
    let mut options = rule.options.clone();
    options.sort_by(|a, b| a.key.cmp(&b.key).then_with(|| a.value.cmp(&b.value)));
    IdsRule { action: rule.action.to_ascii_lowercase(), protocol: rule.protocol.to_ascii_lowercase(), source: normalize_addr(&rule.source), source_port: normalize_port(&rule.source_port), direction: rule.direction.clone(), destination: normalize_addr(&rule.destination), destination_port: normalize_port(&rule.destination_port), options }
}

pub fn normalize_rule_text(rule: &IdsRule) -> String {
    let norm = normalize_rule(rule);
    let mut opts = String::new();
    for RuleOption { key, value } in &norm.options {
        if let Some(value) = value { opts.push_str(&format!("{}:{}; ", key, quote_if_needed(value))); } else { opts.push_str(&format!("{}; ", key)); }
    }
    format!("{} {} {} {} {} {} {} ({})", norm.action, norm.protocol, norm.source, norm.source_port, norm.direction, norm.destination, norm.destination_port, opts.trim())
}

fn quote_if_needed(value: &str) -> String { if value.chars().any(|c| c.is_ascii_whitespace() || c == ';') { format!("\"{}\"", value.replace('"', "\\\"")) } else { value.to_string() } }
fn normalize_addr(value: &str) -> String { if value.eq_ignore_ascii_case("any") { "any".to_string() } else { value.to_ascii_lowercase() } }
fn normalize_port(value: &str) -> String { if value.eq_ignore_ascii_case("any") { "any".to_string() } else { value.to_string() } }
