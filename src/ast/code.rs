use std::any::Any;

use crate::Block;

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

pub struct CodeBlock {
    code_content: String,
    language: String,
}

impl CodeBlock {
    pub fn new(content: String, language: String) -> Self {
        CodeBlock {
            code_content: content,
            language,
        }
    }
}

impl Node for CodeBlock {
    fn token_literal(&self) -> String {
        format!(
            "CodeBlock(\"{}\", lang={})",
            self.code_content, self.language
        )
    }

    fn translate(&self) -> String {
        format!("<pre>{}</pre>", self.code_content)
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Block for CodeBlock {
    fn block_token(&self) {}
}
