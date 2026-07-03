use crate::diag::{Diagnostic, ParseResult};



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]

pub struct Timestamp { pub seconds: i64, pub nanos: u32 }



impl Timestamp {

    pub fn new(seconds: i64, nanos: u32) -> Self { Self { seconds, nanos: nanos.min(999_999_999) } }

    pub fn from_micros(seconds: i64, micros: u32) -> Self { Self::new(seconds, micros.saturating_mul(1000)) }

    pub fn from_nanos(seconds: i64, nanos: u32) -> Self { Self::new(seconds, nanos) }

    pub fn as_millis(self) -> i128 { self.seconds as i128 * 1000 + (self.nanos as i128 / 1_000_000) }

}



pub fn parse_epoch_seconds(input: &str) -> ParseResult<Timestamp> {

    let trimmed = input.trim();

    if trimmed.is_empty() { return Err(Diagnostic::medium("empty timestamp")); }

    let mut parts = trimmed.splitn(2, '.');

    let seconds: i64 = parts.next().unwrap_or_default().parse().map_err(|_| Diagnostic::medium("invalid epoch seconds"))?;

    let nanos = if let Some(frac) = parts.next() {

        let mut value = 0u32;

        let mut scale = 100_000_000u32;

        for b in frac.bytes().take(9) {

            if !b.is_ascii_digit() { return Err(Diagnostic::medium("invalid epoch fraction")); }

            value += ((b - b'0') as u32) * scale;

            scale /= 10;

        }

        value

    } else { 0 };

    Ok(Timestamp::new(seconds, nanos))

}



pub fn parse_duration_seconds(input: &str) -> ParseResult<u64> {

    let s = input.trim();

    if s.is_empty() { return Err(Diagnostic::medium("empty duration")); }

    let (num, mult) = match s.as_bytes()[s.len() - 1] {

        b's' | b'S' => (&s[..s.len() - 1], 1),

        b'm' | b'M' => (&s[..s.len() - 1], 60),

        b'h' | b'H' => (&s[..s.len() - 1], 3600),

        b'd' | b'D' => (&s[..s.len() - 1], 86400),

        _ => (s, 1),

    };

    let base: u64 = num.parse().map_err(|_| Diagnostic::medium("invalid duration number"))?;

    base.checked_mul(mult).ok_or_else(|| Diagnostic::medium("duration overflow"))

}
