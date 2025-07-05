use super::{Inline, Node};

pub struct InlineContainer {
    children: Vec<Box<dyn Node>>,
}

impl InlineContainer {
    pub fn new() -> Self {
        InlineContainer { children: vec![] }
    }

    pub fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.push(child);
    }
}

impl Node for InlineContainer {
    fn token_literal(&self) -> String {
        self.children.token_literal()
    }
}

impl Inline for InlineContainer {
    fn inline_token(&self) {}
}
