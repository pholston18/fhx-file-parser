#[allow(unused)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenKind {
    // Ignored Tokens
    Comment,
    Whitespace,
    Newline,

    // Symbols
    OpenBracket,
    CloseBracket,
    OpenCurly,
    CloseCurly,
    Minus,
    Equal,

    // Values
    Integer,
    Real,
    Text,
    Boolean,
    NamedValue,

    // Keywords
    Keyword,
    Attribute,
    Parameter,

    // End of File
    EOF,
}

impl TokenKind {
    pub fn is_value_kind(&self) -> bool {
        match self {
            TokenKind::Integer => true,
            TokenKind::Real => true,
            TokenKind::Text => true,
            TokenKind::Boolean => true,
            TokenKind::NamedValue => true,
            _ => false,
        }
    }

    pub fn is_string_kind(&self) -> bool {
        match self {
            TokenKind::OpenBracket => true,
            TokenKind::CloseBracket => true,
            TokenKind::OpenCurly => true,
            TokenKind::CloseCurly => true,
            TokenKind::Minus => true,
            TokenKind::Equal => true,
            TokenKind::Text => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
