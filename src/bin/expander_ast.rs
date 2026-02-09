use md_to_html::expander::{ast::Node, lexer::Lexer, parser::Parser};
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("usage: expander_ast <file>.md");
        std::process::exit(1);
    }

    let in_file = args[1].as_str();
    let input = match std::fs::read_to_string(in_file) {
        Ok(input) => input,
        Err(err) => {
            eprintln!("failed to read input: {err}");
            std::process::exit(1);
        }
    };

    let lexer = Lexer::from(input.as_str());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_document();

    println!("{}", program.translate());
    // println!("{}", program.token_literal());
}
