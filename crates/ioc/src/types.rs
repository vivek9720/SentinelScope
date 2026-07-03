use sentinel_core::{CidrRange, Ipv4AddrEx, Severity};



#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub enum IocKind { Ip(Ipv4AddrEx), Cidr(CidrRange), Domain(String), Url(String), Hash(String) }

#[derive(Debug, Clone, PartialEq, Eq)]

pub enum ListDisposition { Observe, Allow, Block }

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct IocEntry { pub kind: IocKind, pub disposition: ListDisposition, pub severity: Severity, pub confidence: u8, pub source: Option<String>, pub tags: Vec<String> }



impl IocEntry {

    pub fn key(&self) -> String { match &self.kind { IocKind::Ip(ip) => format!("ip:{}", ip), IocKind::Cidr(cidr) => format!("cidr:{}", cidr), IocKind::Domain(domain) => format!("domain:{}", domain), IocKind::Url(url) => format!("url:{}", url), IocKind::Hash(hash) => format!("hash:{}", hash) } }

    pub fn blocks(&self) -> bool { self.disposition == ListDisposition::Block }

    pub fn allows(&self) -> bool { self.disposition == ListDisposition::Allow }

}



#[derive(Debug, Clone, PartialEq, Eq)]

pub struct IocMatch { pub key: String, pub field: String, pub value: String, pub disposition: ListDisposition, pub severity: Severity, pub confidence: u8 }
