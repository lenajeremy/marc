use std::any::Any;

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

    pub fn extend(&mut self, inner: InlineContainer) {
        for child in inner.children {
            self.add_child(child);
        }
    }
}

impl Node for InlineContainer {
    fn token_literal(&self) -> String {
        self.children.token_literal()
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn translate(&self) -> String {
        self.children.translate()
    }
}

impl Inline for InlineContainer {
    fn inline_token(&self) {}
}
