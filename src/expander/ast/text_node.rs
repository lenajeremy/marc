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

    fn evaluate(&self) -> String {
        self.token_literal()
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}
