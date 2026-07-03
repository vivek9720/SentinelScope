use crate::diag::{Diagnostic, ParseResult};
use crate::endian::Endian;

#[derive(Debug, Clone)]
pub struct ByteReader<'a> { data: &'a [u8], pos: usize }

impl<'a> ByteReader<'a> {
    pub fn new(data: &'a [u8]) -> Self { Self { data, pos: 0 } }
    pub fn len(&self) -> usize { self.data.len() }
    pub fn remaining(&self) -> usize { self.data.len().saturating_sub(self.pos) }
    pub fn position(&self) -> usize { self.pos }
    pub fn is_empty(&self) -> bool { self.remaining() == 0 }
    pub fn original(&self) -> &'a [u8] { self.data }
    pub fn set_position(&mut self, pos: usize) -> ParseResult<()> {
        if pos > self.data.len() { return Err(Diagnostic::medium("reader seek outside input").at(pos)); }
        self.pos = pos;
        Ok(())
    }
    pub fn take(&mut self, count: usize) -> ParseResult<&'a [u8]> {
        if self.remaining() < count {
            return Err(Diagnostic::medium("truncated input").at(self.pos).with_context(format!("need {} bytes, have {}", count, self.remaining())));
        }
        let start = self.pos;
        self.pos += count;
        Ok(&self.data[start..start + count])
    }
    pub fn peek(&self, count: usize) -> ParseResult<&'a [u8]> {
        if self.remaining() < count { return Err(Diagnostic::medium("truncated input").at(self.pos)); }
        Ok(&self.data[self.pos..self.pos + count])
    }
    pub fn skip(&mut self, count: usize) -> ParseResult<()> { self.take(count).map(|_| ()) }
    pub fn take_u8(&mut self) -> ParseResult<u8> { Ok(self.take(1)?[0]) }
    pub fn take_u16(&mut self, endian: Endian) -> ParseResult<u16> { endian.read_u16(self.take(2)?) }
    pub fn take_u32(&mut self, endian: Endian) -> ParseResult<u32> { endian.read_u32(self.take(4)?) }
    pub fn take_be_u16(&mut self) -> ParseResult<u16> { self.take_u16(Endian::Big) }
    pub fn take_be_u32(&mut self) -> ParseResult<u32> { self.take_u32(Endian::Big) }
    pub fn take_le_u16(&mut self) -> ParseResult<u16> { self.take_u16(Endian::Little) }
    pub fn take_le_u32(&mut self) -> ParseResult<u32> { self.take_u32(Endian::Little) }
    pub fn take_rest(&mut self) -> &'a [u8] { let start = self.pos; self.pos = self.data.len(); &self.data[start..] }
    pub fn bounded_slice(data: &'a [u8], offset: usize, len: usize) -> ParseResult<&'a [u8]> {
        let end = offset.checked_add(len).ok_or_else(|| Diagnostic::medium("slice offset overflow"))?;
        if end > data.len() { return Err(Diagnostic::medium("slice outside input").at(offset)); }
        Ok(&data[offset..end])
    }
}
