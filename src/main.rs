#![allow(warnings)]
use md_to_html::{Lexer, Node, Parser};

fn main() {
    let input = match std::fs::read_to_string("k.md") {
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

    match out(html, "output.html") {
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
