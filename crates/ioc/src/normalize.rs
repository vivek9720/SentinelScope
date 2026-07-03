use sentinel_core::{looks_like_hex, normalize_domain, parse_cidr, parse_ipv4, Diagnostic, ParseResult};

use crate::types::{IocEntry, IocKind, ListDisposition};

use sentinel_core::Severity;



pub fn normalize_ioc_value(raw: &str) -> ParseResult<IocKind> {

    let value = raw.trim().trim_matches('"').trim_matches('\'');

    if value.is_empty() { return Err(Diagnostic::medium("empty IOC value")); }

    if value.contains('/') && value.split('/').count() == 2 && value.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '/') { return Ok(IocKind::Cidr(parse_cidr(value)?)); }

    if let Ok(ip) = parse_ipv4(value) { return Ok(IocKind::Ip(ip)); }

    if looks_like_hex(value, &[32, 40, 64, 96, 128]) { return Ok(IocKind::Hash(value.to_ascii_lowercase())); }

    if value.starts_with("http://") || value.starts_with("https://") { return normalize_url(value).map(IocKind::Url); }

    if let Some(domain) = normalize_domain(value) { return Ok(IocKind::Domain(domain)); }

    Err(Diagnostic::medium("unrecognized IOC value"))

}



pub fn normalize_url(raw: &str) -> ParseResult<String> {

    let lower = raw.trim().to_ascii_lowercase();

    if !(lower.starts_with("http://") || lower.starts_with("https://")) { return Err(Diagnostic::medium("URL IOC must use http or https")); }

    let no_fragment = lower.split('#').next().unwrap_or(&lower);

    let collapsed = no_fragment.trim_end_matches('/').to_string();

    if collapsed.len() > 2048 { return Err(Diagnostic::medium("URL IOC is too long")); }

    Ok(collapsed)

}



pub fn entry_from_parts(value: &str, disposition: ListDisposition, severity: Severity, confidence: u8, source: Option<String>, tags: Vec<String>) -> ParseResult<IocEntry> {

    Ok(IocEntry { kind: normalize_ioc_value(value)?, disposition, severity, confidence: confidence.min(100), source, tags })

}



pub fn disposition_from_word(word: &str) -> ListDisposition {

    match word.trim().to_ascii_lowercase().as_str() { "allow" | "allowlist" | "whitelist" | "trusted" => ListDisposition::Allow, "block" | "deny" | "blocklist" | "blacklist" | "malicious" => ListDisposition::Block, _ => ListDisposition::Observe }

}
