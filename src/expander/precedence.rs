// public class Precedence {
//   public static final int ASSIGNMENT  = 1;
//   public static final int CONDITIONAL = 2;
//   public static final int SUM         = 3;
//   public static final int PRODUCT     = 4;
//   public static final int EXPONENT    = 5;
//   public static final int PREFIX      = 6;
//   public static final int POSTFIX     = 7;
//   public static final int CALL        = 8;
// }

#[repr(u8)]
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Precendence {
    EMPTY, // this is to take up the zeroth index so no useful token has an index of 0
    VARIABLE,
    ASSIGNMENT,
    SUM,
    PRODUCT,
    PREFIX,
    ARRAYACCESS,
    OBJECTACCESS,
    // POSTFIX,
    CALL,
}
