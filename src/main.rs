use md_to_html::{Lexer, Node, Parser};

fn main() {
    let input = "\
    # Heading 1

    Normal text **bold** *italic* `code` ![img](url) [link](url)
    
    # Heading 1 My name is **JEREMIAH** LEnA OSAIGBOKAN\n## Yoooooooo";
    let l = Lexer::from(input);
    let mut p = Parser::new(l);

    let program = p.parse_program();
    println!("Program: {}", program.token_literal());
}
