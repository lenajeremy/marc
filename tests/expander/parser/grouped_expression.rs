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
fn test_grouped_expressions() {
    // Case 1: Simple grouping (5 + 5)
    {
        let input = "{{ (5 + 5) }}";
        let lexer = Lexer::from(input);
        let mut p = Parser::new(lexer);
        let program = p.parse_document();

        let mut expected_program = Document::new();
        // (5 + 5)
        let infix = Box::new(Expression::OperatorInfix(InfixExpression::new(
            Box::new(Expression::Integer(IntegerExpression::new(5))),
            Box::new(Expression::Integer(IntegerExpression::new(5))),
            Op::Math(Math::Plus),
        )));
        expected_program.add_node(infix);

        assert_eq!(
            program.token_literal(),
            expected_program.token_literal(),
            "failed for simple grouped expression: {}",
            input
        );
    }

    // Case 2: Precedence override (5 + 5) * 5
    {
        let input = "{{ (5 + 5) * 5 }}";
        let lexer = Lexer::from(input);
        let mut p = Parser::new(lexer);
        let program = p.parse_document();

        let mut expected_program = Document::new();
        // (5 + 5)
        let five_plus_five = Box::new(Expression::OperatorInfix(InfixExpression::new(
            Box::new(Expression::Integer(IntegerExpression::new(5))),
            Box::new(Expression::Integer(IntegerExpression::new(5))),
            Op::Math(Math::Plus),
        )));

        // Result * 5
        let infix = Box::new(Expression::OperatorInfix(InfixExpression::new(
            five_plus_five,
            Box::new(Expression::Integer(IntegerExpression::new(5))),
            Op::Math(Math::Product),
        )));

        expected_program.add_node(infix);

        assert_eq!(
            program.token_literal(),
            expected_program.token_literal(),
            "failed for grouped precedence expression: {}",
            input
        );
    }

    // Case 3: Right side grouping 5 * (5 + 5)
    {
        let input = "{{ 5 * (5 + 5) }}";
        let lexer = Lexer::from(input);
        let mut p = Parser::new(lexer);
        let program = p.parse_document();

        let mut expected_program = Document::new();

        // (5 + 5)
        let five_plus_five = Box::new(Expression::OperatorInfix(InfixExpression::new(
            Box::new(Expression::Integer(IntegerExpression::new(5))),
            Box::new(Expression::Integer(IntegerExpression::new(5))),
            Op::Math(Math::Plus),
        )));

        // 5 * Result
        let infix = Box::new(Expression::OperatorInfix(InfixExpression::new(
            Box::new(Expression::Integer(IntegerExpression::new(5))),
            five_plus_five,
            Op::Math(Math::Product),
        )));

        expected_program.add_node(infix);

        assert_eq!(
            program.token_literal(),
            expected_program.token_literal(),
            "failed for right-side grouped expression: {}",
            input
        );
    }

    // Case 4: Nested grouping ((5 + 5) * 5)
    {
        let input = "{{ ((5 + 5) * 5) }}";
        let lexer = Lexer::from(input);
        let mut p = Parser::new(lexer);
        let program = p.parse_document();

        let mut expected_program = Document::new();

        // (5 + 5)
        let inner_sum = Box::new(Expression::OperatorInfix(InfixExpression::new(
            Box::new(Expression::Integer(IntegerExpression::new(5))),
            Box::new(Expression::Integer(IntegerExpression::new(5))),
            Op::Math(Math::Plus),
        )));

        // Inner * 5
        let outer_product = Box::new(Expression::OperatorInfix(InfixExpression::new(
            inner_sum,
            Box::new(Expression::Integer(IntegerExpression::new(5))),
            Op::Math(Math::Product),
        )));

        expected_program.add_node(outer_product);

        assert_eq!(
            program.token_literal(),
            expected_program.token_literal(),
            "failed for nested grouped expression: {}",
            input
        );
    }
}
