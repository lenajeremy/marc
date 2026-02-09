use md_to_html::expander::{
    ast::{
        Document, MarcNode, Node,
        expression::{Expression, VariableAccessExpression},
        marcblocks::{ForBlock, IfBlock},
        statement::FunctionDefinitionStatement,
        text_node::TextNode,
    },
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_if_for_require_braced_expressions() {
    let input = "{% for n in m %}\nsum = n * 50\n{% endfor %}\n\n{% if name == \"name\" %}\ntotal_friends = get_friend_count(name)\n{% endif %}";
    let lexer = Lexer::from(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_document();

    let mut expected = Document::new();

    let list_expr = Expression::VariableAccess(VariableAccessExpression::new("m".to_string()));
    let variable = VariableAccessExpression::new("n".to_string());
    let mut for_block = ForBlock::new(list_expr, variable);
    for_block.add_operation(Box::new(MarcNode::Text(TextNode::new("\n".to_string()))));
    for_block.add_operation(Box::new(MarcNode::Text(TextNode::new(
        "sum = n * 50".to_string(),
    ))));
    for_block.add_operation(Box::new(MarcNode::Text(TextNode::new("\n".to_string()))));
    expected.add_node(Box::new(for_block));

    expected.add_node(Box::new(MarcNode::Text(TextNode::new("\n".to_string()))));
    expected.add_node(Box::new(MarcNode::Text(TextNode::new("\n".to_string()))));

    let condition =
        Expression::OperatorInfix(md_to_html::expander::ast::expression::InfixExpression::new(
            Box::new(Expression::VariableAccess(VariableAccessExpression::new(
                "name".to_string(),
            ))),
            Box::new(Expression::String(
                md_to_html::expander::ast::expression::StringExpression::new("name".to_string()),
            )),
            md_to_html::expander::ast::operators::Op::Comp(
                md_to_html::expander::ast::operators::Comparators::Quals,
            ),
        ));
    let mut if_block = IfBlock::new(condition);
    if_block.add_valid_block(Box::new(MarcNode::Text(TextNode::new("\n".to_string()))));
    if_block.add_valid_block(Box::new(MarcNode::Text(TextNode::new(
        "total_friends = get_friend_count(name)".to_string(),
    ))));
    if_block.add_valid_block(Box::new(MarcNode::Text(TextNode::new("\n".to_string()))));

    expected.add_node(Box::new(if_block));

    assert_eq!(program.token_literal(), expected.token_literal());
}

#[test]
fn test_fn_allows_bare_statements() {
    let input = "{% fn add(a, b) %}\n  sum = 50 + a * b\n  return sum\n{% endfn %}";
    let lexer = Lexer::from(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_document();

    let mut expected = Document::new();
    let fn_stmt = FunctionDefinitionStatement::new(
        "add".to_string(),
        vec!["a".to_string(), "b".to_string()],
        vec![Box::new(
            md_to_html::expander::ast::statement::VariableAssignmentStatement::new(
                "sum".to_string(),
                Box::new(Expression::OperatorInfix(
                    md_to_html::expander::ast::expression::InfixExpression::new(
                        Box::new(Expression::Integer(
                            md_to_html::expander::ast::expression::IntegerExpression::new(50),
                        )),
                        Box::new(Expression::OperatorInfix(
                            md_to_html::expander::ast::expression::InfixExpression::new(
                                Box::new(Expression::VariableAccess(
                                    VariableAccessExpression::new("a".to_string()),
                                )),
                                Box::new(Expression::VariableAccess(
                                    VariableAccessExpression::new("b".to_string()),
                                )),
                                md_to_html::expander::ast::operators::Op::Math(
                                    md_to_html::expander::ast::operators::Math::Product,
                                ),
                            ),
                        )),
                        md_to_html::expander::ast::operators::Op::Math(
                            md_to_html::expander::ast::operators::Math::Plus,
                        ),
                    ),
                )),
            ),
        )],
        Some(md_to_html::expander::ast::statement::ReturnStatement::new(
            Box::new(Expression::VariableAccess(VariableAccessExpression::new(
                "sum".to_string(),
            ))),
        )),
    );
    expected.add_node(Box::new(fn_stmt));

    assert_eq!(program.token_literal(), expected.token_literal());
}
