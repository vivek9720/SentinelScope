use crate::diag::{Diagnostic, ParseResult};

use std::fmt;



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]

pub struct Ipv4AddrEx(pub u32);



impl Ipv4AddrEx {

    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self { Self(u32::from_be_bytes([a, b, c, d])) }

    pub fn octets(self) -> [u8; 4] { self.0.to_be_bytes() }

    pub fn is_private(self) -> bool { let [a,b,_,_] = self.octets(); a == 10 || (a == 172 && (16..=31).contains(&b)) || (a == 192 && b == 168) }

    pub fn is_loopback(self) -> bool { self.octets()[0] == 127 }

    pub fn is_multicast(self) -> bool { (224..=239).contains(&self.octets()[0]) }

    pub fn is_link_local(self) -> bool { let [a,b,_,_] = self.octets(); a == 169 && b == 254 }

    pub fn is_unspecified(self) -> bool { self.0 == 0 }

    pub fn is_global_unicast_candidate(self) -> bool { !self.is_private() && !self.is_loopback() && !self.is_multicast() && !self.is_link_local() && !self.is_unspecified() }

}



impl fmt::Display for Ipv4AddrEx {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { let [a,b,c,d] = self.octets(); write!(f, "{}.{}.{}.{}", a,b,c,d) }

}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

pub struct CidrRange { pub network: Ipv4AddrEx, pub prefix: u8 }



impl CidrRange {

    pub fn new(network: Ipv4AddrEx, prefix: u8) -> ParseResult<Self> {

        if prefix > 32 { return Err(Diagnostic::medium("CIDR prefix larger than 32")); }

        let mask = cidr_mask(prefix);

        Ok(Self { network: Ipv4AddrEx(network.0 & mask), prefix })

    }

    pub fn contains(self, ip: Ipv4AddrEx) -> bool { let mask = cidr_mask(self.prefix); (ip.0 & mask) == self.network.0 }

    pub fn size_hint(self) -> u64 { if self.prefix == 0 { 1u64 << 32 } else { 1u64 << (32 - self.prefix) } }

}



impl fmt::Display for CidrRange {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}/{}", self.network, self.prefix) }

}



pub fn cidr_mask(prefix: u8) -> u32 { if prefix == 0 { 0 } else { u32::MAX << (32 - prefix) } }



pub fn parse_ipv4(input: &str) -> ParseResult<Ipv4AddrEx> {

    let mut octets = [0u8; 4];

    let mut count = 0usize;

    for part in input.trim().split('.') {

        if count >= 4 || part.is_empty() { return Err(Diagnostic::medium("invalid IPv4 address")); }

        if part.len() > 1 && part.starts_with('0') { return Err(Diagnostic::low("IPv4 octet has leading zero")); }

        let value: u16 = part.parse().map_err(|_| Diagnostic::medium("invalid IPv4 octet"))?;

        if value > 255 { return Err(Diagnostic::medium("IPv4 octet outside range")); }

        octets[count] = value as u8;

        count += 1;

    }

    if count != 4 { return Err(Diagnostic::medium("IPv4 address has wrong octet count")); }

    Ok(Ipv4AddrEx::new(octets[0], octets[1], octets[2], octets[3]))

}



pub fn parse_cidr(input: &str) -> ParseResult<CidrRange> {

    let (addr, prefix) = input.trim().split_once('/').ok_or_else(|| Diagnostic::medium("CIDR range missing prefix"))?;

    let ip = parse_ipv4(addr)?;

    let prefix: u8 = prefix.parse().map_err(|_| Diagnostic::medium("invalid CIDR prefix"))?;

    CidrRange::new(ip, prefix)

}



pub fn parse_ip_or_cidr(input: &str) -> ParseResult<CidrRange> { if input.contains('/') { parse_cidr(input) } else { CidrRange::new(parse_ipv4(input)?, 32) } }



pub fn parse_port(input: &str) -> ParseResult<u16> {

    let s = input.trim();

    if s.eq_ignore_ascii_case("any") { return Ok(0); }

    let port: u32 = s.parse().map_err(|_| Diagnostic::medium("invalid port"))?;

    if port > 65535 { return Err(Diagnostic::medium("port outside range")); }

    Ok(port as u16)

}



pub fn parse_port_range(input: &str) -> ParseResult<(u16, u16)> {

    let s = input.trim();

    if s.eq_ignore_ascii_case("any") { return Ok((0, 65535)); }

    if let Some((a, b)) = s.split_once(':').or_else(|| s.split_once('-')) {

        let start = if a.is_empty() { 0 } else { parse_port(a)? };

        let end = if b.is_empty() { 65535 } else { parse_port(b)? };

        if start > end { return Err(Diagnostic::medium("port range is reversed")); }

        Ok((start, end))

    } else {

        let p = parse_port(s)?;

        Ok((p, p))

    }

}



pub fn ip_in_ranges(ip: Ipv4AddrEx, ranges: &[CidrRange]) -> bool { ranges.iter().any(|range| range.contains(ip)) }
