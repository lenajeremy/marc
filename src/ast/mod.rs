pub mod heading;
pub mod image;
pub mod inline_container;
pub mod link;
pub mod text;

pub trait Node {
    fn token_literal(&self) -> String;
    //fn translate(&self) -> String;
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
