use md_to_html::expander::{
    ast::{
        Document, Node,
        expression::{Expression, IntegerExpression},
    },
    lexer::Lexer,
    parser::Parser,
};

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
