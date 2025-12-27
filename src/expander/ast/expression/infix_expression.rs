use super::{super::operators::Op, Expression, Node};

pub struct InfixExpression {
    left: Box<Expression>,
    right: Box<Expression>,
    operator: Op,
}

impl InfixExpression {
    pub fn literal(&self) -> String {
        format!(
            "InfixExpression({} {:?} {})",
            self.left.token_literal(),
            self.operator,
            self.right.token_literal()
        )
    }

    pub fn new(left: Box<Expression>, right: Box<Expression>, operator: Op) -> Self {
        Self {
            left,
            right,
            operator,
        }
    }
}
