#![allow(unused)]

mod lexer;
mod regex_handler;
mod regex_pattern;
mod token;
mod token_kind;

// use crate::token::Token;
// use crate::token_kind::TokenKind;

// use minidom::Element;
use regex::Regex;

fn main() {
    // let kind = TokenKind::Text;
    // let comment = String::from("86");

    // let token = Token::new(kind, comment);
    // let other_token = token.clone();

    // println!("{}", token);
    // println!("{}", other_token);

    let re = Regex::new(r#"\nhello\n\n"#).unwrap();

    let text = "\nhello\n\nmacy's day parade";

    let capture = match re.captures(text) {
        Some(capture) => capture[0].to_owned(),
        None => panic!("No match found"),
    };

    // println!("{:?}", capture);

    // let element = Element::builder("ROOT", "").build();

    // println!("{:?}", element);

    let text = r#"/* Version: 10.2.0.5598.fh */
        /* "30-Jul-2024 09:47:35" */

        SCHEMA
         user="J984FG56" time=1722350796/* "30-Jul-2024 09:46:36" */
        {
          /* Database last updated on "29-Jul-2019 20:07:54" */
          VERSION=1564448874/* "29-Jul-2019 20:07:54" */
          MAJOR_VERSION=10
          MINOR_VERSION=2
          MAINTENANCE_VERSION=0
          BUILD_VERSION=5598
          BUILD_ID="fh"
          VERSION_STR="10.2.0.5598.fh"
          ONLINE_UPGRADE=F
        }"#;

    // let line: &str;

    // let mut lines = text.lines();

    // while let Some(line) = lines.next() {
    //     println!("{}", line);
    // }

    let text = String::from(text);
    let mut lexer = lexer::Lexer::new();
    let tokens = lexer.lex(text);

    // println!("{:?}", tokens);
    // println!("{:?}", lexer.line)
}
