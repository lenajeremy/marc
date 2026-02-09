use md_to_html::expander::{
    ast::{
        Document, MarcNode, Node,
        expression::{Expression, VariableAccessExpression},
        marcblocks::ForBlock,
        text_node::TextNode,
    },
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_for_block() {
    let input = "{% for item in items %}Hello {{ item }}{% endfor %}";
    let lexer = Lexer::from(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_document();

    let mut expected_program = Document::new();

    let list_expr = Expression::VariableAccess(VariableAccessExpression::new("items".to_string()));
    let variable = VariableAccessExpression::new("item".to_string());
    let mut for_block = ForBlock::new(list_expr, variable);

    for_block.add_operation(Box::new(MarcNode::Text(TextNode::new(
        "Hello ".to_string(),
    ))));
    for_block.add_operation(Box::new(MarcNode::Expression(Box::new(
        Expression::VariableAccess(VariableAccessExpression::new("item".to_string())),
    ))));

    expected_program.add_node(Box::new(for_block));

    assert_eq!(program.token_literal(), expected_program.token_literal());
}
