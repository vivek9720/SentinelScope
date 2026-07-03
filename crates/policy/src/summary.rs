use std::collections::BTreeMap;

use crate::types::{PolicyAction, SecurityPolicy};



#[derive(Debug, Clone, Default)]

pub struct PolicySummary { pub rules: usize, pub accepts: usize, pub drops: usize, pub rejects: usize, pub logs: usize, pub chains: BTreeMap<String, usize> }



pub fn summarize_policy(policy: &SecurityPolicy) -> PolicySummary {

    let mut summary = PolicySummary { rules: policy.rules.len(), ..Default::default() };

    for rule in &policy.rules {

        *summary.chains.entry(rule.chain.clone()).or_default() += 1;

        match &rule.action { Some(PolicyAction::Accept) => summary.accepts += 1, Some(PolicyAction::Drop) => summary.drops += 1, Some(PolicyAction::Reject) => summary.rejects += 1, Some(PolicyAction::Log) => summary.logs += 1, _ => {} }
    }

    summary

}



impl PolicySummary {

    pub fn render_text(&self) -> String {

        let mut lines = vec![format!("rules: {}", self.rules), format!("accept: {}", self.accepts), format!("drop: {}", self.drops), format!("reject: {}", self.rejects), format!("log: {}", self.logs)];

        for (chain, count) in &self.chains { lines.push(format!("chain {}: {}", chain, count)); }

        lines.join("\n")

    }

}
