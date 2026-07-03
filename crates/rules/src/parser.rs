use sentinel_core::{Diagnostic, DiagnosticSet};

use crate::lexer::{lex_rule, Token};

use crate::types::{IdsRule, RuleOption};



#[derive(Debug, Clone, Default)]

pub struct RuleParseOutput { pub rules: Vec<IdsRule>, pub diagnostics: DiagnosticSet }



pub fn parse_rules_text(text: &str) -> RuleParseOutput {

    let mut out = RuleParseOutput::default();

    for (idx, line) in text.lines().enumerate() {

        let clean = line.trim();

        if clean.is_empty() || clean.starts_with('#') { continue; }

        match parse_rule_line(clean) { Ok(rule) => out.rules.push(rule), Err(err) => out.diagnostics.push(err.at(idx + 1)) }

    }

    out

}

pub fn parse_rules_bytes(data: &[u8]) -> RuleParseOutput { parse_rules_text(&String::from_utf8_lossy(data)) }



pub fn parse_rule_line(line: &str) -> Result<IdsRule, Diagnostic> {

    let open = line.find('(').ok_or_else(|| Diagnostic::medium("IDS rule missing options"))?;

    let close = line.rfind(')').ok_or_else(|| Diagnostic::medium("IDS rule missing closing parenthesis"))?;

    if close <= open { return Err(Diagnostic::medium("IDS rule option block is malformed")); }

    let header = line[..open].split_whitespace().collect::<Vec<_>>();

    if header.len() != 7 { return Err(Diagnostic::medium("IDS rule header must have seven fields")); }

    let options = parse_options(&line[open + 1..close])?;

    Ok(IdsRule { action: header[0].to_ascii_lowercase(), protocol: header[1].to_ascii_lowercase(), source: header[2].to_string(), source_port: header[3].to_string(), direction: header[4].to_string(), destination: header[5].to_string(), destination_port: header[6].to_string(), options })

}



fn parse_options(input: &str) -> Result<Vec<RuleOption>, Diagnostic> {

    let tokens = lex_rule(input)?; let mut out = Vec::new(); let mut idx = 0usize;

    while idx < tokens.len() {

        match &tokens[idx] {

            Token::Semicolon => { idx += 1; }

            Token::Word(key) => {

                let key = key.to_ascii_lowercase(); idx += 1; let mut value = None;

                if matches!(tokens.get(idx), Some(Token::Colon)) {

                    idx += 1;

                    match tokens.get(idx) { Some(Token::StringLiteral(s)) | Some(Token::Word(s)) => { value = Some(s.clone()); idx += 1; }, _ => return Err(Diagnostic::medium("rule option missing value")) }

                }

                out.push(RuleOption { key, value });

                if matches!(tokens.get(idx), Some(Token::Semicolon)) { idx += 1; }

            }

            _ => return Err(Diagnostic::medium("unexpected token in rule options")),

        }

    }

    Ok(out)

}
