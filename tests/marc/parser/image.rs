use md_to_html::marc::{
    ast::{Node, Program, image::Image},
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn parse_image() {
    let input = "![this is the alt](https://google.com)";
    let lexer = Lexer::from(input);
    let mut parser = Parser::new(lexer);

    let image = Box::new(Image::new(
        "https://google.com".to_string(),
        "this is the alt".to_string(),
    ));

    let mut expected_program = Program::new();
    expected_program.add_block(image);

    let program = parser.parse_program();

    println!(
        "Got: {}, Expected: {}",
        program.token_literal(),
        expected_program.token_literal()
    );

    assert_eq!(program.token_literal(), expected_program.token_literal())
}
