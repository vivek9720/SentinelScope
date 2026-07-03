use sentinel_core::{internet_checksum, ByteReader, Diagnostic, Ipv4AddrEx, ParseResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ipv4Packet<'a> { pub version: u8, pub ihl: u8, pub dscp_ecn: u8, pub total_length: u16, pub identification: u16, pub flags: u8, pub fragment_offset: u16, pub ttl: u8, pub protocol: u8, pub checksum: u16, pub source: Ipv4AddrEx, pub destination: Ipv4AddrEx, pub options: &'a [u8], pub payload: &'a [u8], pub checksum_valid: bool }

impl<'a> Ipv4Packet<'a> { pub fn is_fragmented(&self) -> bool { self.fragment_offset != 0 || (self.flags & 0b001) != 0 } pub fn transport_name(&self) -> &'static str { match self.protocol { 6 => "tcp", 17 => "udp", 1 => "icmp", _ => "other" } } }

pub fn parse_ipv4_packet(data: &[u8]) -> ParseResult<Ipv4Packet<'_>> {
    let mut reader = ByteReader::new(data);
    if reader.remaining() < 20 { return Err(Diagnostic::medium("IPv4 packet is shorter than base header")); }
    let first = reader.take_u8()?; let version = first >> 4; let ihl = first & 0x0f;
    if version != 4 { return Err(Diagnostic::medium("packet is not IPv4")); }
    if ihl < 5 { return Err(Diagnostic::medium("IPv4 header length is too small")); }
    let header_len = ihl as usize * 4;
    if data.len() < header_len { return Err(Diagnostic::medium("truncated IPv4 header")); }
    let dscp_ecn = reader.take_u8()?;
    let total_length = reader.take_be_u16()?;
    if total_length as usize > data.len() { return Err(Diagnostic::medium("IPv4 total length exceeds captured bytes")); }
    if (total_length as usize) < header_len { return Err(Diagnostic::medium("IPv4 total length is smaller than header")); }
    let identification = reader.take_be_u16()?;
    let flags_fragment = reader.take_be_u16()?;
    let flags = ((flags_fragment >> 13) & 0x7) as u8;
    let fragment_offset = flags_fragment & 0x1fff;
    let ttl = reader.take_u8()?; let protocol = reader.take_u8()?; let checksum = reader.take_be_u16()?;
    let src_raw = reader.take(4)?; let dst_raw = reader.take(4)?;
    let source = Ipv4AddrEx::new(src_raw[0], src_raw[1], src_raw[2], src_raw[3]);
    let destination = Ipv4AddrEx::new(dst_raw[0], dst_raw[1], dst_raw[2], dst_raw[3]);
    let options = reader.take(header_len - 20)?;
    let payload = &data[header_len..header_len + (total_length as usize - header_len)];
    let checksum_valid = internet_checksum(&data[..header_len]) == 0;
    Ok(Ipv4Packet { version, ihl, dscp_ecn, total_length, identification, flags, fragment_offset, ttl, protocol, checksum, source, destination, options, payload, checksum_valid })
}
