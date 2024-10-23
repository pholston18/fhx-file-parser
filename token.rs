use crate::token_kind::TokenKind;

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    value: String,
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Self {
        return Self { kind, value };
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.kind.is_string_kind() {
            write!(
                f,
                "Token {{ kind: TokenKind::{}, value: \"{}\" }}",
                self.kind, self.value
            )
        } else {
            write!(
                f,
                "Token {{ kind: TokenKind::{}, value: {} }}",
                self.kind, self.value
            )
        }
    }
}
