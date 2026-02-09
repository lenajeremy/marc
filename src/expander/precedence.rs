#[repr(u8)]
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Precedence {
    EMPTY, // this is to take up the zeroth index so no useful token has an index of 0
    VARIABLE,
    ASSIGNMENT,
    COMPARISON,
    SUM,
    PRODUCT,
    PREFIX,
    ArrayAccess,
    OBJECTACCESS,
    // POSTFIX,
    CALL,
}
