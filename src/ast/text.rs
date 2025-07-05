use super::{Inline, Node, inline_container::InlineContainer};

pub struct Text {
    value: String,
}

impl Text {
    pub fn new(value: String) -> Self {
        Text { value: value }
    }
}

impl Node for Text {
    fn token_literal(&self) -> String {
        format!("Text(\"{}\")", self.value.clone())
    }
}

impl Inline for Text {
    fn inline_token(&self) {
        todo!()
    }
}

pub struct BoldText {
    inner: Box<dyn Node>,
}

impl BoldText {
    pub fn new() -> Self {
        BoldText {
            inner: Box::new(InlineContainer::new()),
        }
    }

    pub fn set_inner(&mut self, content: Box<dyn Node>) {
        self.inner = content;
    }
}

impl Node for BoldText {
    fn token_literal(&self) -> String {
        format!("Bold({})", self.inner.token_literal())
    }
}

impl Inline for BoldText {
    fn inline_token(&self) {
        todo!()
    }
}
