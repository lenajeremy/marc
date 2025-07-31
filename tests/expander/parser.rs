//expression::{
//    ArrayAccessExpression, Expression, ObjectAccessExpression, VariableAccessExpression,
//},

//#[test]
//fn test_variable_input() {
//    let input = "\
//Hello {{ name }}
//Hello {{ jeremiah }}
//I have {{ numberOfApples }} apples in my bag.
//";
//
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
//    let name_variable = VariableAccessExpression::new("jeremiah".to_string());
//    paragraph2_inner.add_child(Box::new(Expression::VariableAccess(name_variable)));
//    paragraph2.set_inner(paragraph2_inner);
//
//    let mut paragraph3 = Box::new(ParagraphText::new());
//    let mut paragraph3_inner = Box::new(InlineContainer::new());
//    paragraph3_inner.add_child(Box::new(Text::new("I have ".to_string())));
//    let name_variable = VariableAccessExpression::new("numberOfApples".to_string());
//    paragraph3_inner.add_child(Box::new(Expression::VariableAccess(name_variable)));
//    paragraph3_inner.add_child(Box::new(Text::new("apples. in my bag.".to_string())));
//    paragraph3.set_inner(paragraph3_inner);
//
//    expected_program.add_block(paragraph1);
//    expected_program.add_block(paragraph2);
//    expected_program.add_block(paragraph3);
//
//    let parsed_program = p.parse_program();
//    println!("parsed program\n{}", parsed_program.token_literal());
//    println!("expected program\n{}", expected_program.token_literal());
//
//    assert_eq!(
//        parsed_program.token_literal(),
//        expected_program.token_literal()
//    );
//}
//
////#[test]
////fn test_object_expressions() {
////    let input = "\
////Hello {{ name }}
////Hello {{ name.upper }}
////Hello {{ name[0] }}
////";
////    let lexer = Lexer::from(input);
////    let mut p = Parser::new(lexer);
////    let mut expected_program = Program::new();
////
////    let mut paragraph1 = Box::new(ParagraphText::new());
////    let mut paragraph1_inner = Box::new(InlineContainer::new());
////    paragraph1_inner.add_child(Box::new(Text::new("Hello ".to_string())));
////    let name_variable = VariableAccessExpression::new("name".to_string());
////    paragraph1_inner.add_child(Box::new(Expression::VariableAccess(name_variable)));
////    paragraph1.set_inner(paragraph1_inner);
////
////    let mut paragraph2 = Box::new(ParagraphText::new());
////    let mut paragraph2_inner = Box::new(InlineContainer::new());
////    paragraph2_inner.add_child(Box::new(Text::new("Hello ".to_string())));
////    let name_upper = Expression::ObjectAccess(ObjectAccessExpression::new(
////        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
////            "name".to_string(),
////        ))),
////        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
////            "upper".to_string(),
////        ))),
////    ));
////    paragraph2_inner.add_child(Box::new(name_upper));
////    paragraph2.set_inner(paragraph2_inner);
////
////    let mut paragraph3 = Box::new(ParagraphText::new());
////    let mut paragraph3_inner = Box::new(InlineContainer::new());
////    paragraph3_inner.add_child(Box::new(Text::new("Hello ".to_string())));
////    let name_zero = Expression::ArrayAccess(ArrayAccessExpression::new(
////        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
////            "name".to_string(),
////        ))),
////        0,
////    ));
////    paragraph3_inner.add_child(Box::new(name_zero));
////    paragraph3.set_inner(paragraph3_inner);
////
////    expected_program.add_block(paragraph1);
////    expected_program.add_block(paragraph2);
////    expected_program.add_block(paragraph3);
////}
////
////#[test]
////fn test_for_blocks() {
////    let input = "\
////{% for name in people.name %}
////- {{ name }}
////{% endfor %}
////";
////}
