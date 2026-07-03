use crate::collection::IocCollection;

use crate::types::{IocKind, IocMatch};

use sentinel_packet::PacketMetadata;



pub fn match_packet_metadata(collection: &IocCollection, meta: &PacketMetadata) -> Vec<IocMatch> {

    let mut out = Vec::new();

    for entry in collection.entries() {

        match &entry.kind {

            IocKind::Ip(ip) => { if Some(*ip) == meta.source_ip { push(&mut out, entry, "source_ip", &ip.to_string()); } if Some(*ip) == meta.destination_ip { push(&mut out, entry, "destination_ip", &ip.to_string()); } }

            IocKind::Cidr(cidr) => { if meta.source_ip.map(|ip| cidr.contains(ip)).unwrap_or(false) { push(&mut out, entry, "source_cidr", &cidr.to_string()); } if meta.destination_ip.map(|ip| cidr.contains(ip)).unwrap_or(false) { push(&mut out, entry, "destination_cidr", &cidr.to_string()); } }

            IocKind::Domain(domain) => { for q in &meta.dns_queries { if domain_match(q, domain) { push(&mut out, entry, "dns_query", q); } } for a in &meta.dns_answers { if domain_match(a, domain) { push(&mut out, entry, "dns_answer", a); } } }

            IocKind::Url(url) => { for note in &meta.notes { if note.to_ascii_lowercase().contains(url) { push(&mut out, entry, "note_url", note); } } }

            IocKind::Hash(_) => {}

        }

    }

    out

}



fn push(out: &mut Vec<IocMatch>, entry: &crate::types::IocEntry, field: &str, value: &str) {

    out.push(IocMatch { key: entry.key(), field: field.to_string(), value: value.to_string(), disposition: entry.disposition.clone(), severity: entry.severity, confidence: entry.confidence });

}



pub fn domain_match(observed: &str, ioc: &str) -> bool { let obs = observed.trim_end_matches('.').to_ascii_lowercase(); obs == ioc || obs.ends_with(&format!(".{}", ioc)) }
