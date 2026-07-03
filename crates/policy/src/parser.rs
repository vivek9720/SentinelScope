use crate::iptables::parse_iptables_line;
use crate::nftables::parse_nft_line;
use crate::types::SecurityPolicy;
use sentinel_core::{split_csv_line, strip_comment, DiagnosticSet};

#[derive(Debug, Clone, Default)]
pub struct PolicyParseOutput { pub policy: SecurityPolicy, pub diagnostics: DiagnosticSet }

pub fn parse_policy_text(text: &str) -> PolicyParseOutput {
    let mut out = PolicyParseOutput::default();
    for (idx, line) in text.lines().enumerate() {
        let clean = strip_comment(line).trim();
        if clean.is_empty() { continue; }
        if clean.starts_with('[') || clean.contains('=') && !clean.starts_with('-') { parse_setting(clean, &mut out.policy); continue; }
        let parsed = if clean.starts_with("add rule") || clean.contains(" dport ") || clean.contains(" saddr ") { parse_nft_line(clean) } else { parse_iptables_line(clean) };
        match parsed { Ok(Some(rule)) => out.policy.rules.push(rule), Ok(None) => {}, Err(err) => out.diagnostics.push(err.at(idx + 1)) }
    }
    out
}
pub fn parse_policy_bytes(data: &[u8]) -> PolicyParseOutput { parse_policy_text(&String::from_utf8_lossy(data)) }
fn parse_setting(line: &str, policy: &mut SecurityPolicy) { if let Some((k, v)) = line.split_once('=') { policy.settings.push((k.trim().to_string(), v.trim().to_string())); } else if line.contains(',') { let fields = split_csv_line(line); if fields.len() >= 2 { policy.settings.push((fields[0].clone(), fields[1].clone())); } } }
