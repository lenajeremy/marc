use md_to_html::expander::{
    ast::{
        Document, Node,
        expression::{
            ArrayAccessExpression, Expression, FunctionCallExpression, InfixExpression,
            IntegerExpression, VariableAccessExpression,
        },
    },
    lexer::Lexer,
    parser::Parser,
};

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
fn test_array_index_infix_expression() {
    // array[1 + 1]
    let input = "{{ array[1 + 1] }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();
    let parent = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "array".to_string(),
    )));

    // 1 + 1
    let index = Box::new(Expression::OperatorInfix(InfixExpression::new(
        Box::new(Expression::Integer(IntegerExpression::new(1))),
        Box::new(Expression::Integer(IntegerExpression::new(1))),
        md_to_html::expander::ast::operators::Op::Math(
            md_to_html::expander::ast::operators::Math::Plus,
        ),
    )));

    let array_access = Box::new(Expression::ArrayAccess(ArrayAccessExpression::new(
        parent, index,
    )));
    expected_program.add_node(array_access);

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for array index with infix expression"
    );
}

#[test]
fn test_array_index_function_call() {
    // array[getIndex()]
    let input = "{{ array[getIndex()] }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();
    let parent = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "array".to_string(),
    )));

    // getIndex()
    let func_name = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "getIndex".to_string(),
    )));
    let index = Box::new(Expression::FunctionCall(FunctionCallExpression::new(
        func_name,
    )));

    let array_access = Box::new(Expression::ArrayAccess(ArrayAccessExpression::new(
        parent, index,
    )));
    expected_program.add_node(array_access);

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for array index with function call"
    );
}

#[test]
fn test_array_access_on_function_result() {
    // getArray()[0]
    let input = "{{ getArray()[0] }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();

    // getArray()
    let func_name = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "getArray".to_string(),
    )));
    let parent_expr = Box::new(Expression::FunctionCall(FunctionCallExpression::new(
        func_name,
    )));

    let index = Box::new(Expression::Integer(IntegerExpression::new(0)));
    let array_access = Box::new(Expression::ArrayAccess(ArrayAccessExpression::new(
        parent_expr,
        index,
    )));
    expected_program.add_node(array_access);

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for array access on function result"
    );
}

#[test]
fn test_nested_array_access() {
    // arr[arr[0]]
    let input = "{{ arr[arr[0]] }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();
    let parent = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "arr".to_string(),
    )));

    // arr[0]
    let inner_parent = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "arr".to_string(),
    )));
    let inner_index = Box::new(Expression::Integer(IntegerExpression::new(0)));
    let index = Box::new(Expression::ArrayAccess(ArrayAccessExpression::new(
        inner_parent,
        inner_index,
    )));

    let array_access = Box::new(Expression::ArrayAccess(ArrayAccessExpression::new(
        parent, index,
    )));
    expected_program.add_node(array_access);

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for nested array access"
    );
}

