use std::collections::HashMap;

use super::{Node, operators::Op};

pub struct InfixExpression {
    left: Box<Expression>,
    right: Box<Expression>,
    operator: Op,
}

impl InfixExpression {
    fn literal(&self) -> String {
        format!(
            "{} {:?} {}",
            self.left.token_literal(),
            self.operator,
            self.right.token_literal()
        )
    }
}

pub struct PrefixExpression {
    operator: Op,
    right: Box<Expression>,
}

impl PrefixExpression {
    fn literal(&self) -> String {
        format!("{}{}", self.operator.string(), self.right.token_literal())
    }
}

pub struct VariableAccessExpression {
    variable_name: String,
}

impl VariableAccessExpression {
    fn literal(&self) -> String {
        self.variable_name.clone()
    }

    pub fn new(variable_name: String) -> Self {
        VariableAccessExpression {
            variable_name: variable_name,
        }
    }
}

pub struct ObjectAccessExpression {
    parent: Box<Expression>,
    child: Box<Expression>,
}

impl ObjectAccessExpression {
    fn literal(&self) -> String {
        format!(
            "{}.{}",
            self.parent.token_literal(),
            self.child.token_literal()
        )
    }
    pub fn new(parent: Box<Expression>, child: Box<Expression>) -> Self {
        Self {
            parent: parent,
            child: child,
        }
    }
}

pub struct ArrayAccessExpression {
    parent: Box<Expression>,
    index: usize,
}

impl ArrayAccessExpression {
    fn literal(&self) -> String {
        format!("{}[{}]", self.parent.token_literal(), self.index)
    }

    pub fn new(parent: Box<Expression>, index: usize) -> Self {
        Self { index, parent }
    }
}

pub struct FunctionCallExpression {
    function_name: String,
    args: Vec<Box<Expression>>,
}

impl FunctionCallExpression {
    fn literal(&self) -> String {
        format!(
            "{}({})",
            self.function_name,
            self.args
                .iter()
                .map(|x| { x.token_literal() })
                .collect::<Vec<String>>()
                .join(",")
        )
    }

    pub fn new(function_name: String) -> Self {
        Self {
            function_name,
            args: vec![],
        }
    }

    pub fn add_arg(&mut self, arg: Box<Expression>) {
        self.args.push(arg);
    }
}

pub struct IntegerExpression {
    value: i64,
}

impl IntegerExpression {
    fn literal(&self) -> String {
        format!("Integer({})", self.value)
    }
}

pub struct StringExpression {
    value: String,
}

impl StringExpression {
    fn literal(&self) -> String {
        self.value.to_owned()
    }
}

pub enum Expression {
    Binary(InfixExpression),
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
            Self::Binary(e) => e.literal(),
            Self::VariableAccess(e) => e.literal(),
            Self::ArrayAccess(e) => e.literal(),
            Self::ObjectAccess(e) => e.literal(),
            Self::FunctionCall(e) => e.literal(),
            Self::Integer(i) => i.literal(),
            Self::String(i) => i.literal(),
            Self::True => "true".to_string(),
            Self::False => "false".to_string(),
            Self::Empty => "".to_string(),
        }
    }

    fn translate(&self) -> String {
        self.token_literal()
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}
