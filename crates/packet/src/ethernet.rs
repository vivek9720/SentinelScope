use sentinel_core::{ByteReader, Diagnostic, ParseResult};



#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct MacAddress(pub [u8; 6]);

impl MacAddress { pub fn is_multicast(self) -> bool { self.0[0] & 1 == 1 } pub fn is_broadcast(self) -> bool { self.0 == [0xff; 6] } pub fn is_zero(self) -> bool { self.0 == [0; 6] } pub fn render(self) -> String { self.0.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(":") } }



#[derive(Debug, Clone, PartialEq, Eq)]

pub struct EthernetFrame<'a> { pub destination: MacAddress, pub source: MacAddress, pub ethertype: u16, pub vlan_tags: Vec<u16>, pub payload: &'a [u8] }



pub fn parse_ethernet_frame(data: &[u8]) -> ParseResult<EthernetFrame<'_>> {

    let mut reader = ByteReader::new(data);

    if reader.remaining() < 14 { return Err(Diagnostic::medium("Ethernet frame is shorter than header")); }

    let mut dst = [0u8; 6]; dst.copy_from_slice(reader.take(6)?);

    let mut src = [0u8; 6]; src.copy_from_slice(reader.take(6)?);

    let mut ethertype = reader.take_be_u16()?;

    let mut tags = Vec::new();

    while ethertype == 0x8100 || ethertype == 0x88a8 {

        if reader.remaining() < 4 { return Err(Diagnostic::medium("truncated VLAN tag").at(reader.position())); }

        tags.push(reader.take_be_u16()?);

        ethertype = reader.take_be_u16()?;

        if tags.len() > 4 { return Err(Diagnostic::medium("too many nested VLAN tags")); }

    }

    Ok(EthernetFrame { destination: MacAddress(dst), source: MacAddress(src), ethertype, vlan_tags: tags, payload: reader.take_rest() })

}



pub fn ethertype_name(value: u16) -> &'static str { match value { 0x0800 => "ipv4", 0x0806 => "arp", 0x86dd => "ipv6", 0x8100 => "vlan", 0x88a8 => "provider-bridge", _ => "unknown" } }
