use md_to_html::expander::{
    ast::{
        Document, Node,
        expression::{
            Expression, FunctionCallExpression, InfixExpression, IntegerExpression,
            ObjectAccessExpression, VariableAccessExpression,
        },
    },
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_function_calls() {
    let input = "{{ foo(bar, baz, 10) }}";
    let lexer = Lexer::from(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_document();

    let mut expected_program = Document::new();

    let foo_identifier = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        String::from("foo"),
    )));

    let bar_function_arg = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        String::from("bar"),
    )));

    let baz_function_arg = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        String::from("baz"),
    )));

    let ten_function_arg = Box::new(Expression::Integer(IntegerExpression::new(10)));

    let mut function_expression = FunctionCallExpression::new(foo_identifier);

    function_expression.add_arg(bar_function_arg);
    function_expression.add_arg(baz_function_arg);
    function_expression.add_arg(ten_function_arg);

    expected_program.add_node(Box::new(Expression::FunctionCall(function_expression)));

    assert_eq!(program.token_literal(), expected_program.token_literal());
}

#[test]
fn test_function_call_with_expression_args() {
    // add(1 + 2, a * b)
    let input = "{{ add(1 + 2, a * b) }}";
    let lexer = Lexer::from(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_document();

    let mut expected_program = Document::new();
    let add_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "add".to_string(),
    )));
    let mut func_call = FunctionCallExpression::new(add_var);

    // 1 + 2
    let arg1 = Box::new(Expression::OperatorInfix(InfixExpression::new(
        Box::new(Expression::Integer(IntegerExpression::new(1))),
        Box::new(Expression::Integer(IntegerExpression::new(2))),
        md_to_html::expander::ast::operators::Op::Math(
            md_to_html::expander::ast::operators::Math::Plus,
        ),
    )));
    func_call.add_arg(arg1);

    // a * b
    let arg2 = Box::new(Expression::OperatorInfix(InfixExpression::new(
        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
            "a".to_string(),
        ))),
        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
            "b".to_string(),
        ))),
        md_to_html::expander::ast::operators::Op::Math(
            md_to_html::expander::ast::operators::Math::Product,
        ),
    )));
    func_call.add_arg(arg2);

    expected_program.add_node(Box::new(Expression::FunctionCall(func_call)));

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for function call with expression args"
    );
}

#[test]
fn test_nested_function_calls() {
    // outer(inner())
    let input = "{{ outer(inner()) }}";
    let lexer = Lexer::from(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_document();

    let mut expected_program = Document::new();
    let outer_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "outer".to_string(),
    )));
    let mut outer_call = FunctionCallExpression::new(outer_var);

    // inner()
    let inner_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "inner".to_string(),
    )));
    let inner_call = Box::new(Expression::FunctionCall(FunctionCallExpression::new(
        inner_var,
    )));
    outer_call.add_arg(inner_call);

    expected_program.add_node(Box::new(Expression::FunctionCall(outer_call)));

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for nested function calls"
    );
}

#[test]
fn test_function_call_chained() {
    // obj.method(arg)
    let input = "{{ obj.method(arg) }}";
    let lexer = Lexer::from(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_document();

    let mut expected_program = Document::new();

    // obj.method
    let obj_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "obj".to_string(),
    )));
    let method_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "method".to_string(),
    )));
    let obj_method = Box::new(Expression::ObjectAccess(ObjectAccessExpression::new(
        obj_var, method_var,
    )));

    let mut func_call = FunctionCallExpression::new(obj_method);
    let arg_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "arg".to_string(),
    )));
    func_call.add_arg(arg_var);

    expected_program.add_node(Box::new(Expression::FunctionCall(func_call)));

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for chained function call"
    );
}
