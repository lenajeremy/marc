use crate::expander::ast::{Node, expression::Expression};

pub struct ObjectAccessExpression {
    parent: Box<Expression>,
    child: Box<Expression>,
}

impl ObjectAccessExpression {
    pub fn literal(&self) -> String {
        format!(
            "ObjectExpression(value: {}.{})",
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
