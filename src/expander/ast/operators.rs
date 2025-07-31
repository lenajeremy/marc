/// Comparator describes tokens that can be used to compare the values of two expressions.
/// eg, >, <, == (all used for numbers and string)
#[derive(Debug)]
pub enum Comparators {
    LessThan,
    GreaterThan,
    LessQuals,
    GreaterQuals,
    Quals,
    NeQuals,
}

/// this describes operators typically used in math related operations
#[derive(Debug)]
pub enum Math {
    Plus,
    Minus,
    Divide,
    Product,
}

#[derive(Debug)]
pub enum Op {
    Math(Math),
    Comp(Comparators),
    // Not is the boolean operator "!"
    Not,
}

impl Op {
    pub fn string(&self) -> String {
        match self {
            Op::Math(math) => match math {
                Math::Plus => "+".to_string(),
                Math::Minus => "-".to_string(),
                Math::Divide => "/".to_string(),
                Math::Product => "*".to_string(),
            },
            Op::Comp(comparators) => match comparators {
                Comparators::LessThan => "<".to_string(),
                Comparators::GreaterThan => ">".to_string(),
                Comparators::LessQuals => "<=".to_string(),
                Comparators::GreaterQuals => ">=".to_string(),
                Comparators::Quals => "==".to_string(),
                Comparators::NeQuals => "!=".to_string(),
            },
            Op::Not => "!".to_string(),
        }
    }
}
