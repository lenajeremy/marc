use super::Node;

pub struct Expression {
    literal: String,
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        format!("Expression(literal={}", self.literal)
    }

    fn translate(&self) -> String {
        todo!()
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}
