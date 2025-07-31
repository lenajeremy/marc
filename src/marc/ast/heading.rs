use std::any::Any;

use super::{Block, Node, inline_container::InlineContainer};

pub struct Heading {
    // `content` is a Vec because a heading can have pure text as well as other inline nodes
    // like links, bold and italicized texts, etc. and they should all be recorded/parsed
    // independently
    inner: Box<dyn Node>,
    level: i8,
}

impl Heading {
    pub fn new(level: i8) -> Self {
        Heading {
            level,
            inner: Box::new(InlineContainer::new()),
        }
    }

    pub fn set_inner(&mut self, content: Box<dyn Node>) {
        self.inner = content;
    }
}

impl Node for Heading {
    fn token_literal(&self) -> String {
        format!(
            "Heading{}(children: {})",
            self.level,
            self.inner.token_literal()
        )
    }

    fn translate(&self) -> String {
        format!(
            "<h{}>{}</h{}>",
            self.level,
            self.inner.translate(),
            self.level
        )
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Block for Heading {
    fn block_token(&self) {
        todo!()
    }
}
