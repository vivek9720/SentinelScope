use sentinel_core::{ByteReader, Diagnostic, ParseResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TcpSegment<'a> { pub source_port: u16, pub destination_port: u16, pub sequence: u32, pub acknowledgement: u32, pub data_offset: u8, pub flags: u16, pub window: u16, pub checksum: u16, pub urgent_pointer: u16, pub options: &'a [u8], pub payload: &'a [u8] }

impl<'a> TcpSegment<'a> {
    pub fn syn(&self) -> bool { self.flags & 0x02 != 0 } pub fn ack(&self) -> bool { self.flags & 0x10 != 0 } pub fn fin(&self) -> bool { self.flags & 0x01 != 0 } pub fn rst(&self) -> bool { self.flags & 0x04 != 0 } pub fn psh(&self) -> bool { self.flags & 0x08 != 0 }
    pub fn flag_names(&self) -> Vec<&'static str> { let mut out = Vec::new(); if self.fin() { out.push("FIN"); } if self.syn() { out.push("SYN"); } if self.rst() { out.push("RST"); } if self.psh() { out.push("PSH"); } if self.ack() { out.push("ACK"); } if self.flags & 0x20 != 0 { out.push("URG"); } if self.flags & 0x40 != 0 { out.push("ECE"); } if self.flags & 0x80 != 0 { out.push("CWR"); } out }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UdpDatagram<'a> { pub source_port: u16, pub destination_port: u16, pub length: u16, pub checksum: u16, pub payload: &'a [u8] }

pub fn parse_tcp_segment(data: &[u8]) -> ParseResult<TcpSegment<'_>> {
    let mut reader = ByteReader::new(data);
    if reader.remaining() < 20 { return Err(Diagnostic::medium("TCP segment shorter than header")); }
    let source_port = reader.take_be_u16()?; let destination_port = reader.take_be_u16()?; let sequence = reader.take_be_u32()?; let acknowledgement = reader.take_be_u32()?;
    let offset_flags = reader.take_be_u16()?; let data_offset = ((offset_flags >> 12) & 0x0f) as u8;
    if data_offset < 5 { return Err(Diagnostic::medium("TCP data offset is too small")); }
    let header_len = data_offset as usize * 4;
    if data.len() < header_len { return Err(Diagnostic::medium("truncated TCP options")); }
    let flags = offset_flags & 0x01ff; let window = reader.take_be_u16()?; let checksum = reader.take_be_u16()?; let urgent_pointer = reader.take_be_u16()?;
    let options = reader.take(header_len - 20)?; let payload = &data[header_len..];
    Ok(TcpSegment { source_port, destination_port, sequence, acknowledgement, data_offset, flags, window, checksum, urgent_pointer, options, payload })
}

pub fn parse_udp_datagram(data: &[u8]) -> ParseResult<UdpDatagram<'_>> {
    let mut reader = ByteReader::new(data);
    if reader.remaining() < 8 { return Err(Diagnostic::medium("UDP datagram shorter than header")); }
    let source_port = reader.take_be_u16()?; let destination_port = reader.take_be_u16()?; let length = reader.take_be_u16()?; let checksum = reader.take_be_u16()?;
    if length < 8 { return Err(Diagnostic::medium("UDP length is smaller than header")); }
    if length as usize > data.len() { return Err(Diagnostic::medium("UDP length exceeds captured bytes")); }
    Ok(UdpDatagram { source_port, destination_port, length, checksum, payload: &data[8..length as usize] })
}

pub fn port_role(port: u16) -> &'static str { if port == 0 { "unspecified" } else if port < 1024 { "well-known" } else if port < 49152 { "registered" } else { "dynamic" } }
