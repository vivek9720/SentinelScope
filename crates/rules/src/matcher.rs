use crate::types::IdsRule;

use sentinel_packet::PacketMetadata;

use sentinel_core::parse_port_range;



#[derive(Debug, Clone, PartialEq, Eq)]

pub struct RuleMetadataMatch { pub sid: Option<u32>, pub message: Option<String>, pub reason: String }



pub fn match_rule_metadata(rule: &IdsRule, meta: &PacketMetadata) -> Option<RuleMetadataMatch> {

    if rule.protocol != "ip" && meta.protocol.as_deref() != Some(rule.protocol.as_str()) { return None; }

    if !port_expr_matches(&rule.source_port, meta.source_port) { return None; }

    if !port_expr_matches(&rule.destination_port, meta.destination_port) { return None; }

    let mut reason = Vec::new();

    if let Some(service) = meta.service_hint() { reason.push(format!("service {}", service)); }

    for content in rule.contents() { if meta.dns_queries.iter().any(|q| q.contains(content)) { reason.push(format!("dns query contains {}", content)); } }

    Some(RuleMetadataMatch { sid: rule.sid(), message: rule.msg().map(str::to_string), reason: if reason.is_empty() { "header match".to_string() } else { reason.join(", ") } })

}



fn port_expr_matches(expr: &str, observed: Option<u16>) -> bool {

    if expr.eq_ignore_ascii_case("any") { return true; }

    let Some(port) = observed else { return false; };

    if let Ok((start, end)) = parse_port_range(expr) { (start..=end).contains(&port) } else { false }

}
