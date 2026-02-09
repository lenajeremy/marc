use md_to_html::expander::{
    ast::{
        Document, Node,
        expression::{Expression, InfixExpression, IntegerExpression, VariableAccessExpression},
        operators::{Comparators, Math, Op},
    },
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_comparator_expressions() {
    let inputs = vec![
        ("{{ a == b }}", Op::Comp(Comparators::Quals)),
        ("{{ a != b }}", Op::Comp(Comparators::NeQuals)),
        ("{{ a < b }}", Op::Comp(Comparators::LessThan)),
        ("{{ a > b }}", Op::Comp(Comparators::GreaterThan)),
        ("{{ a <= b }}", Op::Comp(Comparators::LessQuals)),
        ("{{ a >= b }}", Op::Comp(Comparators::GreaterQuals)),
    ];

    for (input, op) in inputs {
        let lexer = Lexer::from(input);
        let mut p = Parser::new(lexer);
        let program = p.parse_document();

        let mut expected_program = Document::new();
        let left = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
            "a".to_string(),
        )));
        let right = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
            "b".to_string(),
        )));
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
fn test_comparator_precedence_with_math() {
    // a + b > c * d  -> (a + b) > (c * d)
    let input = "{{ a + b > c * d }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();

    let left = Box::new(Expression::OperatorInfix(InfixExpression::new(
        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
            "a".to_string(),
        ))),
        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
            "b".to_string(),
        ))),
        Op::Math(Math::Plus),
    )));

    let right = Box::new(Expression::OperatorInfix(InfixExpression::new(
        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
            "c".to_string(),
        ))),
        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
            "d".to_string(),
        ))),
        Op::Math(Math::Product),
    )));

    let infix = Box::new(Expression::OperatorInfix(InfixExpression::new(
        left,
        right,
        Op::Comp(Comparators::GreaterThan),
    )));

    expected_program.add_node(infix);

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for comparator precedence"
    );
}

#[test]
fn test_comparator_precedence_with_integers() {
    // 1 + 2 == 3 -> (1 + 2) == 3
    let input = "{{ 1 + 2 == 3 }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();

    let left = Box::new(Expression::OperatorInfix(InfixExpression::new(
        Box::new(Expression::Integer(IntegerExpression::new(1))),
        Box::new(Expression::Integer(IntegerExpression::new(2))),
        Op::Math(Math::Plus),
    )));

    let right = Box::new(Expression::Integer(IntegerExpression::new(3)));

    let infix = Box::new(Expression::OperatorInfix(InfixExpression::new(
        left,
        right,
        Op::Comp(Comparators::Quals),
    )));

    expected_program.add_node(infix);

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for integer comparator precedence"
    );
}

#[test]
fn test_grouped_comparisons() {
    // (a < b) == (c > d)
    let input = "{{ (a < b) == (c > d) }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();

    let left = Box::new(Expression::OperatorInfix(InfixExpression::new(
        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
            "a".to_string(),
        ))),
        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
            "b".to_string(),
        ))),
        Op::Comp(Comparators::LessThan),
    )));

    let right = Box::new(Expression::OperatorInfix(InfixExpression::new(
        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
            "c".to_string(),
        ))),
        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
            "d".to_string(),
        ))),
        Op::Comp(Comparators::GreaterThan),
    )));

    let infix = Box::new(Expression::OperatorInfix(InfixExpression::new(
        left,
        right,
        Op::Comp(Comparators::Quals),
    )));

    expected_program.add_node(infix);

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for grouped comparator expressions"
    );
}

#[test]
fn test_comparator_with_array_access() {
    // arr[0] != arr[1]
    let input = "{{ arr[0] != arr[1] }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();

    let left = Box::new(Expression::ArrayAccess(
        md_to_html::expander::ast::expression::ArrayAccessExpression::new(
            Box::new(Expression::VariableAccess(VariableAccessExpression::new(
                "arr".to_string(),
            ))),
            Box::new(Expression::Integer(IntegerExpression::new(0))),
        ),
    ));

    let right = Box::new(Expression::ArrayAccess(
        md_to_html::expander::ast::expression::ArrayAccessExpression::new(
            Box::new(Expression::VariableAccess(VariableAccessExpression::new(
                "arr".to_string(),
            ))),
            Box::new(Expression::Integer(IntegerExpression::new(1))),
        ),
    ));

    let infix = Box::new(Expression::OperatorInfix(InfixExpression::new(
        left,
        right,
        Op::Comp(Comparators::NeQuals),
    )));

    expected_program.add_node(infix);

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for array access comparator"
    );
}

#[test]
fn test_comparator_with_object_access() {
    // user.age >= 18
    let input = "{{ user.age >= 18 }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();

    let left = Box::new(Expression::ObjectAccess(
        md_to_html::expander::ast::expression::ObjectAccessExpression::new(
            Box::new(Expression::VariableAccess(VariableAccessExpression::new(
                "user".to_string(),
            ))),
            Box::new(Expression::VariableAccess(VariableAccessExpression::new(
                "age".to_string(),
            ))),
        ),
    ));

    let right = Box::new(Expression::Integer(IntegerExpression::new(18)));

    let infix = Box::new(Expression::OperatorInfix(InfixExpression::new(
        left,
        right,
        Op::Comp(Comparators::GreaterQuals),
    )));

    expected_program.add_node(infix);

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for object access comparator"
    );
}
