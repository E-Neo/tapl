use derive_more::Display;

#[derive(Debug, Display, Clone, PartialEq)]
pub enum AstNode {
    True,
    False,
    #[display(fmt = "IfThenElse({}, {}, {})", _0, _1, _2)]
    IfThenElse(Box<AstNode>, Box<AstNode>, Box<AstNode>),
    Zero,
    #[display(fmt = "Succ({})", _0)]
    Succ(Box<AstNode>),
    #[display(fmt = "Pred({})", _0)]
    Pred(Box<AstNode>),
    #[display(fmt = "IsZero({})", _0)]
    IsZero(Box<AstNode>),
}
