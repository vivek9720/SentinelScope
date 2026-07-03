use std::collections::{BTreeMap, BTreeSet};

use crate::types::{IocEntry, IocKind, ListDisposition};

use sentinel_core::{CidrRange, Ipv4AddrEx, ValidationReport, Diagnostic};



#[derive(Debug, Clone, Default)]

pub struct IocCollection { entries: Vec<IocEntry>, keys: BTreeSet<String> }



impl IocCollection {

    pub fn new() -> Self { Self { entries: Vec::new(), keys: BTreeSet::new() } }

    pub fn entries(&self) -> &[IocEntry] { &self.entries }

    pub fn len(&self) -> usize { self.entries.len() }

    pub fn is_empty(&self) -> bool { self.entries.is_empty() }

    pub fn insert(&mut self, entry: IocEntry) -> bool { let key = entry.key(); let fresh = self.keys.insert(key); if fresh { self.entries.push(entry); } fresh }

    pub fn extend<I: IntoIterator<Item = IocEntry>>(&mut self, entries: I) -> usize { let mut added = 0; for entry in entries { if self.insert(entry) { added += 1; } } added }

    pub fn duplicates(entries: &[IocEntry]) -> Vec<String> { let mut counts: BTreeMap<String, usize> = BTreeMap::new(); for entry in entries { *counts.entry(entry.key()).or_default() += 1; } counts.into_iter().filter(|(_, c)| *c > 1).map(|(k, _)| k).collect() }

    pub fn validate(&self) -> ValidationReport {

        let mut report = ValidationReport::new("ioc collection");

        for entry in &self.entries {

            if entry.confidence > 100 { report.add(Diagnostic::medium("IOC confidence above 100")); }

            if matches!(&entry.kind, IocKind::Cidr(CidrRange { prefix: 0, .. })) && entry.disposition == ListDisposition::Block { report.add(Diagnostic::high("blocklist contains default route CIDR")); }
        }

        report

    }

    pub fn allowed_ip(&self, ip: Ipv4AddrEx) -> bool { self.entries.iter().any(|e| e.disposition == ListDisposition::Allow && match &e.kind { IocKind::Ip(v) => *v == ip, IocKind::Cidr(c) => c.contains(ip), _ => false }) }
}
