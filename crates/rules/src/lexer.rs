use sentinel_core::{Diagnostic, ParseResult};



#[derive(Debug, Clone, PartialEq, Eq)]

pub enum Token { Word(String), StringLiteral(String), LParen, RParen, Semicolon, Colon }



pub fn lex_rule(input: &str) -> ParseResult<Vec<Token>> {

    let mut out = Vec::new();

    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {

        match ch {

            c if c.is_ascii_whitespace() => {}

            '(' => out.push(Token::LParen),

            ')' => out.push(Token::RParen),

            ';' => out.push(Token::Semicolon),

            ':' => out.push(Token::Colon),

            '"' => {

                let mut s = String::new(); let mut closed = false;

                while let Some(c) = chars.next() { match c { '\\' => if let Some(next) = chars.next() { s.push(next); }, '"' => { closed = true; break; }, _ => s.push(c) } }

                if !closed { return Err(Diagnostic::medium("unterminated rule string")); }

                out.push(Token::StringLiteral(s));

            }

            _ => {

                let mut word = String::new(); word.push(ch);

                while let Some(&c) = chars.peek() { if c.is_ascii_whitespace() || matches!(c, '(' | ')' | ';' | ':' | '"') { break; } word.push(c); chars.next(); }

                out.push(Token::Word(word));

            }

        }

    }

    Ok(out)

}
