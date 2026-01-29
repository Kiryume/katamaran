mod ast;
mod lexer;
mod parser;
mod types;

use clap::Parser;
use lexer::types::LexerError;
use miette::{IntoDiagnostic, NamedSource, Report, Result};

#[derive(Parser)]
struct Args {
    /// Input file
    input: String,
}

fn main() {
    if let Err(report) = run() {
        eprintln!("{report:?}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();
    let input = std::fs::read_to_string(&args.input).into_diagnostic()?;
    let lexer = lexer::LexerCursor::new(&input);
    // lexer.collect::<Result<Vec<_>, LexerError>>().map_err(|e| {
    //     let src = NamedSource::new(args.input.clone(), input.clone());
    //     Report::new(e).with_source_code(src)
    // })?;
    Ok(())
}
