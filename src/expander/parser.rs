use crate::expander::{
    ast::{
        Document, MarcNode,
        expression::{Expression, VariableAccessExpression},
        marcblocks::ForBlock,
        text_node::TextNode,
    },
    lexer::Lexer,
    token::{Token, TokenType as TT},
};

pub struct Parser {
    curr_token: Token,
    next_token: Token,
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            curr_token: Token::new(TT::EOF, String::new(), 0, 0),
            next_token: Token::new(TT::EOF, String::new(), 0, 0),
            lexer,
        };
        parser.advance_token();
        parser.advance_token();
        parser
    }

    fn advance_token(&mut self) {
        self.curr_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Box<MarcNode> {
        let marcnode = match self.curr_token.token_type {
            TT::Text => MarcNode::Text(TextNode::new(self.curr_token.literal.clone())),
            TT::KeywordStart => {
                let next_token = self.next_token.clone();
                match next_token.token_type {
                    TT::For => {
                        let for_block = self.parse_for_block();
                        MarcNode::For(for_block)
                    }
                    _ => {
                        let for_block = self.parse_for_block();
                        MarcNode::For(for_block)
                    }
                }
            }
            _ => MarcNode::Expression(Expression::Empty),
        };
        self.advance_token();
        Box::new(marcnode)
    }

    fn parse_for_block(&mut self) -> ForBlock {
        ForBlock::new(
            Expression::VariableAccess(VariableAccessExpression::new("products".to_string())),
            VariableAccessExpression::new("product".to_string()),
        )
    }

    pub fn parse_document(&mut self) -> Document {
        let mut program = Document::new();

        while self.curr_token.token_type != TT::EOF {
            let node = self.parse();
            program.add_node(node);
            self.advance_token();
        }

        program
    }
}
