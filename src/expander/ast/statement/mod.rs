pub mod function_definition_statement;
pub mod import_statement;
pub mod return_statement;
pub mod variable_assignment_statement;

use crate::expander::ast::Node;
pub use function_definition_statement::*;
pub use import_statement::*;
pub use return_statement::*;
use std::any::Any;
pub use variable_assignment_statement::*;

enum Statement {
    Import(ImportStatement),
    VariableAssignment(VariableAssignmentStatement),
    FunctionDefinition(FunctionDefinitionStatement),
    Return(ReturnStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Self::VariableAssignment(statement) => statement.literal(),
            Self::Import(statement) => statement.literal(),
            Self::FunctionDefinition(statement) => statement.literal(),
            Self::Return(statement) => statement.literal(),
        }
    }

    fn translate(&self) -> String {
        self.token_literal()
    }

    fn as_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
