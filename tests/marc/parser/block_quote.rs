use md_to_html::marc::{
    ast::{Node, Program, block_quote::BlockQuote, inline_container::InlineContainer, text::Text},
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_parses_block_quote_valid() {
    let input = "> Hello World";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);

    let mut expected_program = Program::new();

    let mut block_quote = Box::new(BlockQuote::new());
    let mut block_quote_inner = Box::new(InlineContainer::new());
    block_quote_inner.add_child(Box::new(Text::new(" Hello World".to_string())));

    block_quote.set_inner(block_quote_inner);

    expected_program.add_block(block_quote);

    let parsed_program = p.parse_program();

    assert_eq!(
        parsed_program.token_literal(),
        expected_program.token_literal()
    );
}

#[test]
fn test_parses_block_quote_with_middle_sign() {
    let input = "> Hello > World";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);

    let mut expected_program = Program::new();

    let mut block_quote = Box::new(BlockQuote::new());
    let mut block_quote_inner = Box::new(InlineContainer::new());
    block_quote_inner.add_child(Box::new(Text::new(" Hello > World".to_string())));

    block_quote.set_inner(block_quote_inner);

    expected_program.add_block(block_quote);

    let parsed_program = p.parse_program();

    assert_eq!(
        parsed_program.token_literal(),
        expected_program.token_literal()
    );
}
