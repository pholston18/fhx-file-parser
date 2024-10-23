use regex::Regex;

use crate::regex_handler::RegexHandler;
use crate::token_kind::TokenKind;

pub struct RegexPattern {
    pub regex: Regex,
    pub kind: TokenKind,
    pub handler: RegexHandler,
}

impl RegexPattern {
    pub fn new(regex: Regex, kind: TokenKind) -> Self {
        return Self {
            regex,
            kind: kind.clone(),
            handler: RegexHandler::new(kind),
        };
    }
}

impl std::fmt::Debug for RegexPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RegexPattern {{ regex: {:?}, Kind: {:?} }}",
            self.regex, self.kind,
        )
    }
}
