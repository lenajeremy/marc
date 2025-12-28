use md_to_html::expander::{
    ast::{
        Document, Node,
        expression::{Expression, InfixExpression, IntegerExpression},
        operators::{Math, Op},
    },
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_infix_operators() {
    let inputs = vec![
        ("{{ 5 + 5 }}", 5, Op::Math(Math::Plus), 5),
        ("{{ 5 - 5 }}", 5, Op::Math(Math::Minus), 5),
        ("{{ 5 * 5 }}", 5, Op::Math(Math::Product), 5),
        ("{{ 5 / 5 }}", 5, Op::Math(Math::Divide), 5),
    ];

    for (input, left_val, op, right_val) in inputs {
        let lexer = Lexer::from(input);
        let mut p = Parser::new(lexer);
        let program = p.parse_document();

        let mut expected_program = Document::new();
        let left = Box::new(Expression::Integer(IntegerExpression::new(left_val)));
        let right = Box::new(Expression::Integer(IntegerExpression::new(right_val)));
        let infix = Box::new(Expression::OperatorInfix(InfixExpression::new(
            left, right, op,
        )));
        expected_program.add_node(infix);

        assert_eq!(
            program.token_literal(),
            expected_program.token_literal(),
            "failed for input: {}",
            input
        );
    }
}

#[test]
fn test_chained_infix_operators() {
    // 1 + 2 + 3
    let input = "{{ 1 + 2 + 3 }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();

    // 1 + 2
    let one_plus_two = Box::new(Expression::OperatorInfix(InfixExpression::new(
        Box::new(Expression::Integer(IntegerExpression::new(1))),
        Box::new(Expression::Integer(IntegerExpression::new(2))),
        Op::Math(Math::Plus),
    )));

    // (1 + 2) + 3
    let result = Box::new(Expression::OperatorInfix(InfixExpression::new(
        one_plus_two,
        Box::new(Expression::Integer(IntegerExpression::new(3))),
        Op::Math(Math::Plus),
    )));

    expected_program.add_node(result);

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for chained infix operators"
    );
}
