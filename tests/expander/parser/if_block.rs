use md_to_html::expander::{
    ast::{
        Document, Node,
        expression::{Expression, InfixExpression, StringExpression, VariableAccessExpression},
        marcblocks::IfBlock,
        operators::{Comparators, Op},
    },
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_if_block() {
    let input = "{% if name == \"Jeremiah\" %}Hello{% endif %}";
    let lexer = Lexer::from(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_document();

    let mut expected_program = Document::new();

    let left = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "name".to_string(),
    )));
    let right = Box::new(Expression::String(StringExpression::new(
        "Jeremiah".to_string(),
    )));
    let condition = Expression::OperatorInfix(InfixExpression::new(
        left,
        right,
        Op::Comp(Comparators::Quals),
    ));

    let mut if_block = IfBlock::new(condition);
    if_block.add_valid_block(Box::new(md_to_html::expander::ast::MarcNode::Text(
        md_to_html::expander::ast::text_node::TextNode::new("Hello".to_string()),
    )));

    expected_program.add_node(Box::new(if_block));

    assert_eq!(program.token_literal(), expected_program.token_literal());
}
