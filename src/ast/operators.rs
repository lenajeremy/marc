/// Comparator describes tokens that can be used to compare the values of two expressions.
/// eg, >, <, = (all used for numbers and string)
#[derive(Debug)]
pub enum Comparators {
    LessThan,
    GreaterThan,
    Quals,
}

/// this describes operators typically used in math related operations
#[derive(Debug)]
pub enum Math {
    LessThan,
    GreaterThan,
    Quals,
}

#[derive(Debug)]
pub enum Op {
    Math(Math),
    Comp(Comparators),
}
