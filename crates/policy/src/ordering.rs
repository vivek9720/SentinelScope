use std::collections::{BTreeMap, BTreeSet};

use crate::types::{FirewallRule, PolicyAction, SecurityPolicy};



#[derive(Debug, Clone, PartialEq, Eq)]

pub struct OrderingFinding { pub index: usize, pub kind: String, pub detail: String }



pub fn analyze_ordering(policy: &SecurityPolicy) -> Vec<OrderingFinding> {

    let mut findings = Vec::new(); let mut seen = BTreeSet::new(); let mut terminal_by_chain: BTreeMap<String, usize> = BTreeMap::new();

    for (idx, rule) in policy.rules.iter().enumerate() {

        let key = rule.key();

        if !seen.insert(key) { findings.push(OrderingFinding { index: idx, kind: "duplicate".to_string(), detail: "duplicate firewall rule".to_string() }); }

        if let Some(first) = terminal_by_chain.get(&rule.chain) { findings.push(OrderingFinding { index: idx, kind: "shadowed".to_string(), detail: format!("rule follows broad terminal rule at {}", first) }); }

        if is_broad_terminal(rule) { terminal_by_chain.entry(rule.chain.clone()).or_insert(idx); }

    }

    findings

}



fn is_broad_terminal(rule: &FirewallRule) -> bool {

    matches!(&rule.action, Some(PolicyAction::Accept | PolicyAction::Drop | PolicyAction::Reject)) && rule.source.is_none() && rule.destination.is_none() && rule.source_port.is_none() && rule.destination_port.is_none() && rule.protocol.is_none()
}
