use md_to_html::marc::{
    ast::{Node, Program, link::Link},
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn parse_link() {
    let input = "\
[this is the alt](https://google.com)
[hello world](https://jeremiah.vercel.app)";
    let lexer = Lexer::from(input);
    let mut parser = Parser::new(lexer);

    let link1 = Box::new(Link::new(
        "this is the alt".to_string(),
        "https://google.com".to_string(),
    ));

    let link2 = Box::new(Link::new(
        "hello world".to_string(),
        "https://jeremiah.vercel.app".to_string(),
    ));

    let mut expected_program = Program::new();
    expected_program.add_block(link1);
    expected_program.add_block(link2);

    let program = parser.parse_program();

    println!(
        "Got: {} \n Expected: {}",
        program.token_literal(),
        expected_program.token_literal()
    );

    assert_eq!(program.token_literal(), expected_program.token_literal())
}
