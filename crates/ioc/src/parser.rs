use sentinel_core::{collapse_spaces, split_csv_line, strip_comment, Diagnostic, DiagnosticSet, Severity};

use crate::normalize::{disposition_from_word, entry_from_parts};

use crate::types::{IocEntry, ListDisposition};



#[derive(Debug, Clone, Default)]

pub struct IocParseOutput { pub entries: Vec<IocEntry>, pub diagnostics: DiagnosticSet }



pub fn parse_ioc_text(text: &str) -> IocParseOutput {

    let mut out = IocParseOutput::default();

    for (idx, line) in text.lines().enumerate() {

        let stripped = strip_comment(line).trim();

        if stripped.is_empty() { continue; }

        match parse_ioc_line(stripped) { Ok(entry) => out.entries.push(entry), Err(err) => out.diagnostics.push(err.at(idx + 1)) }

    }

    out

}



pub fn parse_ioc_bytes(data: &[u8]) -> IocParseOutput { parse_ioc_text(&String::from_utf8_lossy(data)) }



pub fn parse_ioc_line(line: &str) -> Result<IocEntry, Diagnostic> {

    if line.trim_start().starts_with('{') { return parse_jsonish_line(line); }

    if line.contains(',') { return parse_csv_line(line); }

    let compact = collapse_spaces(line);

    let mut parts = compact.split_whitespace();

    let value = parts.next().ok_or_else(|| Diagnostic::medium("missing IOC value"))?;

    let mut disposition = ListDisposition::Observe; let mut severity = Severity::Medium; let mut confidence = 50u8; let mut tags = Vec::new(); let mut source = None;

    for part in parts {

        if let Some(v) = part.strip_prefix("severity=") { severity = Severity::from_word(v).unwrap_or(severity); }

        else if let Some(v) = part.strip_prefix("confidence=") { confidence = v.parse().unwrap_or(confidence); }

        else if let Some(v) = part.strip_prefix("source=") { source = Some(v.to_string()); }

        else if let Some(v) = part.strip_prefix("tag=") { tags.push(v.to_string()); }

        else { disposition = disposition_from_word(part); }

    }

    entry_from_parts(value, disposition, severity, confidence, source, tags)

}



fn parse_csv_line(line: &str) -> Result<IocEntry, Diagnostic> {

    let fields = split_csv_line(line);

    let value = fields.get(0).map(String::as_str).unwrap_or("");

    let disposition = fields.get(1).map(|s| disposition_from_word(s)).unwrap_or(ListDisposition::Observe);

    let severity = fields.get(2).and_then(|s| Severity::from_word(s)).unwrap_or(Severity::Medium);

    let confidence = fields.get(3).and_then(|s| s.parse().ok()).unwrap_or(50);

    let source = fields.get(4).filter(|s| !s.is_empty()).cloned();

    let tags = fields.iter().skip(5).filter(|s| !s.is_empty()).cloned().collect();

    entry_from_parts(value, disposition, severity, confidence, source, tags)

}



fn parse_jsonish_line(line: &str) -> Result<IocEntry, Diagnostic> {

    let mut value = None; let mut disposition = ListDisposition::Observe; let mut severity = Severity::Medium; let mut confidence = 50u8;

    let body = line.trim().trim_start_matches('{').trim_end_matches('}');

    for field in body.split(',') {

        let Some((k, v)) = field.split_once(':') else { continue; };

        let key = k.trim().trim_matches('"'); let val = v.trim().trim_matches('"');

        match key { "value" | "ioc" | "indicator" => value = Some(val.to_string()), "disposition" | "list" => disposition = disposition_from_word(val), "severity" => severity = Severity::from_word(val).unwrap_or(severity), "confidence" => confidence = val.parse().unwrap_or(confidence), _ => {} }

    }

    entry_from_parts(value.as_deref().ok_or_else(|| Diagnostic::medium("JSON IOC missing value"))?, disposition, severity, confidence, None, Vec::new())

}
