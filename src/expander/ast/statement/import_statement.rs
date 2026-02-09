pub struct ImportStatement {
    src: String,
    alias: String,
}

impl ImportStatement {
    pub fn new(src: String, alias: String) -> Self {
        ImportStatement { src, alias }
    }

    pub fn literal(&self) -> String {
        format!(
            "ImportStatement(src=\"{}\", alias=\"{}\")",
            self.src, self.alias
        )
    }
}

impl crate::expander::ast::Node for ImportStatement {
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
