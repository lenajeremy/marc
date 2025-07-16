use super::Node;

pub struct BinaryExpression {
    left: Box<Expression>,
    right: Box<Expression>,
    operator: String,
}

impl BinaryExpression {
    fn literal(&self) -> String {
        format!(
            "{} {} {}",
            self.left.token_literal(),
            self.operator,
            self.right.token_literal()
        )
    }
}

pub struct VariableAccessExpression {
    variable_name: String,
}

impl VariableAccessExpression {
    fn literal(&self) -> String {
        self.variable_name.clone()
    }
}

pub struct ObjectAccessExpression {
    parent_variable: String,
    child_variable: String,
}

impl ObjectAccessExpression {
    fn literal(&self) -> String {
        format!("{}.{}", self.parent_variable, self.child_variable)
    }
}

pub struct ArrayAccessExpression {
    array_name: String,
    index: usize,
}

impl ArrayAccessExpression {
    fn literal(&self) -> String {
        format!("{}[{}]", self.array_name, self.index)
    }
}

pub enum Expression {
    Binary(BinaryExpression),
    VariableAccess(VariableAccessExpression),
    ObjectAccess(ObjectAccessExpression),
    ArrayAccess(ArrayAccessExpression),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Self::Binary(e) => e.literal(),
            Self::VariableAccess(e) => e.literal(),
            Self::ArrayAccess(e) => e.literal(),
            Self::ObjectAccess(e) => e.literal(),
        }
    }

    fn translate(&self) -> String {
        self.token_literal()
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}
