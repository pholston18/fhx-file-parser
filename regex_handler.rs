#![allow(unused)]

use crate::lexer::Lexer;
use crate::token::Token;
use crate::token_kind::TokenKind;

use regex::Captures;

pub struct RegexHandler {
    kind: TokenKind,
}

impl RegexHandler {
    pub fn new(kind: TokenKind) -> Self {
        return Self { kind };
    }

    pub fn handle(&self, lex: &mut Lexer, cap: Captures) -> () {
        let handler_function = self.determine_handler();
        handler_function(lex, cap, &self.kind);
    }

    fn determine_handler(&self) -> impl Fn(&mut Lexer, Captures, &TokenKind) -> () {
        return match self.kind {
            TokenKind::Comment => handle_ignored,
            TokenKind::Whitespace => handle_ignored,
            TokenKind::Newline => handle_ignored,
            TokenKind::OpenBracket => handle_character,
            TokenKind::CloseBracket => handle_character,
            TokenKind::OpenCurly => handle_character,
            TokenKind::CloseCurly => handle_character,
            TokenKind::Minus => handle_character,
            TokenKind::Equal => handle_character,
            TokenKind::Integer => handle_number,
            TokenKind::Real => handle_number,
            TokenKind::Text => handle_text,
            TokenKind::Boolean => handle_boolean,
            TokenKind::NamedValue => handle_named_value,
            TokenKind::Keyword => handle_keyword,
            TokenKind::Attribute => handle_attribute,
            _ => panic!("Unknown token kind"),
        };
    }
}

impl std::fmt::Debug for RegexHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RegexHandler {{ Kind: {:?} }}", self.kind,)
    }
}

fn handle_ignored(lex: &mut Lexer, cap: Captures, kind: &TokenKind) -> () {
    lex.cursor = lex.cursor + cap[0].chars().count();
    lex.line = lex.line + cap[0].matches("\n").count();
}

fn handle_character(lex: &mut Lexer, cap: Captures, kind: &TokenKind) -> () {
    lex.cursor = lex.cursor + cap[0].chars().count();

    if *kind == TokenKind::OpenCurly {
        lex.keyword_flag = false
    }

    let text = &cap[0];
    let token = Token::new(kind.clone(), text.to_owned());
    lex.tokens.push(token);
}

fn handle_number(lex: &mut Lexer, cap: Captures, kind: &TokenKind) -> () {
    lex.cursor = lex.cursor + cap[0].chars().count();
    let text = &cap[0];
    let token = Token::new(kind.clone(), text.to_owned());
    lex.tokens.push(token);
}

fn handle_text(lex: &mut Lexer, cap: Captures, kind: &TokenKind) -> () {
    lex.cursor = lex.cursor + cap[0].chars().count();
    let text = &cap[0][1..cap[0].len() - 1];
    let token = Token::new(kind.clone(), text.to_owned());
    lex.tokens.push(token);
}

fn handle_boolean(lex: &mut Lexer, cap: Captures, kind: &TokenKind) -> () {
    lex.cursor = lex.cursor + cap[0].chars().count();
    let text = &cap[0].contains("T").to_string();
    let token = Token::new(kind.clone(), text.to_owned());
    lex.tokens.push(token);
}

fn handle_named_value(lex: &mut Lexer, cap: Captures, kind: &TokenKind) -> () {
    lex.cursor = lex.cursor + cap[0].chars().count();
    let text = &cap[0];
    let token = Token::new(kind.clone(), text.to_owned());
    lex.tokens.push(token);
}

fn handle_keyword(lex: &mut Lexer, cap: Captures, kind: &TokenKind) -> () {
    lex.cursor = lex.cursor + cap[0].chars().count();
    lex.line = lex.line + cap[0].matches("\n").count();
    lex.keyword_flag = true;
    let text = cap[0].trim();
    let token = Token::new(kind.clone(), text.to_owned());
    lex.tokens.push(token);
}

fn handle_attribute(lex: &mut Lexer, cap: Captures, kind: &TokenKind) -> () {
    lex.cursor = lex.cursor + cap[0].chars().count() - 1;
    let text = &cap[0][0..cap[0].len() - 1];

    let token: Token;

    if lex.keyword_flag {
        token = Token::new(kind.clone(), text.to_owned());
    } else {
        token = Token::new(TokenKind::Parameter, text.to_owned());
    }

    lex.tokens.push(token);
}
