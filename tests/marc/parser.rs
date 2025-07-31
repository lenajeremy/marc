use md_to_html::marc::{
    ast::{
        Node, Program,
        block_quote::BlockQuote,
        code::CodeBlock,
        heading::Heading,
        image::Image,
        inline_container::InlineContainer,
        link::Link,
        text::{BoldText, Text},
    },
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn parse_image() {
    let input = "![this is the alt](https://google.com)";
    let lexer = Lexer::from(input);
    let mut parser = Parser::new(lexer);

    let image = Box::new(Image::new(
        "https://google.com".to_string(),
        "this is the alt".to_string(),
    ));

    let mut expected_program = Program::new();
    expected_program.add_block(image);

    let program = parser.parse_program();

    println!(
        "Got: {}, Expected: {}",
        program.token_literal(),
        expected_program.token_literal()
    );

    assert_eq!(program.token_literal(), expected_program.token_literal())
}

#[test]
fn parse_link() {
    let input = "\
[this is the alt](https://google.com)
[hello world](https://jeremiah.vercel.app)";
    let lexer = Lexer::from(input);
    let mut parser = Parser::new(lexer);

    let link1 = Box::new(Link::new(
        "this is the alt".to_string(),
        "https://google.com".to_string(),
    ));

    let link2 = Box::new(Link::new(
        "hello world".to_string(),
        "https://jeremiah.vercel.app".to_string(),
    ));

    let mut expected_program = Program::new();
    expected_program.add_block(link1);
    expected_program.add_block(link2);

    let program = parser.parse_program();

    println!(
        "Got: {} \n Expected: {}",
        program.token_literal(),
        expected_program.token_literal()
    );

    assert_eq!(program.token_literal(), expected_program.token_literal())
}

#[test]
fn parse_heading_1() {
    let input = "# Hello World\n## Hello World 2";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);

    let mut expected_program = Program::new();
    let mut heading_block = Box::new(Heading::new(1));
    let mut heading_2_block = Box::new(Heading::new(2));

    let mut inner_1 = Box::new(InlineContainer::new());
    let mut inner_2 = Box::new(InlineContainer::new());

    inner_1.add_child(Box::new(Text::new(" Hello World".to_string())));
    inner_2.add_child(Box::new(Text::new(" Hello World 2".to_string())));

    heading_block.set_inner(inner_1);
    heading_2_block.set_inner(inner_2);

    expected_program.add_block(heading_block);
    expected_program.add_block(heading_2_block);

    let parsed_program = p.parse_program();

    println!("Got: {}", parsed_program.token_literal());
    println!("Expected: {}", expected_program.token_literal());

    assert_eq!(
        parsed_program.token_literal(),
        expected_program.token_literal()
    );
}

#[test]
fn parses_heading_with_inline_elements() {
    let input = "# Hello **World**";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);

    let mut expected_program = Program::new();

    let mut heading_block = Box::new(Heading::new(1));
    let mut inner = Box::new(InlineContainer::new());
    inner.add_child(Box::new(Text::new(" Hello ".to_string())));

    let mut bold_text = Box::new(BoldText::new());
    let mut bold_inner = Box::new(InlineContainer::new());
    bold_inner.add_child(Box::new(Text::new("World".to_string())));
    bold_text.set_inner(bold_inner);

    inner.add_child(bold_text);

    heading_block.set_inner(inner);

    expected_program.add_block(heading_block);
    let parsed_program = p.parse_program();

    println!("{}", parsed_program.token_literal());
    println!("{}", expected_program.token_literal());

    assert_eq!(
        parsed_program.token_literal(),
        expected_program.token_literal()
    );
}

#[test]
fn test_parses_block_quote_valid() {
    let input = "> Hello World";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);

    let mut expected_program = Program::new();

    let mut block_quote = Box::new(BlockQuote::new());
    let mut block_quote_inner = Box::new(InlineContainer::new());
    block_quote_inner.add_child(Box::new(Text::new(" Hello World".to_string())));

    block_quote.set_inner(block_quote_inner);

    expected_program.add_block(block_quote);

    let parsed_program = p.parse_program();

    assert_eq!(
        parsed_program.token_literal(),
        expected_program.token_literal()
    );
}

#[test]
fn test_parses_block_quote_with_middle_sign() {
    let input = "> Hello > World";
    let lexer = Lexer::from(input);
    let mut p = Parser::new(lexer);

    let mut expected_program = Program::new();

    let mut block_quote = Box::new(BlockQuote::new());
    let mut block_quote_inner = Box::new(InlineContainer::new());
    block_quote_inner.add_child(Box::new(Text::new(" Hello > World".to_string())));

    block_quote.set_inner(block_quote_inner);

    expected_program.add_block(block_quote);

    let parsed_program = p.parse_program();

    assert_eq!(
        parsed_program.token_literal(),
        expected_program.token_literal()
    );
}

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
