use md_to_html::marc::{
    ast::{Node, Program, code::CodeBlock},
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_parses_code_blocks() {
    let input = "```python
print(\"Hello World\")
```";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);

    let mut expected_program = Program::new();
    let code_block = Box::new(CodeBlock::new(
        "print(\"Hello World\")\n".to_string(),
        "python".to_string(),
    ));
    expected_program.add_block(code_block);

    let parsed_program = p.parse_program();
    println!(
        "{}\n{}",
        expected_program.token_literal(),
        parsed_program.token_literal()
    );

    assert_eq!(
        parsed_program.token_literal(),
        expected_program.token_literal()
    )
}
