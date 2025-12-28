use md_to_html::expander::{
    ast::{
        Document, Node,
        expression::{
            ArrayAccessExpression, Expression, FunctionCallExpression, IntegerExpression,
            ObjectAccessExpression, VariableAccessExpression,
        },
    },
    lexer::Lexer,
    parser::Parser,
};

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
fn test_nested_object_access() {
    // user.address.street
    let input = "{{ user.address.street }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();

    // user.address
    let user_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "user".to_string(),
    )));
    let address_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "address".to_string(),
    )));
    let user_address = Box::new(Expression::ObjectAccess(ObjectAccessExpression::new(
        user_var,
        address_var,
    )));

    // (user.address).street
    let street_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "street".to_string(),
    )));
    let access = Box::new(Expression::ObjectAccess(ObjectAccessExpression::new(
        user_address,
        street_var,
    )));

    expected_program.add_node(access);

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for nested object access"
    );
}

#[test]
fn test_object_access_on_function_result() {
    // getUser().name
    let input = "{{ getUser().name }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();

    // getUser()
    let get_user_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "getUser".to_string(),
    )));
    let func_call = Box::new(Expression::FunctionCall(FunctionCallExpression::new(
        get_user_var,
    )));

    // (getUser()).name
    let name_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "name".to_string(),
    )));
    let access = Box::new(Expression::ObjectAccess(ObjectAccessExpression::new(
        func_call, name_var,
    )));

    expected_program.add_node(access);

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for object access on function result"
    );
}

#[test]
fn test_object_access_on_array_element() {
    // users[0].name
    let input = "{{ users[0].name }}";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_document();

    let mut expected_program = Document::new();

    // users[0]
    let users_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "users".to_string(),
    )));
    let index_0 = Box::new(Expression::Integer(IntegerExpression::new(0)));
    let array_access = Box::new(Expression::ArrayAccess(ArrayAccessExpression::new(
        users_var, index_0,
    )));

    // (users[0]).name
    let name_var = Box::new(Expression::VariableAccess(VariableAccessExpression::new(
        "name".to_string(),
    )));
    let access = Box::new(Expression::ObjectAccess(ObjectAccessExpression::new(
        array_access,
        name_var,
    )));

    expected_program.add_node(access);

    assert_eq!(
        program.token_literal(),
        expected_program.token_literal(),
        "failed for object access on array element"
    );
}
