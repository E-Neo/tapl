use crate::arith::AstNode;

pub fn eval(ast: &AstNode) -> Result<AstNode, ()> {
    match ast {
        AstNode::True => Ok(AstNode::True),
        AstNode::False => Ok(AstNode::False),
        AstNode::IfThenElse(t1, t2, t3) => match eval(t1)? {
            AstNode::True => eval(t2),
            AstNode::False => eval(t3),
            _ => Err(()),
        },
        AstNode::Zero => Ok(AstNode::Zero),
        AstNode::Succ(t) => Ok(AstNode::Succ(Box::new(eval(t)?))),
        AstNode::Pred(t) => match eval(t)? {
            AstNode::Zero => Ok(AstNode::Zero),
            AstNode::Succ(t) => Ok(*t),
            _ => Err(()),
        },
        AstNode::IsZero(t) => match eval(t)? {
            AstNode::Zero => Ok(AstNode::True),
            AstNode::Succ(_) => Ok(AstNode::False),
            _ => Err(()),
        },
    }
}
