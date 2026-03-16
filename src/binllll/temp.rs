use md_to_html::expander::{
    ast::Node, environment::Environment, lexer::Lexer, object::Object, parser::Parser,
    token::TokenType,
};

fn main() {
    let input = "50 * 350 * 5\n";

    let mut env = Environment::new();

    let mut lexer = Lexer::from(input);
    lexer.set_detailed(true);

    loop {
        let curr = lexer.next_token();
        println!("{:?}", curr);
        if curr.token_type == TokenType::EOF {
            break;
        }
    }

    // let mut parser = Parser::new(lexer);
    // let program = parser.parse_document();
    //
    // let out_file = "output/temp.md";
    // std::fs::write(out_file, program.translate(&mut env)).unwrap();
    // std::fs::write(out_file, program.token_literal()).unwrap();
    // println!("{}", program.token_literal())
}
