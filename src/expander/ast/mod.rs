use std::any::Any;

pub mod expression;
pub mod marcblocks;
pub mod operators;
pub mod text_node;
pub mod statement;

pub trait Node: Any {
    fn token_literal(&self) -> String;
    fn evaluate(&self) -> String;
    fn as_any(self: Box<Self>) -> Box<dyn Any>;
}

pub enum MarcNode {
    If(marcblocks::IfBlock),
    For(marcblocks::ForBlock),
    Import(statement::ImportStatement),
    Text(text_node::TextNode),
    Expression(Box<expression::Expression>),
    FunctionDefinition(statement::FunctionDefinitionStatement),
    Statement(Box<dyn Node>),
}

impl Node for MarcNode {
    fn token_literal(&self) -> String {
        match self {
            MarcNode::For(b) => b.token_literal(),
            MarcNode::If(b) => b.token_literal(),
            MarcNode::Text(b) => b.token_literal(),
            MarcNode::Expression(b) => b.token_literal(),
            MarcNode::FunctionDefinition(b) => b.token_literal(),
            MarcNode::Import(b) => b.token_literal(),
            MarcNode::Statement(b) => b.token_literal(),
        }
    }

    fn evaluate(&self) -> String {
        self.token_literal()
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

pub struct Document {
    nodes: Vec<Box<dyn Node>>,
}

impl Node for Document {
    fn token_literal(&self) -> String {
        self.nodes.token_literal()
    }

    fn evaluate(&self) -> String {
        self.nodes.evaluate()
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Document {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn add_node(&mut self, block: Box<dyn Node>) {
        self.nodes.push(block);
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

    fn evaluate(&self) -> String {
        if self.len() > 0 {
            let literal: String = self.iter().map(|x| x.evaluate()).collect();
            literal
        } else {
            String::from("")
        }
    }
    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
