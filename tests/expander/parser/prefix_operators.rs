use md_to_html::expander::{
    ast::{
        Document, Node,
        expression::{Expression, IntegerExpression, PrefixExpression},
        operators::{Math, Op},
    },
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_prefix_operators() {
    let inputs = vec![
        ("{{ !5 }}", Op::Not, 5),
        ("{{ -5 }}", Op::Math(Math::Minus), 5),
        ("{{ +5 }}", Op::Math(Math::Plus), 5),
    ];

    for (input, op, val) in inputs {
        let lexer = Lexer::from(input);
        let mut p = Parser::new(lexer);
        let program = p.parse_document();

        let mut expected_program = Document::new();
        let integer_expr = Box::new(Expression::Integer(IntegerExpression::new(val)));
        let prefix_expr = Box::new(Expression::Prefix(PrefixExpression::new(op, integer_expr)));
        expected_program.add_node(prefix_expr);

        assert_eq!(
            program.token_literal(),
            expected_program.token_literal(),
            "failed for input: {}",
            input
        );
    }
}

#[test]
fn test_repeated_prefix_operators() {
    // - -5
    let input = "{{ - -5 }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();
    let integer_expr = Box::new(Expression::Integer(IntegerExpression::new(5)));

    // -5
    let inner_prefix = Box::new(Expression::Prefix(PrefixExpression::new(
        Op::Math(Math::Minus),
        integer_expr,
    )));

    // - (-5)
    let outer_prefix = Box::new(Expression::Prefix(PrefixExpression::new(
        Op::Math(Math::Minus),
        inner_prefix,
    )));

    expected_program.add_node(outer_prefix);

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for repeated prefix"
    );
}
