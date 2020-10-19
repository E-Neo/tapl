#[derive(Debug, PartialEq)]
pub enum AstNode {
    True,
    False,
    IfThenElse(Box<AstNode>, Box<AstNode>, Box<AstNode>),
    Zero,
    Succ(Box<AstNode>),
    Pred(Box<AstNode>),
    IsZero(Box<AstNode>),
}
