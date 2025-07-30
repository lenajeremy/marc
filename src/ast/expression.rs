use std::collections::HashMap;

use super::{Node, operators::Op};

pub struct BinaryExpression {
    left: Box<Expression>,
    right: Box<Expression>,
    operator: Op,
}

impl BinaryExpression {
    fn literal(&self) -> String {
        format!(
            "{} {:?} {}",
            self.left.token_literal(),
            self.operator,
            self.right.token_literal()
        )
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

pub enum Expression {
    Binary(BinaryExpression),
    VariableAccess(VariableAccessExpression),
    ObjectAccess(ObjectAccessExpression),
    ArrayAccess(ArrayAccessExpression),
    FunctionCall(FunctionCallExpression),
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

fn dance(x: i32, y: i32) -> String {
    String::from("hello world")
}

fn test(action: impl Fn(i32, i32) -> String) {
    let res = action(50, 50);
    let res2 = action(50, 50);
}

fn main() {
    let cb = |x: i32, y: i32| {
        println!("{}", x + y);
        format!("{}", x + y)
    };

    let mut hm = HashMap::new();
    hm.insert("hello".to_string(), cb);

    let s = hm.get(&"hello".to_string()).unwrap();
    let res = s(50, 100);

    test(cb);
    test(dance);
}
