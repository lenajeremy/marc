use md_to_html::expander::{ast::Node, environment::Environment, lexer::Lexer, parser::Parser};

// #[test]
// fn test_assigns_variable_expression() {
//     let input = "{% fn main() %}
// let name = 40 + 50
// return name
// {% endfn %}
//
// {{ main() }}
// ";
//     let lexer = Lexer::from(input);
//     let mut parser = Parser::new(lexer);
//     let mut env = Environment::new();
//
//     let document = parser.parse_document();
//     let res = document.translate(&mut env);
//
//     assert_eq!(res.trim(), "90");
// }
