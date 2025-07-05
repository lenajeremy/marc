use super::{Block, Node, inline_container::InlineContainer};

pub struct BlockQuote {
    inner: Box<dyn Node>,
}

impl BlockQuote {
    pub fn new() -> Self {
        BlockQuote {
            inner: Box::new(InlineContainer::new()),
        }
    }

    pub fn set_inner(&mut self, inner: Box<dyn Node>) {
        self.inner = inner;
    }
}

impl Node for BlockQuote {
    fn token_literal(&self) -> String {
        format!("BlockQuote(\"content: {}\")", self.inner.token_literal())
    }

    fn translate(&self) -> String {
        format!("<blockquote>{}</blockquote>", self.inner.translate())
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl Block for BlockQuote {
    fn block_token(&self) {}
}
