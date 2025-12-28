use md_to_html::expander::{
    ast::{
        Document, Node,
        expression::{
            Expression, InfixExpression, IntegerExpression, PrefixExpression,
            VariableAccessExpression,
        },
        operators::{Math, Op},
    },
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_complex_expressions() {
    // -a * b
    let input = "{{ -a * b }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();

    // -a
    let a_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "a".to_string(),
    )));
    let prefix_minus_a = Box::new(Expression::Prefix(PrefixExpression::new(
        Op::Math(Math::Minus),
        a_var,
    )));

    // b
    let b_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "b".to_string(),
    )));

    // (-a) * b
    let infix = Box::new(Expression::OperatorInfix(InfixExpression::new(
        prefix_minus_a,
        b_var,
        Op::Math(Math::Product),
    )));

    expected_program.add_node(infix);

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for input: {}",
        input
    );
}

#[test]
fn test_operator_precedence() {
    // a + b * c  -> a + (b * c)
    let input = "{{ a - b - c }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();

    let a_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "a".to_string(),
    )));
    let b_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "b".to_string(),
    )));
    let c_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "c".to_string(),
    )));

    // a - b
    let a_minus_b = Box::new(Expression::OperatorInfix(InfixExpression::new(
        a_var,
        b_var,
        Op::Math(Math::Minus),
    )));

    // (a - b) - c
    let a_minus_b_minus_c = Box::new(Expression::OperatorInfix(InfixExpression::new(
        a_minus_b,
        c_var,
        Op::Math(Math::Minus),
    )));

    expected_program.add_node(a_minus_b_minus_c);

    println!("{}", program.token_literal());
    println!("{}", expected_program.token_literal());

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for precedence test"
    );
}
