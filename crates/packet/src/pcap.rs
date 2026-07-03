use sentinel_core::{ByteReader, Diagnostic, Endian, ParseResult, Timestamp};



#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum PcapResolution { Micros, Nanos }

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct PcapGlobalHeader { pub endian: Endian, pub resolution: PcapResolution, pub version_major: u16, pub version_minor: u16, pub snaplen: u32, pub network: u32 }

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct PcapRecord<'a> { pub timestamp: Timestamp, pub included_len: u32, pub original_len: u32, pub data: &'a [u8] }

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct PcapFile<'a> { pub header: PcapGlobalHeader, pub records: Vec<PcapRecord<'a>> }



pub fn parse_pcap(data: &[u8]) -> ParseResult<PcapFile<'_>> {

    let mut reader = ByteReader::new(data);

    if reader.remaining() < 24 { return Err(Diagnostic::medium("PCAP input shorter than global header")); }

    let magic = reader.take(4)?;

    let (endian, resolution) = match magic {

        [0xd4, 0xc3, 0xb2, 0xa1] => (Endian::Little, PcapResolution::Micros),

        [0xa1, 0xb2, 0xc3, 0xd4] => (Endian::Big, PcapResolution::Micros),

        [0x4d, 0x3c, 0xb2, 0xa1] => (Endian::Little, PcapResolution::Nanos),

        [0xa1, 0xb2, 0x3c, 0x4d] => (Endian::Big, PcapResolution::Nanos),

        _ => return Err(Diagnostic::medium("unrecognized PCAP magic")),

    };

    let version_major = reader.take_u16(endian)?; let version_minor = reader.take_u16(endian)?;

    let _thiszone = reader.take_u32(endian)?; let _sigfigs = reader.take_u32(endian)?;

    let snaplen = reader.take_u32(endian)?; let network = reader.take_u32(endian)?;

    if snaplen == 0 || snaplen > 16_777_216 { return Err(Diagnostic::medium("PCAP snaplen outside expected range")); }

    let header = PcapGlobalHeader { endian, resolution, version_major, version_minor, snaplen, network };

    let mut records = Vec::new();

    while reader.remaining() >= 16 {

        let ts_sec = reader.take_u32(endian)? as i64; let ts_frac = reader.take_u32(endian)?;

        let included_len = reader.take_u32(endian)?; let original_len = reader.take_u32(endian)?;

        if included_len > snaplen { return Err(Diagnostic::medium("PCAP record included length exceeds snaplen").at(reader.position())); }

        if included_len > original_len { return Err(Diagnostic::medium("PCAP record included length exceeds original length").at(reader.position())); }

        let bytes = reader.take(included_len as usize)?;

        let timestamp = match resolution { PcapResolution::Micros => Timestamp::from_micros(ts_sec, ts_frac), PcapResolution::Nanos => Timestamp::from_nanos(ts_sec, ts_frac) };

        records.push(PcapRecord { timestamp, included_len, original_len, data: bytes });

        if records.len() > 100_000 { return Err(Diagnostic::medium("PCAP record limit exceeded")); }

    }

    if !reader.is_empty() { return Err(Diagnostic::low("PCAP trailing partial record").at(reader.position())); }

    Ok(PcapFile { header, records })

}



pub fn linktype_name(network: u32) -> &'static str { match network { 1 => "ethernet", 101 => "raw-ip", 113 => "linux-sll", 127 => "ieee802_11", 228 => "ipv4", 229 => "ipv6", _ => "unknown" } }
