use std::any::Any;

pub mod block_quote;
pub mod code;
pub mod heading;
pub mod image;
pub mod inline_container;
pub mod link;
pub mod text;

pub trait Node: Any {
    fn token_literal(&self) -> String;
    fn translate(&self) -> String;
    fn as_any(self: Box<Self>) -> Box<dyn Any>;
}

pub trait Block: Node {
    fn block_token(&self);
}

pub trait Inline: Node {
    fn inline_token(&self);
}

pub struct Program {
    nodes: Vec<Box<dyn Node>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        self.nodes.token_literal()
    }

    fn translate(&self) -> String {
        let inside: String = self.nodes.iter().map(|node| node.translate()).collect();
        format!(
            "
            <html>
                <link href = \"./styles.css\" rel = \"stylesheet\"></link>
                <body>{}</body>
            </html>",
            inside
        )
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Node for Vec<Box<dyn Node>> {
    fn token_literal(&self) -> String {
        if self.len() > 0 {
            let literal: String = self.iter().map(|x| x.token_literal() + ",").collect();

            let len = literal.len() - 1;
            format!("[{}]", &literal[..len])
        } else {
            String::from("")
        }
    }

    fn translate(&self) -> String {
        if self.len() > 0 {
            let literal: String = self.iter().map(|x| x.translate()).collect();
            literal
        } else {
            String::from("")
        }
    }
    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Node for Vec<Box<dyn Inline>> {
    fn token_literal(&self) -> String {
        if self.len() > 0 {
            let literal: String = self.iter().map(|x| x.token_literal() + ",").collect();

            let len = literal.len() - 1;
            format!("[{}]", &literal[..len])
        } else {
            String::from("")
        }
    }

    fn translate(&self) -> String {
        if self.len() > 0 {
            let literal: String = self.iter().map(|x| x.translate()).collect();
            literal
        } else {
            String::from("")
        }
    }
    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Node for Vec<Box<dyn Block>> {
    fn token_literal(&self) -> String {
        if self.len() > 0 {
            let literal: String = self.iter().map(|x| x.token_literal() + ",").collect();

            let len = literal.len() - 1;
            format!("[{}]", &literal[..len])
        } else {
            String::from("")
        }
    }

    fn translate(&self) -> String {
        todo!()
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Program {
    pub fn new() -> Program {
        Program { nodes: vec![] }
    }

    pub fn add_block(&mut self, block: Box<dyn Node>) {
        self.nodes.push(block);
    }

    pub fn get_blocks(&self) -> &Vec<Box<dyn Node>> {
        &self.nodes
    }
}
