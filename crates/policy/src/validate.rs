use crate::ordering::analyze_ordering;
use crate::types::{PolicyAction, SecurityPolicy};
use sentinel_core::{parse_ip_or_cidr, parse_port_range, Diagnostic, ValidationReport};

pub fn validate_policy(policy: &SecurityPolicy) -> ValidationReport {
    let mut report = ValidationReport::new("security policy");
    for (idx, rule) in policy.rules.iter().enumerate() {
        if rule.action.is_none() { report.add(Diagnostic::medium(format!("rule {} has no action", idx))); }
        if matches!(&rule.action, Some(PolicyAction::Unknown(_))) { report.add(Diagnostic::medium(format!("rule {} has unknown action", idx))); }
        if let Some(source) = &rule.source { if source != "anywhere" && parse_ip_or_cidr(source).is_err() { report.add(Diagnostic::low(format!("rule {} has non-CIDR source", idx))); } }
        if let Some(destination) = &rule.destination { if destination != "anywhere" && parse_ip_or_cidr(destination).is_err() { report.add(Diagnostic::low(format!("rule {} has non-CIDR destination", idx))); } }
        if let Some(port) = &rule.source_port { if parse_port_range(port).is_err() { report.add(Diagnostic::medium(format!("rule {} has invalid source port", idx))); } }
        if let Some(port) = &rule.destination_port { if parse_port_range(port).is_err() { report.add(Diagnostic::medium(format!("rule {} has invalid destination port", idx))); } }
        if rule.protocol.is_none() && (rule.source_port.is_some() || rule.destination_port.is_some()) { report.add(Diagnostic::low(format!("rule {} uses ports without protocol", idx))); }
    }
    for finding in analyze_ordering(policy) { report.add(Diagnostic::medium(format!("{} at rule {}: {}", finding.kind, finding.index, finding.detail))); }
    report
}
