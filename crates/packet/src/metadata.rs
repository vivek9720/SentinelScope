use sentinel_core::{service_name, DiagnosticSet, Ipv4AddrEx};
use crate::{parse_dns_message, parse_ethernet_frame, parse_ipv4_packet, parse_tcp_segment, parse_udp_datagram};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PacketMetadata { pub frame_index: usize, pub source_ip: Option<Ipv4AddrEx>, pub destination_ip: Option<Ipv4AddrEx>, pub protocol: Option<String>, pub source_port: Option<u16>, pub destination_port: Option<u16>, pub dns_queries: Vec<String>, pub dns_answers: Vec<String>, pub tcp_flags: Vec<String>, pub payload_len: usize, pub notes: Vec<String> }

impl PacketMetadata {
    pub fn endpoint_key(&self) -> String {
        format!("{}:{} -> {}:{} {}", self.source_ip.map(|v| v.to_string()).unwrap_or_else(|| "?".to_string()), self.source_port.map(|v| v.to_string()).unwrap_or_else(|| "?".to_string()), self.destination_ip.map(|v| v.to_string()).unwrap_or_else(|| "?".to_string()), self.destination_port.map(|v| v.to_string()).unwrap_or_else(|| "?".to_string()), self.protocol.as_deref().unwrap_or("unknown"))
    }
    pub fn service_hint(&self) -> Option<&'static str> { let proto = self.protocol.as_deref()?; let port = self.destination_port.or(self.source_port)?; service_name(port, proto) }
}

pub fn metadata_from_frame(index: usize, frame: &[u8]) -> (PacketMetadata, DiagnosticSet) {
    let mut meta = PacketMetadata { frame_index: index, payload_len: frame.len(), ..Default::default() };
    let mut diagnostics = DiagnosticSet::new();
    let eth = match parse_ethernet_frame(frame) { Ok(value) => value, Err(err) => { diagnostics.push(err); return (meta, diagnostics); } };
    if eth.source.is_zero() { meta.notes.push("zero source mac".to_string()); }
    if eth.destination.is_broadcast() { meta.notes.push("broadcast destination mac".to_string()); }
    if eth.ethertype != 0x0800 { meta.notes.push(format!("non-ipv4 ethertype 0x{:04x}", eth.ethertype)); return (meta, diagnostics); }
    let ip = match parse_ipv4_packet(eth.payload) { Ok(value) => value, Err(err) => { diagnostics.push(err); return (meta, diagnostics); } };
    meta.source_ip = Some(ip.source); meta.destination_ip = Some(ip.destination); meta.protocol = Some(ip.transport_name().to_string()); meta.payload_len = ip.payload.len();
    if !ip.checksum_valid { meta.notes.push("invalid ipv4 checksum".to_string()); }
    match ip.protocol {
        6 => match parse_tcp_segment(ip.payload) { Ok(tcp) => { meta.source_port = Some(tcp.source_port); meta.destination_port = Some(tcp.destination_port); meta.tcp_flags = tcp.flag_names().into_iter().map(|s| s.to_string()).collect(); meta.payload_len = tcp.payload.len(); } Err(err) => diagnostics.push(err) },
        17 => match parse_udp_datagram(ip.payload) { Ok(udp) => { meta.source_port = Some(udp.source_port); meta.destination_port = Some(udp.destination_port); meta.payload_len = udp.payload.len(); if udp.source_port == 53 || udp.destination_port == 53 { if let Ok(dns) = parse_dns_message(udp.payload) { meta.dns_queries = dns.questions.into_iter().map(|q| q.name).collect(); meta.dns_answers = dns.answers.into_iter().map(|a| a.name).collect(); } } } Err(err) => diagnostics.push(err) },
        _ => {}
    }
    (meta, diagnostics)
}
