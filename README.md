# ToReSy - Token Rewriting System

- [Quickstart](#quickstart)
- [What is a Rewrite System?](#what-is-a-rewrite-system)
- [Data](#data)
- [Rules](#rules)
- [Output / Formatting](#output--formatting)

## Quickstart
`cat test.txt | cargo run -q -- -r rules.sexp -f format.rhai`

This will apply the rules defined in `rules.sexp` to the text in `test.txt` and formats it according to
whats defined in `format.rhai`.

## What is a Rewrite System?
A rewrite system is a piece of software that applies rules to an input in order to rewrite it into an output.

[Wikipedia/Rewriting](https://en.wikipedia.org/wiki/Rewriting)

## Data
The system tokenizes its input and works with those tokens.

These are the token definitions that are used:
```rust
pub enum QuoteStyle {
    Single,
    Double,
}

pub enum Token {
    Identifier(String),
    Number(String),
    Symbol(String),
    OpenParen(String),
    CloseParen(String),
    StringLiteral(String, QuoteStyle),
}
```

## Rules
The rewrite rules are represented as s-expressions. An example can be found in the file [rules.sexp](./rules.sexp).
Each rule consists of a pattern and a replacement part. If the pattern matches, it's replaced with its replacement.
Using a pattern like `AnyNumber` will match any number and bind it to the specified name. This name can be used
in the replacement and the system will automatically insert the number that was bound to that name.

Here are the definitions for the rules:
```rust
pub enum QuoteStylePattern {
    Single,
    Double,
    Any,
}

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

pub struct Rule {
    pattern: Vec<Pattern>,
    replacement: Vec<Token>,
}
```

## Output / Formatting
By default, the tool outputs the rewritten terms in s-expression format. If you want a custom format, you can create
a rhai file in order to format the tokens however you want. For an example you can look at [format.rhai](./format.rhai).