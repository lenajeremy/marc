use crate::expander::ast::{Node, expression::Expression, operators::Op};

pub struct PrefixExpression {
    operator: Op,
    right: Box<Expression>,
}

impl PrefixExpression {
    pub fn literal(&self) -> String {
        format!(
            "PrefixExpression(value={}{})",
            self.operator.string(),
            self.right.token_literal()
        )
    }

    pub fn new(op: Op, right: Box<Expression>) -> Self {
        Self {
            operator: op,
            right,
        }
    }

    fn evaluate(&self) -> isize {
        // match self.operator {
        //     Op::Math(math_operator) => {
        //         let right_expression_evaluated = self.right.evaluate();
        //         match math_operator {
        //
        //         }
        //     }
        // }
        0
    }
}
