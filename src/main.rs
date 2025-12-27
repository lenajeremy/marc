use md_to_html::marc::{
    ast::{Node, Program},
    lexer::Lexer,
    parser::Parser,
    token::TokenType,
};

use std::env;

fn main() {
    lexer();
    let args: Vec<_> = env::args().collect();

    if args.len() <= 1 {
        panic!("source markdown file not provided. please run marc <file>.md");
    }

    let in_file = args[1].as_str();

    let input = match std::fs::read_to_string(in_file) {
        Ok(input) => input,
        Err(_) => {
            println!("Failed to read from input");
            return;
        }
    };

    let l = Lexer::from(input.as_str());
    let mut p = Parser::new(l);

    let program = p.parse_program();

    let html = program.translate();

    let out_file: Vec<_> = in_file.split(".").collect();
    let out_file = out_file.first().unwrap().to_string() + ".html";

    match out(html, &out_file) {
        Err(x) => {
            println!("failed to write html to file");
            println!("{}", x);
        }
        Ok(_) => {
            println!("html successfully written to file");
        }
    }
}

fn out(text: String, file: &str) -> Result<(), std::io::Error> {
    std::fs::write(file, text)
}

fn lexer() {
    let input = "Hello {{ name.something }}";
    let mut lexer = Lexer::from(input);

    loop {
        let token = lexer.next_token();
        println!("{token:?}");
        if token.token_type == TokenType::EOF {
            break;
        }
    }
}
