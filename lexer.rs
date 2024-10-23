use crate::regex_pattern::RegexPattern;
use crate::token::Token;
use crate::token_kind::TokenKind;

use regex::Regex;

#[derive(Debug)]
pub struct Lexer {
    pub cursor: usize,
    pub line: usize,
    pub tokens: Vec<Token>,
    pub keyword_flag: bool,
}

impl Lexer {
    pub fn new() -> Self {
        return Self {
            cursor: 0,
            line: 1,
            tokens: Vec::new(),
            keyword_flag: false,
        };
    }

    pub fn lex(&mut self, source: String) -> Vec<Token> {
        let patterns = intialize_patterns();

        while self.cursor < source.len() {
            let mut found_match = false;

            for pattern in &patterns {
                let text = &source[self.cursor..];

                if let Some(cap) = pattern.regex.captures(text) {
                    pattern.handler.handle(self, cap);
                    found_match = true;
                    break;
                }
            }

            if !found_match {
                panic!("Unrecognized token: {}", &source[self.cursor..]);
            }
        }

        self.tokens.push(Token::new(TokenKind::EOF, "".to_string()));
        self.line = self.line + 1;

        return self.tokens.clone();
    }
}

impl std::fmt::Display for Lexer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Lexer {{ Length of Token Array: {} }}",
            self.tokens.len()
        )
    }
}

fn intialize_patterns() -> Vec<RegexPattern> {
    return vec![
        RegexPattern::new(Regex::new(r#"^\s+"#).unwrap(), TokenKind::Whitespace),
        RegexPattern::new(Regex::new(r#"^\n+"#).unwrap(), TokenKind::Newline),
        RegexPattern::new(Regex::new(r#"^\["#).unwrap(), TokenKind::OpenBracket),
        RegexPattern::new(Regex::new(r#"^\]"#).unwrap(), TokenKind::CloseBracket),
        RegexPattern::new(Regex::new(r#"^\{"#).unwrap(), TokenKind::OpenCurly),
        RegexPattern::new(Regex::new(r#"^\}"#).unwrap(), TokenKind::CloseCurly),
        RegexPattern::new(Regex::new(r#"^\-"#).unwrap(), TokenKind::Minus),
        RegexPattern::new(Regex::new(r#"^\="#).unwrap(), TokenKind::Equal),
        RegexPattern::new(
            Regex::new(r#"^/\*(.|[\r\n])*?\*/"#).unwrap(),
            TokenKind::Comment,
        ),
        RegexPattern::new(
            Regex::new(r#"^([0-9]+)?+\.[0-9]+"#).unwrap(),
            TokenKind::Real,
        ),
        RegexPattern::new(Regex::new(r#"^[0-9]+"#).unwrap(), TokenKind::Integer),
        RegexPattern::new(Regex::new(r#"^"[^"]*""#).unwrap(), TokenKind::Text),
        RegexPattern::new(Regex::new(r#"^(F|T)+(\n|\s)"#).unwrap(), TokenKind::Boolean),
        RegexPattern::new(Regex::new(r#"^[\w]+(\n|\s)+"#).unwrap(), TokenKind::Keyword),
        RegexPattern::new(Regex::new(r#"^[\w]+[=]"#).unwrap(), TokenKind::Attribute),
        RegexPattern::new(Regex::new(r#"^[\w]+"#).unwrap(), TokenKind::NamedValue),
    ];
}
