use toresy::{Rewriter, Rule, Token, Tokenizer};

use script_format::{FormattingEngine, ScriptResult};

use std::io::Read;
use std::path::PathBuf;

use clap::Parser;

const SEPARATOR: &str = "----------------------------------------";

#[derive(Parser)]
struct Args {
    /// The data to be rewritten
    /// If not provided, the data will be read from stdin
    #[arg(conflicts_with = "input")]
    data: Option<String>,

    /// The rules to be used for rewriting
    #[arg(long, short)]
    rules: PathBuf,

    /// The input file to be rewritten
    #[arg(long, short, conflicts_with = "data")]
    input: Option<PathBuf>,

    /// The output file to write the rewritten data to
    /// If not provided, the rewritten data will be written to stdout
    #[arg(long, short)]
    output: Option<PathBuf>,

    /// The script to format the rewritten data
    #[arg(long, short)]
    format: Option<PathBuf>,

    /// Enable debug mode for formatting
    #[arg(long, short)]
    debug: bool,

    /// Enable verbose mode
    #[arg(long, short)]
    verbose: bool,
}

fn main() {
    let Args {
        rules,
        input,
        data,
        output,
        format,
        debug,
        verbose,
    } = Args::parse();

    let rules = std::fs::read_to_string(rules).unwrap();

    let rules: Vec<Rule> = serde_lexpr::from_str(&rules).unwrap();
    if verbose {
        eprintln!("Rules:");
        eprintln!("{}", serde_yaml::to_string(&rules).unwrap());
    }

    let rewriter = Rewriter::new(rules);

    let input = data.unwrap_or_else(|| {
        input
            .map(|p| std::fs::read_to_string(p).unwrap())
            .unwrap_or_else(|| {
                let mut buf = Vec::new();
                std::io::stdin().read_to_end(&mut buf).unwrap();
                String::from_utf8(buf).unwrap()
            })
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

    let formatted = if let Some(ref format) = format {
        let mut engine = FormattingEngine::new(debug);

        engine
            .register_type::<Token>()
            .register_get("enum_type", Token::enum_type)
            .register_get("value", Token::value)
            .register_get("quote_style", |t: &mut Token| -> ScriptResult<String> {
                Token::quote_style(t).ok_or("no quote style".into())
            });

        engine
            .format_from_file("tokens", rewritten_tokens, format)
            .unwrap()
            .trim()
            .to_owned()
    } else {
        serde_lexpr::to_string(&rewritten_tokens)
            .unwrap()
            .trim()
            .to_owned()
    };

    if verbose {
        eprintln!();
        eprintln!("{SEPARATOR}");
        eprintln!();
    }

    match output {
        Some(ref output) => {
            std::fs::write(output, formatted).unwrap();
        }
        None => {
            println!("{formatted}");
        }
    }
}
