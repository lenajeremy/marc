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
    let input = "\
Hello {{ name }}
Hello {{ jeremiah }}
I have {{ numberOfApples }} apples in my bag.
";
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

//#[test]
//fn test_object_expressions() {
//    let input = "\
//Hello {{ name }}
//Hello {{ name.upper }}
//Hello {{ name[0] }}
//";
//    let lexer = Lexer::from(input);
//    let mut p = Parser::new(lexer);
//    let mut expected_program = Program::new();
//
//    let mut paragraph1 = Box::new(ParagraphText::new());
//    let mut paragraph1_inner = Box::new(InlineContainer::new());
//    paragraph1_inner.add_child(Box::new(Text::new("Hello ".to_string())));
//    let name_variable = VariableAccessExpression::new("name".to_string());
//    paragraph1_inner.add_child(Box::new(Expression::VariableAccess(name_variable)));
//    paragraph1.set_inner(paragraph1_inner);
//
//    let mut paragraph2 = Box::new(ParagraphText::new());
//    let mut paragraph2_inner = Box::new(InlineContainer::new());
//    paragraph2_inner.add_child(Box::new(Text::new("Hello ".to_string())));
//    let name_upper = Expression::ObjectAccess(ObjectAccessExpression::new(
//        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
//            "name".to_string(),
//        ))),
//        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
//            "upper".to_string(),
//        ))),
//    ));
//    paragraph2_inner.add_child(Box::new(name_upper));
//    paragraph2.set_inner(paragraph2_inner);
//
//    let mut paragraph3 = Box::new(ParagraphText::new());
//    let mut paragraph3_inner = Box::new(InlineContainer::new());
//    paragraph3_inner.add_child(Box::new(Text::new("Hello ".to_string())));
//    let name_zero = Expression::ArrayAccess(ArrayAccessExpression::new(
//        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
//            "name".to_string(),
//        ))),
//        0,
//    ));
//    paragraph3_inner.add_child(Box::new(name_zero));
//    paragraph3.set_inner(paragraph3_inner);
//
//    expected_program.add_block(paragraph1);
//    expected_program.add_block(paragraph2);
//    expected_program.add_block(paragraph3);
//}
//
//#[test]
//fn test_for_blocks() {
//    let input = "\
//{% for name in people.name %}
//- {{ name }}
//{% endfor %}
//";
//}
