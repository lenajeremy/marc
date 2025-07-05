use super::{Inline, Node};

pub struct Link {
    alt_text: String,
    href: String,
}

impl Node for Link {
    fn token_literal(&self) -> String {
        format!("Link(href: \"{}\", alt: \"{}\")", self.href, self.alt_text)
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
