use md_to_html::expander::{
    ast::{
        Document, Node,
        expression::{Expression, VariableAccessExpression},
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
