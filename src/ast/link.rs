use std::any::Any;

use super::{Inline, Node};

pub struct Link {
    alt_text: String,
    href: String,
}

impl Node for Link {
    fn token_literal(&self) -> String {
        format!("Link(href: \"{}\", alt: \"{}\")", self.href, self.alt_text)
    }

    fn translate(&self) -> String {
        format!("<a href = \"{}\">{}</a>", self.href, self.alt_text)
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Link {
    pub fn new(alt_text: String, href: String) -> Self {
        Link { alt_text, href }
    }
}

impl Inline for Link {
    fn inline_token(&self) {}
}
