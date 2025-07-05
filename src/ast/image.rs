use super::{Block, Node};

pub struct Image {
    src: String,
    alt: String,
}

impl Image {
    pub fn new(src: String, alt: String) -> Self {
        Image { src, alt }
    }
}
impl Node for Image {
    fn token_literal(&self) -> String {
        format!("Image(src = \"{}\", alt = \"{}\")", self.src, self.alt)
    }
}

impl Block for Image {
    fn block_token(&self) {}
}
