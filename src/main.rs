mod lexer;
mod parser;
mod tokentree;
use clap::Parser;
use parser::Statement;

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
    let input =
        std::fs::read_to_string(args.input).map_err(|_| "Failed to read input file".to_string())?;
    let lexer = lexer::LexerCursor::new(&input);
    let tokens = lexer.collect::<Result<Vec<_>, String>>()?;
    let tokenstream = tokentree::TokenTree::parse_from_tokens(&mut tokens.into_iter())?;
    let mut parser = parser::Parser::new(tokenstream);
    let ast: Vec<Statement> = parser.parse_statements().unwrap();
    parser.errors.iter().for_each(|e| println!("{}", e));
    println!("{:#?}", ast);

    Ok(())
}
