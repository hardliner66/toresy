mod formatting;

use toresy::{Rewriter, Rule, Token, Tokenizer};

use std::path::PathBuf;
use std::rc::Rc;
use std::{cell::RefCell, io::Read};

use clap::Parser;
use formatting::build_engine;
use rhai::Scope;

const SEPARATOR: &str = "----------------------------------------";

#[derive(Parser)]
struct Args {
    input: Option<String>,

    #[arg(long, short)]
    rules: PathBuf,

    #[arg(long, short)]
    output: Option<PathBuf>,

    #[arg(long, short)]
    format: Option<PathBuf>,

    #[arg(long, short)]
    debug: bool,

    #[arg(long, short)]
    verbose: bool,
}

fn main() {
    let Args {
        rules,
        input,
        output,
        format,
        debug,
        verbose,
    } = Args::parse();

    let messages = Rc::new(RefCell::new(Vec::new()));

    let rules = std::fs::read_to_string(rules).unwrap();

    let rules: Vec<Rule> = serde_lexpr::from_str(&rules).unwrap();
    if verbose {
        eprintln!("Rules:");
        eprintln!("{}", serde_yaml::to_string(&rules).unwrap());
    }

    let rewriter = Rewriter::new(rules);

    let input = input.unwrap_or_else(|| {
        let mut buf = Vec::new();
        std::io::stdin().read_to_end(&mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    });

    let mut tokenizer = Tokenizer::new(&input);
    let mut tokens = Vec::new();

    while let Some(token) = tokenizer.next_token() {
        tokens.push(token);
    }

    if verbose {
        eprintln!();
        eprintln!("{SEPARATOR}");
        eprintln!();
        eprintln!("Tokens:");
        eprintln!("{}", serde_yaml::to_string(&tokens).unwrap().trim());
    }
    let rewritten_tokens = rewriter.rewrite(tokens);
    if verbose {
        eprintln!();
        eprintln!("{SEPARATOR}");
        eprintln!();
        eprintln!("Rewritten Tokens:");
        eprintln!(
            "{}",
            serde_yaml::to_string(&rewritten_tokens).unwrap().trim()
        );
    }

    if let Some(ref format) = format {
        let script = std::fs::read_to_string(format).unwrap();
        let mut scope = Scope::new();
        scope.push_constant("tokens", rewritten_tokens);
        scope.push_constant("NL", "\n");
        scope.push_constant("Identifier", "Identifier");
        scope.push_constant("Number", "Number");
        scope.push_constant("Symbol", "Symbol");
        scope.push_constant("OpenParen", "OpenParen");
        scope.push_constant("CloseParen", "CloseParen");
        scope.push_constant("StringLiteral", "StringLiteral");
        let engine = build_engine(messages.clone(), debug);
        engine.run_with_scope(&mut scope, &script).unwrap();
    } else {
        let mut messages = messages.borrow_mut();
        messages.clear();
        messages.push(
            serde_lexpr::to_string(&rewritten_tokens)
                .unwrap()
                .trim()
                .to_owned(),
        );
    }

    if verbose {
        eprintln!();
        eprintln!("{SEPARATOR}");
        eprintln!();
    }

    let text = messages.borrow().join("");

    match output {
        Some(ref output) => {
            std::fs::write(output, text).unwrap();
        }
        None => {
            println!("{text}");
        }
    }
}
