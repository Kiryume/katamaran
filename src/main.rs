mod lexer;
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
    let input =
        std::fs::read_to_string(args.input).map_err(|_| "Failed to read input file".to_string())?;
    let lexer = lexer::LexerCursor::new(&input);
    let tokens = lexer.collect::<Result<Vec<_>, String>>()?;
    println!("{:#?}", tokens);

    Ok(())
}
