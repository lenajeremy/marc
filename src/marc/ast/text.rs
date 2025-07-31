use std::any::Any;

use super::{Block, Inline, Node, inline_container::InlineContainer};

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

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn translate(&self) -> String {
        self.value.to_owned()
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

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn translate(&self) -> String {
        format!("<strong>{}</strong>", self.inner.translate())
    }
}

impl Inline for BoldText {
    fn inline_token(&self) {
        todo!()
    }
}

pub struct ItalicizedText {
    inner: Box<dyn Node>,
}

impl ItalicizedText {
    pub fn new() -> Self {
        ItalicizedText {
            inner: Box::new(InlineContainer::new()),
        }
    }

    pub fn set_inner(&mut self, content: Box<dyn Node>) {
        self.inner = content;
    }
}

impl Node for ItalicizedText {
    fn token_literal(&self) -> String {
        format!("Italics({})", self.inner.token_literal())
    }

    fn translate(&self) -> String {
        format!("<em>{}</em>", self.inner.translate())
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Inline for ItalicizedText {
    fn inline_token(&self) {}
}

pub struct ParagraphText {
    inner: Box<dyn Node>,
}

impl ParagraphText {
    pub fn new() -> Self {
        ParagraphText {
            inner: Box::new(InlineContainer::new()),
        }
    }

    pub fn set_inner(&mut self, content: Box<dyn Node>) {
        self.inner = content;
    }
}

impl Node for ParagraphText {
    fn token_literal(&self) -> String {
        format!("Paragraph({})", self.inner.token_literal())
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn translate(&self) -> String {
        format!("<p>{}</p>", self.inner.translate())
    }
}

impl Block for ParagraphText {
    fn block_token(&self) {}
}
