use crate::expander::ast::{Node, expression::Expression};

pub struct ArrayAccessExpression {
    parent: Box<Expression>,
    index: Box<Expression>,
}

impl ArrayAccessExpression {
    pub fn literal(&self) -> String {
        format!(
            "ArrayAccess({}[{}])",
            self.parent.token_literal(),
            self.index.token_literal()
        )
    }

    pub fn new(parent: Box<Expression>, index: Box<Expression>) -> Self {
        Self { index, parent }
    }
}
