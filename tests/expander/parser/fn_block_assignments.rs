use md_to_html::expander::{
    ast::{
        Document, Node,
        expression::{Expression, InfixExpression, IntegerExpression, VariableAccessExpression},
        operators::{Math, Op},
        statement::{FunctionDefinitionStatement, ReturnStatement, VariableAssignmentStatement},
    },
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_fn_block_with_assignment_and_return() {
    let input = "{% fn add(a, b) %}\n    sum = 50 + a * b\n    {% return sum %}\n    {% endfn %}";
    let lexer = Lexer::from(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_document();

    let mut expected_program = Document::new();

    let rhs = Box::new(Expression::OperatorInfix(InfixExpression::new(
        Box::new(Expression::Integer(IntegerExpression::new(50))),
        Box::new(Expression::OperatorInfix(InfixExpression::new(
            Box::new(Expression::VariableAccess(VariableAccessExpression::new(
                "a".to_string(),
            ))),
            Box::new(Expression::VariableAccess(VariableAccessExpression::new(
                "b".to_string(),
            ))),
            Op::Math(Math::Product),
        ))),
        Op::Math(Math::Plus),
    )));

    let assign = VariableAssignmentStatement::new("sum".to_string(), rhs);
    let body: Vec<Box<dyn Node>> = vec![Box::new(assign)];

    let return_stmt = ReturnStatement::new(Box::new(Expression::VariableAccess(
        VariableAccessExpression::new("sum".to_string()),
    )));

    let fn_stmt = FunctionDefinitionStatement::new(
        "add".to_string(),
        vec!["a".to_string(), "b".to_string()],
        body,
        Some(return_stmt),
    );

    expected_program.add_node(Box::new(fn_stmt));

    assert_eq!(program.token_literal(), expected_program.token_literal());
}
