use std::any::Any;

use super::{Inline, Node};

pub struct InlineCode {
    code_content: String,
}

impl InlineCode {
    pub fn new(content: String) -> Self {
        InlineCode {
            code_content: content,
        }
    }
}

impl Node for InlineCode {
    fn token_literal(&self) -> String {
        format!("InlineCode(\"{}\")", self.code_content)
    }

    fn translate(&self) -> String {
        format!("<code>{}</code>", self.code_content)
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Inline for InlineCode {
    fn inline_token(&self) {}
}
