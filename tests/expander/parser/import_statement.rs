use md_to_html::expander::{
    ast::{Document, Node, statement::ImportStatement},
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_import_statement() {
    let input = "@import \"person.json\" as person";
    let lexer = Lexer::from(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_document();

    let mut expected_program = Document::new();
    expected_program.add_node(Box::new(ImportStatement::new(
        "person.json".to_string(),
        "person".to_string(),
    )));

    assert_eq!(program.token_literal(), expected_program.token_literal());
}
