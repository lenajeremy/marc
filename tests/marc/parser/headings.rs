use md_to_html::marc::{
    ast::{
        Node, Program,
        heading::Heading,
        inline_container::InlineContainer,
        text::{BoldText, Text},
    },
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn parse_heading_1() {
    let input = "# Hello World\n## Hello World 2";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);

    let mut expected_program = Program::new();
    let mut heading_block = Box::new(Heading::new(1));
    let mut heading_2_block = Box::new(Heading::new(2));

    let mut inner_1 = Box::new(InlineContainer::new());
    let mut inner_2 = Box::new(InlineContainer::new());

    inner_1.add_child(Box::new(Text::new(" Hello World".to_string())));
    inner_2.add_child(Box::new(Text::new(" Hello World 2".to_string())));

    heading_block.set_inner(inner_1);
    heading_2_block.set_inner(inner_2);

    expected_program.add_block(heading_block);
    expected_program.add_block(heading_2_block);

    let parsed_program = p.parse_program();

    println!("Got: {}", parsed_program.token_literal());
    println!("Expected: {}", expected_program.token_literal());

    assert_eq!(
        parsed_program.token_literal(),
        expected_program.token_literal()
    );
}

#[test]
fn parses_heading_with_inline_elements() {
    let input = "# Hello **World**";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);

    let mut expected_program = Program::new();

    let mut heading_block = Box::new(Heading::new(1));
    let mut inner = Box::new(InlineContainer::new());
    inner.add_child(Box::new(Text::new(" Hello ".to_string())));

    let mut bold_text = Box::new(BoldText::new());
    let mut bold_inner = Box::new(InlineContainer::new());
    bold_inner.add_child(Box::new(Text::new("World".to_string())));
    bold_text.set_inner(bold_inner);

    inner.add_child(bold_text);

    heading_block.set_inner(inner);

    expected_program.add_block(heading_block);
    let parsed_program = p.parse_program();

    println!("{}", parsed_program.token_literal());
    println!("{}", expected_program.token_literal());

    assert_eq!(
        parsed_program.token_literal(),
        expected_program.token_literal()
    );
}
