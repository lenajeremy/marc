use super::{
    Node,
    expression::{Expression, VariableAccessExpression},
};

pub struct IfBlock {
    expression: Expression,
    valid: Vec<Expression>,
    invalid: Vec<Expression>,
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

    fn add_valid_block(&mut self, block: Expression) {
        self.valid.push(block)
    }

    fn add_invalid_block(&mut self, block: Expression) {
        self.invalid.push(block)
    }

    fn set_literal(&mut self, literal: String) {
        self.literal = literal
    }
}

pub struct ForBlock {
    main_list: VariableAccessExpression,
    variable: VariableAccessExpression,
    operations: Vec<Expression>,
    literal: String,
}

impl ForBlock {
    pub fn new(list: VariableAccessExpression, variable: VariableAccessExpression) -> ForBlock {
        ForBlock {
            main_list: list,
            variable: variable,
            operations: vec![],
            literal: String::new(),
        }
    }

    pub fn add_operation(&mut self, expression: Expression) {
        self.operations.push(expression)
    }

    pub fn set_literal(&mut self, literal: String) {
        self.literal = literal
    }
}

/// MarcBlock describes blocks that are unrelated to markdown. MarcBlocks are specific to
/// code/template related blocks, e.g. if, for, while,etc blocks
pub enum MarcBlock {
    If(IfBlock),
    For(ForBlock),
}

impl Node for MarcBlock {
    fn token_literal(&self) -> String {
        match self {
            MarcBlock::For(b) => b.literal.clone(),
            MarcBlock::If(b) => b.literal.clone(),
        }
    }

    fn translate(&self) -> String {
        self.token_literal()
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}
