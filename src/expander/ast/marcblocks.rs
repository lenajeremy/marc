use std::any::Any;

use crate::expander::ast::MarcNode;

use super::{
    Node,
    expression::{Expression, VariableAccessExpression},
    text_node::TextNode,
};

pub struct IfBlock {
    expression: Expression,
    valid: Vec<Box<MarcNode>>,
    invalid: Vec<Box<MarcNode>>,
    literal: String,
}

impl IfBlock {
    pub fn new(expression: Expression) -> IfBlock {
        IfBlock {
            expression: expression,
            valid: vec![],
            invalid: vec![],
            literal: String::new(),
        }
    }

    fn add_valid_block(&mut self, block: Box<MarcNode>) {
        self.valid.push(block)
    }

    fn add_invalid_block(&mut self, block: Box<MarcNode>) {
        self.invalid.push(block)
    }

    fn set_literal(&mut self, literal: String) {
        self.literal = literal
    }
}

impl Node for IfBlock {
    fn token_literal(&self) -> String {
        self.literal.clone()
    }

    fn translate(&self) -> String {
        // this would be updated to take the scope
        self.token_literal()
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

pub struct ForBlock {
    main_list: Expression,
    variable: VariableAccessExpression,
    operations: Vec<Box<dyn Node>>,
    literal: String,
}

impl ForBlock {
    pub fn new(list: Expression, variable: VariableAccessExpression) -> ForBlock {
        ForBlock {
            main_list: list,
            variable: variable,
            operations: vec![],
            literal: String::new(),
        }
    }

    pub fn add_operation(&mut self, node: Box<dyn Node>) {
        self.operations.push(node)
    }

    pub fn set_literal(&mut self, literal: String) {
        self.literal = literal
    }
}

impl Node for ForBlock {
    fn token_literal(&self) -> String {
        self.literal.to_owned()
    }

    fn translate(&self) -> String {
        self.token_literal()
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
