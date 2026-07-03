pub fn trim_ascii(input: &str) -> &str { input.trim_matches(|c: char| c.is_ascii_whitespace() || c == '\u{feff}') }



pub fn collapse_spaces(input: &str) -> String {

    let mut out = String::new();

    let mut last_space = false;

    for ch in input.chars() {

        if ch.is_ascii_whitespace() { if !last_space { out.push(' '); } last_space = true; }

        else { out.push(ch); last_space = false; }

    }

    out.trim().to_string()

}



pub fn strip_comment(line: &str) -> &str {

    let mut in_quote = false;

    let mut escape = false;

    for (idx, ch) in line.char_indices() {

        if escape { escape = false; continue; }

        match ch {

            '\\' if in_quote => escape = true,

            '"' => in_quote = !in_quote,

            '#' | ';' if !in_quote => return &line[..idx],

            _ => {}

        }

    }

    line

}



pub fn split_csv_line(line: &str) -> Vec<String> {

    let mut out = Vec::new();

    let mut current = String::new();

    let mut in_quote = false;

    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {

        match ch {

            '"' if in_quote && chars.peek() == Some(&'"') => { current.push('"'); chars.next(); }

            '"' => in_quote = !in_quote,

            ',' if !in_quote => { out.push(current.trim().to_string()); current.clear(); }

            _ => current.push(ch),

        }

    }

    out.push(current.trim().to_string());

    out

}



pub fn printable_ascii(bytes: &[u8]) -> String { bytes.iter().map(|&b| if b.is_ascii_graphic() || b == b' ' { b as char } else { '.' }).collect() }

pub fn lowercase_ascii(input: &str) -> String { input.chars().map(|c| c.to_ascii_lowercase()).collect() }



pub fn normalize_domain(input: &str) -> Option<String> {

    let mut s = input.trim().trim_end_matches('.').to_ascii_lowercase();

    if s.starts_with("*.") { s = s[2..].to_string(); }

    if s.is_empty() || s.len() > 253 { return None; }

    if !s.split('.').all(valid_domain_label) { return None; }

    Some(s)

}



fn valid_domain_label(label: &str) -> bool {

    if label.is_empty() || label.len() > 63 { return false; }

    let bytes = label.as_bytes();

    if bytes[0] == b'-' || bytes[bytes.len() - 1] == b'-' { return false; }

    bytes.iter().all(|b| b.is_ascii_alphanumeric() || *b == b'-')

}



pub fn looks_like_hex(input: &str, lengths: &[usize]) -> bool { lengths.contains(&input.len()) && input.bytes().all(|b| b.is_ascii_hexdigit()) }



pub fn hex_to_bytes(input: &str) -> Option<Vec<u8>> {

    let s = input.trim();

    if s.len() % 2 != 0 || !s.bytes().all(|b| b.is_ascii_hexdigit()) { return None; }

    let mut out = Vec::with_capacity(s.len() / 2);

    let bytes = s.as_bytes();

    let mut i = 0;

    while i < bytes.len() {

        let hi = hex_value(bytes[i])?;

        let lo = hex_value(bytes[i + 1])?;

        out.push((hi << 4) | lo);

        i += 2;

    }

    Some(out)

}



fn hex_value(b: u8) -> Option<u8> {

    match b { b'0'..=b'9' => Some(b - b'0'), b'a'..=b'f' => Some(b - b'a' + 10), b'A'..=b'F' => Some(b - b'A' + 10), _ => None }

}
