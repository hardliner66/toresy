use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum QuoteStyle {
    Single,
    Double,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Token {
    Identifier(String),
    Number(String),
    Symbol(String),
    OpenParen(String),
    CloseParen(String),
    StringLiteral(String, QuoteStyle),
}

impl Token {
    pub fn enum_type(&mut self) -> String {
        match self {
            Token::Identifier(_) => "Identifier",
            Token::Number(_) => "Number",
            Token::Symbol(_) => "Symbol",
            Token::OpenParen(_) => "OpenParen",
            Token::CloseParen(_) => "CloseParen",
            Token::StringLiteral(_, _) => "StringLiteral",
        }
        .to_owned()
    }

    pub fn value(&mut self) -> String {
        match self {
            Token::Identifier(s)
            | Token::Number(s)
            | Token::Symbol(s)
            | Token::OpenParen(s)
            | Token::CloseParen(s)
            | Token::StringLiteral(s, _) => s,
        }
        .to_owned()
    }

    pub fn quote_style(&mut self) -> Option<String> {
        match self {
            Token::StringLiteral(_, s) => Some(
                match s {
                    QuoteStyle::Single => "Single",
                    QuoteStyle::Double => "Double",
                }
                .to_owned(),
            ),
            _ => None,
        }
    }
}
