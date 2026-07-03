use sentinel_core::{ByteReader, Diagnostic, ParseResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsHeader { pub id: u16, pub flags: u16, pub questions: u16, pub answers: u16, pub authorities: u16, pub additionals: u16 }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion { pub name: String, pub qtype: u16, pub qclass: u16 }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsRecord { pub name: String, pub rrtype: u16, pub class: u16, pub ttl: u32, pub data: Vec<u8> }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsMessage { pub header: DnsHeader, pub questions: Vec<DnsQuestion>, pub answers: Vec<DnsRecord>, pub authorities: Vec<DnsRecord>, pub additionals: Vec<DnsRecord> }

impl DnsHeader { pub fn qr(&self) -> bool { self.flags & 0x8000 != 0 } pub fn opcode(&self) -> u8 { ((self.flags >> 11) & 0x0f) as u8 } pub fn rcode(&self) -> u8 { (self.flags & 0x0f) as u8 } }

pub fn parse_dns_message(data: &[u8]) -> ParseResult<DnsMessage> {
    let mut reader = ByteReader::new(data);
    if reader.remaining() < 12 { return Err(Diagnostic::medium("DNS message shorter than header")); }
    let header = DnsHeader { id: reader.take_be_u16()?, flags: reader.take_be_u16()?, questions: reader.take_be_u16()?, answers: reader.take_be_u16()?, authorities: reader.take_be_u16()?, additionals: reader.take_be_u16()? };
    let mut questions = Vec::new();
    for _ in 0..header.questions.min(64) { let name = read_name(data, &mut reader, 0)?; let qtype = reader.take_be_u16()?; let qclass = reader.take_be_u16()?; questions.push(DnsQuestion { name, qtype, qclass }); }
    let answers = read_records(data, &mut reader, header.answers)?;
    let authorities = read_records(data, &mut reader, header.authorities)?;
    let additionals = read_records(data, &mut reader, header.additionals)?;
    Ok(DnsMessage { header, questions, answers, authorities, additionals })
}

fn read_records(data: &[u8], reader: &mut ByteReader<'_>, count: u16) -> ParseResult<Vec<DnsRecord>> {
    let mut out = Vec::new();
    for _ in 0..count.min(128) {
        let name = read_name(data, reader, 0)?; let rrtype = reader.take_be_u16()?; let class = reader.take_be_u16()?; let ttl = reader.take_be_u32()?; let len = reader.take_be_u16()? as usize; let data_bytes = reader.take(len)?.to_vec();
        out.push(DnsRecord { name, rrtype, class, ttl, data: data_bytes });
    }
    Ok(out)
}

fn read_name(data: &[u8], reader: &mut ByteReader<'_>, depth: usize) -> ParseResult<String> {
    if depth > 12 { return Err(Diagnostic::medium("DNS name compression loop suspected")); }
    let mut labels = Vec::new();
    loop {
        let len = reader.take_u8()?;
        if len == 0 { break; }
        if len & 0xc0 == 0xc0 {
            let next = reader.take_u8()?; let ptr = (((len as u16 & 0x3f) << 8) | next as u16) as usize;
            if ptr >= data.len() { return Err(Diagnostic::medium("DNS compression pointer outside message")); }
            let mut nested = ByteReader::new(data); nested.set_position(ptr)?; labels.push(read_name(data, &mut nested, depth + 1)?); break;
        }
        if len & 0xc0 != 0 { return Err(Diagnostic::medium("unsupported DNS label type")); }
        if len > 63 { return Err(Diagnostic::medium("DNS label too long")); }
        let bytes = reader.take(len as usize)?;
        if !bytes.iter().all(|b| b.is_ascii_alphanumeric() || *b == b'-' || *b == b'_') { labels.push(sentinel_core::printable_ascii(bytes)); }
        else { labels.push(String::from_utf8_lossy(bytes).to_ascii_lowercase()); }
    }
    Ok(labels.into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join("."))
}

pub fn dns_type_name(value: u16) -> &'static str { match value { 1 => "A", 2 => "NS", 5 => "CNAME", 6 => "SOA", 15 => "MX", 16 => "TXT", 28 => "AAAA", 33 => "SRV", 65 => "HTTPS", 255 => "ANY", _ => "TYPE" } }
