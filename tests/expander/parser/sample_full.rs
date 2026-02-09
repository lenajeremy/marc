use md_to_html::expander::{ast::Node, lexer::Lexer, parser::Parser};

#[test]
fn test_expander_sample_full() {
    let input = include_str!("../../../samples/expander_sample.md");
    let lexer = Lexer::from(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_document();

    assert!(!program.token_literal().is_empty());
}
