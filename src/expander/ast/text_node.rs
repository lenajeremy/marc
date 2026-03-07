use crate::expander::environment::Environment;

use super::Node;

#[derive(Clone)]
pub struct TextNode {
    text: String,
}

impl TextNode {
    pub fn new(text: String) -> Self {
        TextNode { text }
    }
}

impl Node for TextNode {
    fn token_literal(&self) -> String {
        format!("Text(\"{}\")", self.text.clone())
    }

    fn translate(&self, _env: &mut Environment) -> String {
        self.text.clone()
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}
