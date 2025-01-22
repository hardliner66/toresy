use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use serde::{Deserialize, Serialize};
use stringlit::s;

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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum QuoteStylePattern {
    Single,
    Double,
    Any,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Identifier(String),                   // Matches a specific identifier
    Number(String),                       // Matches a specific number
    AnyIdentifier(String),                // Matches any identifier and binds it
    AnyNumber(String),                    // Matches any number and binds it
    Symbol(String),                       // Matches a specific symbol
    OpenParen(String),                    // Matches an open parenthesis
    CloseParen(String),                   // Matches a close parenthesis
    String(String, QuoteStylePattern),    // Matches a specific string
    AnyString(String, QuoteStylePattern), // Matches any string and binds it
    Any,                                  // Matches any single token
}

pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    #[must_use]
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(&ch) = self.input.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    self.input.next(); // Skip whitespace
                }
                '(' => {
                    self.input.next();
                    return Some(Token::OpenParen(s!("(")));
                }
                ')' => {
                    self.input.next();
                    return Some(Token::CloseParen(s!(")")));
                }
                '0'..='9' => {
                    return Some(self.consume_number());
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    return Some(self.consume_identifier());
                }
                '\'' => {
                    return Some(self.consume_string(QuoteStyle::Single));
                }
                '"' => {
                    return Some(self.consume_string(QuoteStyle::Double));
                }
                _ => {
                    return Some(self.consume_symbol());
                }
            }
        }
        None
    }

    fn consume_number(&mut self) -> Token {
        let mut number = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch.is_numeric() {
                number.push(ch);
                self.input.next();
            } else {
                break;
            }
        }
        Token::Number(number)
    }

    fn consume_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.input.next();
            } else {
                break;
            }
        }
        Token::Identifier(identifier)
    }

    fn consume_string(&mut self, quote_style: QuoteStyle) -> Token {
        let quote = self.input.next().unwrap(); // Consume the opening quote
        let mut string_content = String::new();

        while let Some(&ch) = self.input.peek() {
            if ch == quote {
                self.input.next(); // Consume the closing quote
                return Token::StringLiteral(string_content, quote_style);
            }
            string_content.push(ch);
            self.input.next();
        }

        panic!("Unterminated string literal");
    }

    fn consume_symbol(&mut self) -> Token {
        let mut symbol = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch.is_alphanumeric()
                || ch.is_whitespace()
                || ch == '('
                || ch == ')'
                || ch == '\''
                || ch == '"'
            {
                break;
            }
            symbol.push(ch);
            self.input.next();
        }
        Token::Symbol(symbol)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pattern: Vec<Pattern>,
    replacement: Vec<Token>,
}

impl Rule {
    #[must_use]
    pub fn new(pattern: Vec<Pattern>, replacement: Vec<Token>) -> Self {
        Self {
            pattern,
            replacement,
        }
    }

    fn will_match(&self, tokens: &[Token]) -> bool {
        for (pattern, token) in self.pattern.iter().zip(tokens) {
            match (pattern, token) {
                (Pattern::Identifier(p), Token::Identifier(t))
                | (Pattern::Number(p), Token::Number(t))
                | (Pattern::Symbol(p), Token::Symbol(t))
                | (Pattern::OpenParen(p), Token::OpenParen(t))
                | (Pattern::CloseParen(p), Token::CloseParen(t))
                | (
                    Pattern::String(p, QuoteStylePattern::Double),
                    Token::StringLiteral(t, QuoteStyle::Double),
                )
                | (
                    Pattern::String(p, QuoteStylePattern::Single),
                    Token::StringLiteral(t, QuoteStyle::Single),
                )
                | (Pattern::String(p, QuoteStylePattern::Any), Token::StringLiteral(t, _))
                    if p == t || p == "*" => {}
                (Pattern::AnyIdentifier(_), Token::Identifier(_))
                | (Pattern::AnyNumber(_), Token::Number(_))
                | (
                    Pattern::AnyString(_, QuoteStylePattern::Double),
                    Token::StringLiteral(_, QuoteStyle::Double),
                )
                | (
                    Pattern::AnyString(_, QuoteStylePattern::Single),
                    Token::StringLiteral(_, QuoteStyle::Single),
                )
                | (Pattern::AnyString(_, QuoteStylePattern::Any), Token::StringLiteral(_, _))
                | (Pattern::Any, _) => {}
                _ => return false,
            }
        }

        true
    }

    fn matches(&self, tokens: &[Token]) -> Vec<(HashMap<String, Token>, usize)> {
        let mut result = Vec::new();

        if tokens.len() < self.pattern.len() {
            return result;
        }

        let diff = tokens.len() - self.pattern.len() + 1;
        for i in 0..=diff {
            if !self.will_match(&tokens[i..]) {
                continue;
            }
            let mut bindings = HashMap::new();
            for (pattern, token) in self.pattern.iter().zip(tokens[i..].iter()) {
                match (pattern, token) {
                    (Pattern::Identifier(p), Token::Identifier(t)) if p == t => {}
                    (Pattern::Number(p), Token::Number(t)) if p == t => {}
                    (Pattern::AnyIdentifier(name), Token::Identifier(t)) => {
                        bindings.insert(name.clone(), Token::Identifier(t.clone()));
                    }
                    (Pattern::AnyNumber(name), Token::Number(t)) => {
                        bindings.insert(name.clone(), Token::Number(t.clone()));
                    }
                    (Pattern::Symbol(p), Token::Symbol(t)) if p == t => {}
                    (Pattern::OpenParen(pp), Token::OpenParen(pt)) if pp == pt || pp == "*" => {}
                    (Pattern::CloseParen(pp), Token::CloseParen(pt)) if pp == pt || pp == "*" => {}
                    (
                        Pattern::String(ps, QuoteStylePattern::Double),
                        Token::StringLiteral(ts, QuoteStyle::Double),
                    ) if ps == ts => {}
                    (
                        Pattern::String(ps, QuoteStylePattern::Single),
                        Token::StringLiteral(ts, QuoteStyle::Single),
                    ) if ps == ts => {}
                    (Pattern::String(ps, QuoteStylePattern::Any), Token::StringLiteral(ts, _))
                        if ps == ts => {}
                    (
                        Pattern::AnyString(name, QuoteStylePattern::Double),
                        Token::StringLiteral(t, QuoteStyle::Double),
                    ) => {
                        bindings.insert(
                            name.clone(),
                            Token::StringLiteral(t.clone(), QuoteStyle::Double),
                        );
                    }
                    (
                        Pattern::AnyString(name, QuoteStylePattern::Single),
                        Token::StringLiteral(t, QuoteStyle::Single),
                    ) => {
                        bindings.insert(
                            name.clone(),
                            Token::StringLiteral(t.clone(), QuoteStyle::Single),
                        );
                    }
                    (
                        Pattern::AnyString(name, QuoteStylePattern::Any),
                        Token::StringLiteral(t, sl),
                    ) => {
                        bindings.insert(name.clone(), Token::StringLiteral(t.clone(), sl.clone()));
                    }
                    (Pattern::Any, t) => {
                        bindings.insert(s!("_"), t.clone());
                    }
                    _ => {}
                }
            }
            result.push((bindings, i));
        }
        result
    }

    fn apply(&self, bindings: &HashMap<String, Token>) -> Vec<Token> {
        self.replacement
            .iter()
            .map(|token| match token {
                Token::Identifier(id) => bindings.get(id).cloned().unwrap_or_else(|| token.clone()),
                Token::Number(num) => bindings.get(num).cloned().unwrap_or_else(|| token.clone()),
                Token::Symbol(sym) => bindings.get(sym).cloned().unwrap_or_else(|| token.clone()),
                Token::StringLiteral(str, _) => {
                    bindings.get(str).cloned().unwrap_or_else(|| token.clone())
                }
                _ => token.clone(),
            })
            .collect()
    }
}

pub struct Rewriter {
    rules: Vec<Rule>,
}

impl Rewriter {
    #[must_use]
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn rewrite(&self, mut tokens: Vec<Token>) -> Vec<Token> {
        for rule in &self.rules {
            let mut offset: i128 = 0;
            for (bindings, match_pos) in rule.matches(&tokens) {
                // Adjust the position based on the current offset
                let adjusted_pos = usize::try_from(match_pos as i128 + offset).unwrap();

                // Ensure the position is valid
                if adjusted_pos + rule.pattern.len() > tokens.len() {
                    continue;
                }

                // Remove the matched tokens
                tokens.drain(adjusted_pos..adjusted_pos + rule.pattern.len());

                // Insert the replacement tokens
                let replacement = rule.apply(&bindings);
                for (i, token) in replacement.iter().enumerate() {
                    tokens.insert(adjusted_pos + i, token.clone());
                }

                // Update the offset based on the size difference
                offset += replacement.len() as i128 - rule.pattern.len() as i128;
            }
        }
        tokens
    }
}
