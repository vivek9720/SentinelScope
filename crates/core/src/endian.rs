use crate::diag::{Diagnostic, ParseResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian { Little, Big }

impl Endian {
    pub fn read_u16(self, bytes: &[u8]) -> ParseResult<u16> {
        if bytes.len() < 2 { return Err(Diagnostic::medium("not enough bytes for u16")); }
        Ok(match self { Endian::Little => u16::from_le_bytes([bytes[0], bytes[1]]), Endian::Big => u16::from_be_bytes([bytes[0], bytes[1]]) })
    }
    pub fn read_u32(self, bytes: &[u8]) -> ParseResult<u32> {
        if bytes.len() < 4 { return Err(Diagnostic::medium("not enough bytes for u32")); }
        Ok(match self { Endian::Little => u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]), Endian::Big => u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) })
    }
    pub fn write_u16(self, value: u16) -> [u8; 2] { match self { Endian::Little => value.to_le_bytes(), Endian::Big => value.to_be_bytes() } }
    pub fn write_u32(self, value: u32) -> [u8; 4] { match self { Endian::Little => value.to_le_bytes(), Endian::Big => value.to_be_bytes() } }
}

pub fn be_u16(bytes: &[u8]) -> ParseResult<u16> { Endian::Big.read_u16(bytes) }
pub fn be_u32(bytes: &[u8]) -> ParseResult<u32> { Endian::Big.read_u32(bytes) }
pub fn le_u16(bytes: &[u8]) -> ParseResult<u16> { Endian::Little.read_u16(bytes) }
pub fn le_u32(bytes: &[u8]) -> ParseResult<u32> { Endian::Little.read_u32(bytes) }
