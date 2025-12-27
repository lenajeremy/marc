use super::Node;

pub mod array_access_expression;
pub mod function_call_expression;
pub mod infix_expression;
pub mod integer_expression;
pub mod object_access_expression;
pub mod prefix_expression;
pub mod string_expression;
pub mod variable_access_expression;

pub use array_access_expression::*;
pub use function_call_expression::*;
pub use infix_expression::*;
pub use integer_expression::*;
pub use object_access_expression::*;
pub use prefix_expression::*;
pub use string_expression::*;
pub use variable_access_expression::*;

pub enum Expression {
    OperatorInfix(InfixExpression),
    Prefix(PrefixExpression),
    VariableAccess(VariableAccessExpression),
    ObjectAccess(ObjectAccessExpression),
    ArrayAccess(ArrayAccessExpression),
    FunctionCall(FunctionCallExpression),
    Integer(IntegerExpression),
    String(StringExpression),
    True,
    False,
    Empty,
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Self::Prefix(i) => i.literal(),
            Self::OperatorInfix(i) => i.literal(),
            Self::VariableAccess(i) => i.literal(),
            Self::ArrayAccess(i) => i.literal(),
            Self::ObjectAccess(i) => i.literal(),
            Self::FunctionCall(i) => i.literal(),
            Self::Integer(i) => i.literal(),
            Self::String(i) => i.literal(),
            Self::True => "true".to_string(),
            Self::False => "false".to_string(),
            Self::Empty => "EMPTY".to_string(),
        }
    }

    fn translate(&self) -> String {
        self.token_literal()
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}
