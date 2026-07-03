use sentinel_core::{DiagnosticSet, ValidationReport};

use crate::{metadata_from_frame, parse_pcap, linktype_name, PacketMetadata};



#[derive(Debug, Clone, Default)]

pub struct PacketSummary { pub records: usize, pub ipv4_records: usize, pub tcp_records: usize, pub udp_records: usize, pub dns_records: usize, pub endpoints: Vec<String>, pub diagnostics: Vec<String> }



impl PacketSummary {

    pub fn render_text(&self) -> String {

        let mut lines = Vec::new();

        lines.push(format!("records: {}", self.records)); lines.push(format!("ipv4: {}", self.ipv4_records)); lines.push(format!("tcp: {}", self.tcp_records)); lines.push(format!("udp: {}", self.udp_records)); lines.push(format!("dns: {}", self.dns_records));

        for endpoint in self.endpoints.iter().take(50) { lines.push(format!("endpoint: {}", endpoint)); }

        for diagnostic in &self.diagnostics { lines.push(format!("finding: {}", diagnostic)); }

        lines.join("\n")

    }

}



pub fn summarize_pcap(data: &[u8]) -> Result<PacketSummary, ValidationReport> {

    let parsed = match parse_pcap(data) { Ok(p) => p, Err(err) => { let mut report = ValidationReport::new("pcap"); report.add(err); return Err(report); } };

    let mut summary = PacketSummary { records: parsed.records.len(), ..Default::default() };

    if linktype_name(parsed.header.network) != "ethernet" { summary.diagnostics.push(format!("unsupported link type {}", parsed.header.network)); }

    for (idx, record) in parsed.records.iter().enumerate() { let (meta, diag) = metadata_from_frame(idx, record.data); accumulate(&mut summary, &meta, &diag); }

    Ok(summary)

}



fn accumulate(summary: &mut PacketSummary, meta: &PacketMetadata, diag: &DiagnosticSet) {

    if meta.source_ip.is_some() { summary.ipv4_records += 1; }

    match meta.protocol.as_deref() { Some("tcp") => summary.tcp_records += 1, Some("udp") => summary.udp_records += 1, _ => {} }

    if !meta.dns_queries.is_empty() || !meta.dns_answers.is_empty() { summary.dns_records += 1; }

    let key = meta.endpoint_key(); if !summary.endpoints.contains(&key) { summary.endpoints.push(key); }

    summary.diagnostics.extend(diag.iter().map(|d| d.to_string()));

}
