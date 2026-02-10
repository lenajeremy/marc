use crate::expander::{
    ast::{
        Document, MarcNode, Node,
        expression::{Expression, VariableAccessExpression},
        marcblocks::ForBlock,
        statement::{FunctionDefinitionStatement, ReturnStatement, VariableAssignmentStatement},
        text_node::TextNode,
    },
    lexer::Lexer,
    parselets::{
        InfixParselet, PrefixParselet, array_parselets::ArrayParselet,
        bracket_expression_parselets::GroupedExpressionParselet,
        function_call_parselet::FunctionCallParselet, integer_parselet::IntegerParselet,
        object_parselets::ObjectNotationParselet, operator_infix_parselets::OperatorInfixParselet,
        operator_prefix_parselets::OperatorPrefixParselet, string_parselet::StringParselet,
        variable_access_parselet::VariableAccessParselet,
    },
    token::{Token, TokenType as TT},
};
use std::collections::HashMap;
use std::panic;
use crate::expander::parselets::boolean_parselet::BooleanParselet;

pub struct Parser {
    curr_token: Token,
    next_token: Token,
    lexer: Lexer,
    prefix_parselets: HashMap<TT, &'static dyn PrefixParselet>,
    infix_parselets: HashMap<TT, &'static dyn InfixParselet>,
}

static VARIABLE_ACCESS_PARSELET: VariableAccessParselet = VariableAccessParselet;
static OPERATOR_PREFIX_PARSELET: OperatorPrefixParselet = OperatorPrefixParselet;
static INTEGER_PARSELET: IntegerParselet = IntegerParselet;
static STRING_PARSELET: StringParselet = StringParselet;
static GROUPED_OPERATION_PARSELET: GroupedExpressionParselet = GroupedExpressionParselet;
static BOOLEAN_PREFIX_PARSELET: BooleanParselet = BooleanParselet;
static OPERATOR_INFIX_PARSELET: OperatorInfixParselet = OperatorInfixParselet;
static FUNCTION_CALL_PARSELET: FunctionCallParselet = FunctionCallParselet;
static ARRAY_PARSELET: ArrayParselet = ArrayParselet;
static OBJECT_NOTATION_PARSELET: ObjectNotationParselet = ObjectNotationParselet;

impl Parser {
    fn register_prefix_parselet(&mut self, tt: TT, parselet: &'static dyn PrefixParselet) {
        self.prefix_parselets.insert(tt, parselet);
    }

    fn register_infix_parselet(&mut self, tt: TT, parselet: &'static dyn InfixParselet) {
        self.infix_parselets.insert(tt, parselet);
    }

    pub fn peek_token(&self) -> Token {
        self.next_token.clone()
    }

    pub fn get_curr_token(&self) -> Token {
        self.curr_token.clone()
    }

    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            curr_token: Token::new(TT::EOF, String::new(), 0, 0),
            next_token: Token::new(TT::EOF, String::new(), 0, 0),
            lexer,
            prefix_parselets: HashMap::new(),
            infix_parselets: HashMap::new(),
        };

        parser.advance_token();
        parser.advance_token();

        // register prefix parselets
        parser.register_prefix_parselet(TT::Identifier, &VARIABLE_ACCESS_PARSELET);
        parser.register_prefix_parselet(TT::Plus, &OPERATOR_PREFIX_PARSELET);
        parser.register_prefix_parselet(TT::Minus, &OPERATOR_PREFIX_PARSELET);
        parser.register_prefix_parselet(TT::Exclamation, &OPERATOR_PREFIX_PARSELET);
        parser.register_prefix_parselet(TT::Integer, &INTEGER_PARSELET);
        parser.register_prefix_parselet(TT::DoubleQuote, &STRING_PARSELET);
        parser.register_prefix_parselet(TT::SingleQuote, &STRING_PARSELET);
        parser.register_prefix_parselet(TT::LeftParen, &GROUPED_OPERATION_PARSELET);
        parser.register_prefix_parselet(TT::True, &BOOLEAN_PREFIX_PARSELET);
        parser.register_prefix_parselet(TT::False, &BOOLEAN_PREFIX_PARSELET);

        // register infix parselets
        parser.register_infix_parselet(TT::Plus, &OPERATOR_INFIX_PARSELET);
        parser.register_infix_parselet(TT::Minus, &OPERATOR_INFIX_PARSELET);
        parser.register_infix_parselet(TT::ForwardSlash, &OPERATOR_INFIX_PARSELET);
        parser.register_infix_parselet(TT::Asterisk, &OPERATOR_INFIX_PARSELET);
        parser.register_infix_parselet(TT::GreaterThan, &OPERATOR_INFIX_PARSELET);
        parser.register_infix_parselet(TT::LessThan, &OPERATOR_INFIX_PARSELET);
        parser.register_infix_parselet(TT::GreQual, &OPERATOR_INFIX_PARSELET);
        parser.register_infix_parselet(TT::LeQual, &OPERATOR_INFIX_PARSELET);
        parser.register_infix_parselet(TT::Equals, &OPERATOR_INFIX_PARSELET);
        parser.register_infix_parselet(TT::NeQual, &OPERATOR_INFIX_PARSELET);

        parser.register_infix_parselet(TT::Dot, &OBJECT_NOTATION_PARSELET);
        parser.register_infix_parselet(TT::LeftBracket, &ARRAY_PARSELET);
        parser.register_infix_parselet(TT::LeftParen, &FUNCTION_CALL_PARSELET);

        parser
    }

    pub fn advance_token(&mut self) {
        self.curr_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn parse(&mut self) -> Box<MarcNode> {
        let marcnode = match self.curr_token.token_type {
            TT::Text | TT::NewLine => {
                MarcNode::Text(TextNode::new(self.curr_token.literal.clone()))
            }
            TT::At => {
                let import_statement = self.parse_import_statement();
                MarcNode::Import(import_statement)
            }
            TT::KeywordStart => {
                let next_token = self.next_token.clone();
                match next_token.token_type {
                    TT::For => {
                        let for_block = self.parse_for_block();
                        MarcNode::For(for_block)
                    }
                    TT::If => {
                        let if_block = self.parse_if_block();
                        MarcNode::If(if_block)
                    }
                    TT::Fn => {
                        let function_definition = self.parse_fn_block();
                        MarcNode::FunctionDefinition(function_definition)
                    }
                    _ => MarcNode::Expression(Box::new(Expression::Empty)),
                }
            }
            TT::LeftDoubleBrace => {
                self.advance_token();
                MarcNode::Expression(self.parse_expression(0))
            },
            TT::True => {
                MarcNode::Expression(Box::new(Expression::True))
            },
            TT::False => {
                MarcNode::Expression(Box::new(Expression::False))
            },
            _ => MarcNode::Expression(Box::new(Expression::Empty)),
        };
        self.advance_token();
        Box::new(marcnode)
    }

    pub fn get_precedence(&self, token: &Token) -> u8 {
        match self.infix_parselets.get(&token.token_type) {
            Some(parselet) => parselet.get_precedence(token.clone()),
            None => 0,
        }
    }

    pub fn peek_precedence(&self) -> u8 {
        self.get_precedence(&self.next_token)
    }

    pub fn parse_expression(&mut self, precedence: u8) -> Box<Expression> {
        let cursor_details = self.lexer.get_cursor();
        let prefix_parselet = *self
            .prefix_parselets
            .get(&self.curr_token.token_type)
            .unwrap_or_else(|| {
                panic!(
                    "failed to parse expression. got {:?}, line: {}, column: {}",
                    self.curr_token, cursor_details.0, cursor_details.1
                );
            });

        let mut left = prefix_parselet.parse_expression(self, self.curr_token.clone());

        while precedence < self.peek_precedence() {
            self.advance_token();

            let infix_parselet = self
                .infix_parselets
                .get(&self.curr_token.token_type)
                .clone();

            left = match infix_parselet {
                Some(parselet) => {
                    let right = parselet.parse_expression(self, left);
                    right
                }
                _ => left,
            }
        }

        left
    }

    fn parse_for_block(&mut self) -> ForBlock {
        self.advance_token(); // move to `for`
        if self.curr_token.token_type != TT::For {
            panic!("expected `for` after keyword start");
        }

        self.advance_token(); // move to loop variable
        if self.curr_token.token_type != TT::Identifier {
            panic!("expected identifier for loop variable");
        }
        let variable = VariableAccessExpression::new(self.curr_token.literal.clone());

        self.advance_token(); // move to `in`
        if self.curr_token.token_type != TT::In {
            panic!("expected `in` after loop variable");
        }

        self.advance_token(); // move to list expression
        let list_expression = self.parse_expression(0);

        if self.next_token.token_type != TT::KeywordEnd {
            panic!("expected `%}}` after for clause");
        }
        self.advance_token(); // move to `%}`
        self.advance_token(); // move to first token after `%}`

        let mut for_block = ForBlock::new(*list_expression, variable);

        while !(self.curr_token.token_type == TT::KeywordStart
            && self.next_token.token_type == TT::EndFor)
        {
            let node = self.parse();
            if node.token_literal() != Expression::Empty.token_literal() {
                for_block.add_operation(node);
            }
        }

        self.advance_token(); // move to `endfor`
        self.advance_token(); // move to `%}`

        for_block
    }

    fn parse_if_block(&mut self) -> crate::expander::ast::marcblocks::IfBlock {
        self.advance_token(); // move to `if`
        if self.curr_token.token_type != TT::If {
            panic!("expected `if` after keyword start");
        }

        self.advance_token(); // move to first expression token
        let condition = self.parse_expression(0);

        if self.next_token.token_type != TT::KeywordEnd {
            panic!("expected `%}}` after if condition");
        }
        self.advance_token(); // move to `%}`
        self.advance_token(); // move to first token after `%}`

        let mut if_block = crate::expander::ast::marcblocks::IfBlock::new(*condition);

        while !(self.curr_token.token_type == TT::KeywordStart
            && self.next_token.token_type == TT::EndIf)
        {
            let node = self.parse();
            if node.token_literal() != Expression::Empty.token_literal() {
                if_block.add_valid_block(node);
            }
        }

        self.advance_token(); // move to `endif`
        self.advance_token(); // move to `%}`

        if_block
    }

    fn parse_import_statement(&mut self) -> crate::expander::ast::statement::ImportStatement {
        self.advance_token(); // move to `import`
        if self.curr_token.token_type != TT::Import {
            panic!("expected `import` after `@`");
        }

        self.advance_token(); // move to opening quote
        let quote_token = self.curr_token.token_type.clone();
        if quote_token != TT::DoubleQuote && quote_token != TT::SingleQuote {
            panic!("expected quote after import keyword");
        }

        let src = self.parse_quoted_string(quote_token);
        if self.next_token.token_type != TT::As {
            panic!("expected `as` after import path");
        }
        self.advance_token(); // move to `as`
        self.advance_token(); // move to alias

        if self.curr_token.token_type != TT::Identifier {
            panic!("expected identifier after `as`");
        }
        let alias = self.curr_token.literal.clone();

        crate::expander::ast::statement::ImportStatement::new(src, alias)
    }

    fn parse_fn_block(&mut self) -> FunctionDefinitionStatement {
        self.advance_token(); // move to `fn`
        if self.curr_token.token_type != TT::Fn {
            panic!("expected `fn` after keyword start");
        }

        self.advance_token(); // move to function name
        if self.curr_token.token_type != TT::Identifier {
            panic!("expected identifier for function name");
        }
        let name = self.curr_token.literal.clone();

        self.advance_token(); // move to `(`
        if self.curr_token.token_type != TT::LeftParen {
            panic!("expected `(` after function name");
        }

        let mut params: Vec<String> = Vec::new();
        self.advance_token(); // move to first param or `)`
        while self.curr_token.token_type != TT::RightParen {
            match self.curr_token.token_type {
                TT::Identifier => params.push(self.curr_token.literal.clone()),
                TT::Comma => {}
                _ => panic!("unexpected token in function parameter list"),
            }
            self.advance_token();
        }

        self.advance_token(); // move to `%}`
        if self.curr_token.token_type != TT::KeywordEnd {
            panic!("expected `%}}` after function signature");
        }

        self.advance_token(); // move to first token in body
        while self.curr_token.token_type == TT::NewLine {
            self.advance_token();
        }

        let mut body: Vec<Box<dyn Node>> = Vec::new();
        let mut return_statement: Option<ReturnStatement> = None;

        while !(self.curr_token.token_type == TT::KeywordStart
            && self.next_token.token_type == TT::EndFn)
        {
            if self.curr_token.token_type == TT::KeywordStart
                && self.next_token.token_type == TT::Return
            {
                self.advance_token(); // move to `return`
                self.advance_token(); // move to first expression token
                let return_expression = self.parse_expression(0);

                if self.next_token.token_type != TT::KeywordEnd {
                    panic!("expected `%}}` after return expression");
                }
                self.advance_token(); // move to `%}`
                self.advance_token(); // move to next token after return block

                return_statement = Some(ReturnStatement::new(return_expression));
                continue;
            }

            if let Some(return_stmt) = self.try_parse_return_from_text() {
                return_statement = Some(return_stmt);
                self.advance_token();
                continue;
            }

            let node = self.parse_block_node();
            if node.token_literal() != Expression::Empty.token_literal() {
                body.push(node);
            }
        }

        self.advance_token(); // move to `endfn`
        self.advance_token(); // move to `%}`
        if self.curr_token.token_type != TT::KeywordEnd {
            panic!("expected `%}}` after endfn");
        }

        FunctionDefinitionStatement::new(name, params, body, return_statement)
    }

    pub fn parse_quoted_string(&mut self, quote_token: TT) -> String {
        if self.curr_token.token_type != quote_token {
            panic!("expected quote token for string literal");
        }

        let mut literal = String::new();
        self.advance_token(); // move to first token inside quotes

        while self.curr_token.token_type != quote_token && self.curr_token.token_type != TT::EOF {
            literal.push_str(self.curr_token.literal.as_str());
            self.advance_token();
        }

        if self.curr_token.token_type != quote_token {
            panic!("unterminated string literal");
        }

        literal
    }

    fn parse_block_node(&mut self) -> Box<MarcNode> {
        match self.curr_token.token_type {
            TT::Text => {
                let text = self.curr_token.literal.clone();
                let node = self.parse_text_as_node(&text);
                self.advance_token();
                node
            }
            TT::NewLine => {
                let text = self.curr_token.literal.clone();
                let node = self.parse_text_as_node(&text);
                self.advance_token();
                node
            }
            TT::LeftDoubleBrace | TT::KeywordStart | TT::At => self.parse(),
            _ => {
                let node = MarcNode::Expression(Box::new(Expression::Empty));
                self.advance_token();
                Box::new(node)
            }
        }
    }

    fn parse_text_as_node(&self, text: &str) -> Box<MarcNode> {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return Box::new(MarcNode::Expression(Box::new(Expression::Empty)));
        }

        if let Some(statement) = self.try_parse_assignment_from_text(trimmed) {
            return Box::new(MarcNode::Statement(statement));
        }

        if let Some(return_stmt) = self.try_parse_return_from_text() {
            return Box::new(MarcNode::Statement(Box::new(return_stmt)));
        }

        if self.is_expression_candidate(trimmed) {
            if let Some(expr) = self.try_parse_expression_from_str(trimmed) {
                return Box::new(MarcNode::Expression(expr));
            }
        }

        Box::new(MarcNode::Text(TextNode::new(text.to_string())))
    }

    fn is_expression_candidate(&self, text: &str) -> bool {
        if text.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return true;
        }
        text.chars().any(|c| {
            matches!(
                c,
                '(' | ')' | '.' | '[' | ']' | '+' | '-' | '*' | '/' | '!' | '<' | '>' | '='
            ) || c.is_numeric()
        })
    }

    fn try_parse_expression_from_str(&self, text: &str) -> Option<Box<Expression>> {
        let expr = text.trim();
        let result = panic::catch_unwind(|| {
            let mut lexer = Lexer::from(expr);
            lexer.set_detailed(true);
            let mut parser = Parser::new(lexer);
            parser.parse_expression(0)
        });
        result.ok()
    }

    fn try_parse_assignment_from_text(&self, text: &str) -> Option<Box<dyn Node>> {
        if let Some((op, lhs, rhs)) = self.split_assignment(text) {
            if !self.is_identifier(lhs) {
                return None;
            }
            let rhs_expr = self.try_parse_expression_from_str(rhs)?;
            let value = match op {
                "+=" => Box::new(Expression::OperatorInfix(
                    crate::expander::ast::expression::InfixExpression::new(
                        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
                            lhs.to_string(),
                        ))),
                        rhs_expr,
                        crate::expander::ast::operators::Op::Math(
                            crate::expander::ast::operators::Math::Plus,
                        ),
                    ),
                )),
                "=" => rhs_expr,
                _ => return None,
            };
            return Some(Box::new(VariableAssignmentStatement::new(
                lhs.to_string(),
                value,
            )));
        }
        None
    }

    fn try_parse_return_from_text(&self) -> Option<ReturnStatement> {
        if self.curr_token.token_type != TT::Text {
            return None;
        }
        let trimmed = self.curr_token.literal.trim();
        if let Some(rest) = trimmed.strip_prefix("return ") {
            let expr = self.try_parse_expression_from_str(rest)?;
            return Some(ReturnStatement::new(expr));
        }
        None
    }

    fn is_identifier(&self, text: &str) -> bool {
        let mut chars = text.chars();
        let first = match chars.next() {
            Some(c) => c,
            None => return false,
        };
        if !(first == '_' || first.is_alphabetic()) {
            return false;
        }
        chars.all(|c| c.is_alphanumeric() || c == '_')
    }

    fn split_assignment<'a>(&self, text: &'a str) -> Option<(&'static str, &'a str, &'a str)> {
        if let Some((lhs, rhs)) = text.split_once("+=") {
            return Some(("+=", lhs.trim(), rhs.trim()));
        }

        let mut prev: Option<char> = None;
        let chars: Vec<char> = text.chars().collect();
        for (i, c) in chars.iter().enumerate() {
            if *c != '=' {
                prev = Some(*c);
                continue;
            }
            let next = chars.get(i + 1).copied();
            if next == Some('=') || prev == Some('=') {
                prev = Some(*c);
                continue;
            }
            if matches!(prev, Some('!') | Some('>') | Some('<') | Some('+')) {
                prev = Some(*c);
                continue;
            }
            let (lhs, rhs) = text.split_at(i);
            return Some(("=", lhs.trim(), rhs[1..].trim()));
        }
        None
    }

    pub fn parse_document(&mut self) -> Document {
        let mut program = Document::new();

        while self.curr_token.token_type != TT::EOF {
            let node = self.parse();
            // println!(
            //     "parsing node: {:?}
            //     curr_token: {:?}",
            //     node.token_literal(),
            //     self.curr_token()
            // );
            if node.token_literal() != Expression::Empty.token_literal() {
                program.add_node(node);
            }
            // self.advance_token();
        }

        program
    }
}
