use md_to_html::expander::{
    ast::{
        Document, Node,
        expression::{Expression, InfixExpression, VariableAccessExpression},
        operators::{Math, Op},
        statement::{FunctionDefinitionStatement, ReturnStatement},
    },
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_fn_block() {
    let input = "{% fn add(a, b) %}\nreturn a + b\n{% endfn %}";
    let lexer = Lexer::from(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_document();

    let mut expected_program = Document::new();

    let left = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "a".to_string(),
    )));
    let right = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "b".to_string(),
    )));
    let infix = Box::new(Expression::OperatorInfix(InfixExpression::new(
        left,
        right,
        Op::Math(Math::Plus),
    )));

    let return_stmt = ReturnStatement::new(infix);
    let fn_stmt = FunctionDefinitionStatement::new(
        "add".to_string(),
        vec!["a".to_string(), "b".to_string()],
        vec![],
        Some(return_stmt),
    );

    expected_program.add_node(Box::new(fn_stmt));

    assert_eq!(program.token_literal(), expected_program.token_literal());
}
