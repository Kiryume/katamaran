mod lexer;
mod tokentree;
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Input file
    input: String,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = Args::parse();
    let input = std::fs::read_to_string(args.input).expect("Failed to read input file");
    let mut lexer = lexer::LexerCursor::new(&input);
    let mut tokens = lexer.collect::<Result<Vec<_>, String>>()?;
    let tokentrees = tokentree::TokenTree::parse_from_tokens(&mut tokens.into_iter())?;
    println!("{:#?}", tokentrees);

    Ok(())
}
