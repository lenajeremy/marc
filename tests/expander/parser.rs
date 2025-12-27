use md_to_html::expander::{
    ast::{
        Document, Node,
        expression::{
            ArrayAccessExpression, Expression, InfixExpression, IntegerExpression,
            ObjectAccessExpression, PrefixExpression, VariableAccessExpression,
        },
        operators::{Math, Op},
        text_node::TextNode,
    },
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_variable_input() {
    let input = "Hello {{ name }}
Hello {{ jeremiah }}
I have {{ numberOfApples }} apples in my bag.";

    let new_line = TextNode::new(String::from("\n"));

    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let mut expected_program = Document::new();

    let hello = TextNode::new(String::from("Hello "));
    let name_variable_expression = Box::new(Expression::VariableAccess(
        VariableAccessExpression::new(String::from("name")),
    ));

    let jeremiah_variable_expression = Box::new(Expression::VariableAccess(
        VariableAccessExpression::new(String::from("jeremiah")),
    ));

    let num_apples_variable = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        String::from("numberOfApples"),
    )));

    expected_program.add_node(Box::new(hello.clone()));
    expected_program.add_node(name_variable_expression);
    expected_program.add_node(Box::new(new_line.clone()));

    expected_program.add_node(Box::new(hello.clone()));
    expected_program.add_node(jeremiah_variable_expression);
    expected_program.add_node(Box::new(new_line.clone()));

    expected_program.add_node(Box::new(TextNode::new(String::from("I have "))));
    expected_program.add_node(num_apples_variable);
    expected_program.add_node(Box::new(TextNode::new(String::from(" apples in my bag."))));
    let program = p.parse_document();
    assert_eq!(program.token_literal(), expected_program.token_literal())
}

#[test]
fn test_integer_expressions() {
    let input = "{{ 5 }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();
    expected_program.add_node(Box::new(Expression::Integer(IntegerExpression::new(5))));

    assert_eq!(program.token_literal(), expected_program.token_literal());
}

#[test]
fn test_prefix_operators() {
    // Test !5, -5 (note: +5 might be valid if parser handles it, assuming yes based on parser.rs)
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
fn test_object_expressions() {
    let input = "{{ name.upper }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();
    let parent = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "name".to_string(),
    )));
    let child = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "upper".to_string(),
    )));
    let obj_access = Box::new(Expression::ObjectAccess(ObjectAccessExpression::new(
        parent, child,
    )));
    expected_program.add_node(obj_access);

    assert_eq!(program.token_literal(), expected_program.token_literal());
}

#[test]
fn test_array_expressions() {
    let input = "{{ array[0] }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();
    let parent = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "array".to_string(),
    )));
    let index = Box::new(Expression::Integer(IntegerExpression::new(0)));
    let array_access = Box::new(Expression::ArrayAccess(ArrayAccessExpression::new(
        parent, index,
    )));
    expected_program.add_node(array_access);

    assert_eq!(program.token_literal(), expected_program.token_literal());
}

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
    let input = "{{ a + b * c }}";
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

    // b * c
    let b_times_c = Box::new(Expression::OperatorInfix(InfixExpression::new(
        b_var,
        c_var,
        Op::Math(Math::Product),
    )));

    // a + (b * c)
    let a_plus_bc = Box::new(Expression::OperatorInfix(InfixExpression::new(
        a_var,
        b_times_c,
        Op::Math(Math::Plus),
    )));

    expected_program.add_node(a_plus_bc);

    println!("{}", program.token_literal());
    println!("{}", expected_program.token_literal());

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for precedence test"
    );
}
