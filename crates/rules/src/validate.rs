use sentinel_core::{parse_port_range, Diagnostic, ValidationReport};

use crate::types::IdsRule;



pub fn validate_rule(rule: &IdsRule) -> ValidationReport {

    let mut report = ValidationReport::new("ids rule");

    if !matches!(rule.action.as_str(), "alert" | "pass" | "drop" | "reject" | "log") { report.add(Diagnostic::medium("unsupported IDS action")); }

    if !matches!(rule.protocol.as_str(), "tcp" | "udp" | "icmp" | "ip" | "http" | "dns") { report.add(Diagnostic::medium("unsupported IDS protocol")); }

    if !matches!(rule.direction.as_str(), "->" | "<>" | "<-" ) { report.add(Diagnostic::medium("unsupported IDS direction")); }

    if parse_port_range(&rule.source_port).is_err() && rule.source_port != "any" { report.add(Diagnostic::medium("invalid source port expression")); }

    if parse_port_range(&rule.destination_port).is_err() && rule.destination_port != "any" { report.add(Diagnostic::medium("invalid destination port expression")); }

    if rule.sid().is_none() { report.add(Diagnostic::medium("IDS rule missing numeric sid")); }

    if rule.rev().is_none() { report.add(Diagnostic::low("IDS rule missing numeric rev")); }

    if rule.msg().is_none() { report.add(Diagnostic::low("IDS rule missing msg")); }

    let content_total: usize = rule.contents().iter().map(|c| c.len()).sum();

    if content_total > 4096 { report.add(Diagnostic::medium("IDS rule content options are unusually large")); }

    report

}



pub fn validate_rules(rules: &[IdsRule]) -> ValidationReport {

    let mut report = ValidationReport::new("ids rules");

    let mut seen = std::collections::BTreeSet::new();

    for rule in rules {

        if let Some(sid) = rule.sid() { if !seen.insert(sid) { report.add(Diagnostic::medium(format!("duplicate sid {}", sid))); } }

        report.merge(validate_rule(rule));

    }

    report

}
