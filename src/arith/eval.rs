use crate::arith::AstNode;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum Error {}

pub fn eval(ast: &AstNode) -> Result<AstNode, Error> {
    match ast {
        AstNode::IfThenElse(t1, t2, t3) => match eval(t1)? {
            AstNode::True => eval(t2),
            AstNode::False => eval(t3),
            _ => Ok(ast.clone()),
        },
        AstNode::Zero => Ok(AstNode::Zero),
        AstNode::Succ(t) => Ok(AstNode::Succ(Box::new(eval(t)?))),
        AstNode::Pred(t) => match eval(t)? {
            AstNode::Zero => Ok(AstNode::Zero),
            AstNode::Succ(t) => Ok(*t),
            _ => Ok(ast.clone()),
        },
        AstNode::IsZero(t) => match eval(t)? {
            AstNode::Zero => Ok(AstNode::True),
            AstNode::Succ(_) => Ok(AstNode::False),
            _ => Ok(ast.clone()),
        },
        _ => Ok(ast.clone()),
    }
}
