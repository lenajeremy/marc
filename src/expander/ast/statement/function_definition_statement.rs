use crate::expander::ast::{Node, statement::ReturnStatement};

pub struct FunctionDefinitionStatement {
    name: String,
    params: Vec<String>,
    body: Vec<Box<dyn Node>>,
    return_statement: Option<ReturnStatement>,
}

impl FunctionDefinitionStatement {
    pub fn new(
        name: String,
        params: Vec<String>,
        body: Vec<Box<dyn Node>>,
        return_statement: Option<ReturnStatement>,
    ) -> Self {
        Self {
            name,
            params,
            body,
            return_statement,
        }
    }

    pub fn literal(&self) -> String {
        let params = self.params.join(", ");
        let body_literal = if self.body.is_empty() {
            "[]".to_string()
        } else {
            let inner: String = self.body.iter().map(|x| x.token_literal() + ",").collect();
            format!("[{}]", &inner[..inner.len() - 1])
        };
        let return_literal = match &self.return_statement {
            Some(stmt) => stmt.literal(),
            None => "None".to_string(),
        };
        format!(
            "FunctionDefinitionStatement(\"fn {}({}) {{ body={}, return={} }}\")",
            self.name, params, body_literal, return_literal
        )
    }
}

impl Node for FunctionDefinitionStatement {
    fn token_literal(&self) -> String {
        self.literal()
    }

    fn translate(&self) -> String {
        self.token_literal()
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}
