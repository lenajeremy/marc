use std::any::Any;

use crate::expander::ast::MarcNode;

use super::{
    Node,
    expression::{Expression, VariableAccessExpression},
};

pub struct IfBlock {
    expression: Expression,
    valid: Vec<Box<MarcNode>>,
    invalid: Vec<Box<MarcNode>>,
}

impl IfBlock {
    pub fn new(expression: Expression) -> IfBlock {
        IfBlock {
            expression,
            valid: vec![],
            invalid: vec![],
        }
    }

    pub fn add_valid_block(&mut self, block: Box<MarcNode>) {
        self.valid.push(block)
    }

    pub fn add_invalid_block(&mut self, block: Box<MarcNode>) {
        self.invalid.push(block)
    }
}

impl Node for IfBlock {
    fn token_literal(&self) -> String {
        let valid_literal = if self.valid.is_empty() {
            "[]".to_string()
        } else {
            let inner: String = self.valid.iter().map(|x| x.token_literal() + ",").collect();
            format!("[{}]", &inner[..inner.len() - 1])
        };
        let invalid_literal = if self.invalid.is_empty() {
            "[]".to_string()
        } else {
            let inner: String = self.invalid.iter().map(|x| x.token_literal() + ",").collect();
            format!("[{}]", &inner[..inner.len() - 1])
        };
        format!(
            "IfBlock(condition={}, valid={}, invalid={})",
            self.expression.token_literal(),
            valid_literal,
            invalid_literal
        )
    }

    fn evaluate(&self) -> String {
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
}

impl ForBlock {
    pub fn new(list: Expression, variable: VariableAccessExpression) -> ForBlock {
        ForBlock {
            main_list: list,
            variable,
            operations: vec![],
        }
    }

    pub fn add_operation(&mut self, node: Box<dyn Node>) {
        self.operations.push(node)
    }
}

impl Node for ForBlock {
    fn token_literal(&self) -> String {
        let ops_literal = if self.operations.is_empty() {
            "[]".to_string()
        } else {
            let inner: String = self
                .operations
                .iter()
                .map(|x| x.token_literal() + ",")
                .collect();
            format!("[{}]", &inner[..inner.len() - 1])
        };
        format!(
            "ForBlock(variable={}, list={}, body={})",
            self.variable.literal(),
            self.main_list.token_literal(),
            ops_literal
        )
    }

    fn evaluate(&self) -> String {
        self.token_literal()
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
