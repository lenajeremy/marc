use super::Node;

pub mod array_access_expression;
pub mod boolean_expression;
pub mod function_call_expression;
pub mod infix_expression;
pub mod integer_expression;
pub mod object_access_expression;
pub mod prefix_expression;
pub mod string_expression;
pub mod variable_access_expression;

pub use super::operators::{Comparators, Math, Op};
pub use crate::expander::object::{Object, TRUE, FALSE, NONE};
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
        self.evaluate().inspect()
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

fn evaluate_prefix_expressions(prefix_expression: &PrefixExpression) -> Object {
    let right_expression_evaluated = prefix_expression.right.evaluate();
    match right_expression_evaluated {
        Object::Integer(i) => {
            if prefix_expression.operator.string() == "+" {
                right_expression_evaluated
            } else if prefix_expression.operator.string() == "-" {
                Object::Integer(-i)
            } else {
                panic!("Integers can only have +/- as prefix operators")
            }
        }

        Object::Boolean(s) => {
            if prefix_expression.operator.string() == "!" {
                Object::Boolean(!s)
            } else {
                panic!("Booleans can only have ! as prefix operators")
            }
        },
        Object::None => panic!("Prefix operator cannot be applied on object type none")
    }
}

fn evaluate_infix_expression(infix_expression: &InfixExpression) -> Object {
    let left_expression_evaluated = infix_expression.left.evaluate();
    let right_expression_evaluated = infix_expression.right.evaluate();

    if left_expression_evaluated.get_type() != right_expression_evaluated.get_type() {
        panic!(
            "Only object of the same type should be operated together, got {} {} {}",
            left_expression_evaluated.get_type(),
            infix_expression.operator.string(),
            right_expression_evaluated.get_type()
        );
    }

    match left_expression_evaluated {
        Object::Integer(left_value) => {
            let Object::Integer(right_value) = right_expression_evaluated else {
                panic!("Right and left values should be the same type");
            };

            match infix_expression.operator {
                Op::Math(Math::Divide) => Object::Integer(left_value / right_value),
                Op::Math(Math::Plus) => Object::Integer(left_value + right_value),
                Op::Math(Math::Minus) => Object::Integer(left_value - right_value),
                Op::Math(Math::Product) => Object::Integer(left_value * right_value),
                Op::Comp(Comparators::GreaterThan) => Object::Boolean(left_value > right_value),
                Op::Comp(Comparators::LessThan) => Object::Boolean(left_value < right_value),
                Op::Comp(Comparators::GreaterQuals) => Object::Boolean(left_value >= right_value),
                Op::Comp(Comparators::LessQuals) => Object::Boolean(left_value <= right_value),
                Op::Comp(Comparators::Quals) => Object::Boolean(left_value == right_value),
                Op::Comp(Comparators::NeQuals) => Object::Boolean(left_value != right_value),
                Op::Not => panic!("Invalid operator"),
            }
        }
        Object::Boolean(left_value) => {
            let Object::Boolean(right_value) = right_expression_evaluated else {
                panic!("Right and right values should be the same type");
            };

            match infix_expression.operator {
                Op::Comp(Comparators::Quals) => Object::Boolean(left_value == right_value),
                Op::Comp(Comparators::NeQuals) => Object::Boolean(left_value != right_value),
                _ => panic!("Invalid operator"),
            }
        }
        Object::None => {
            if !(infix_expression.operator.string() == "==" && infix_expression.operator.string() == "!=") {
                panic!("Object type none cannot be operated with object type {}", right_expression_evaluated.get_type())
            }

            match right_expression_evaluated {
                Object::None => TRUE,
                _ => FALSE,
            }
        },
    }
}

impl Expression {
    pub fn evaluate(&self) -> Object {
        println!("evaluating expression {}", self.token_literal());
        match self {
            Self::Prefix(prefix_expression) => evaluate_prefix_expressions(prefix_expression),
            Self::OperatorInfix(infix_expression) => evaluate_infix_expression(infix_expression),
            Self::Integer(integer_expression) => Object::Integer(integer_expression.value),
            Self::True => TRUE,
            Self::False => FALSE,
            _ => NONE,
            // Expression::Empty => {}
        }
    }
}
