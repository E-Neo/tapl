use crate::arith::AstNode;
use derive_more::{Display, Error};

#[derive(Debug, Display, PartialEq)]
pub enum Type {
    Bool,
    Nat,
}

#[derive(Debug, Display, Error)]
pub enum Error {
    TypeError,
}

pub fn type_of(ast: &AstNode) -> Result<Type, Error> {
    match ast {
        AstNode::True | AstNode::False => Ok(Type::Bool),
        AstNode::IfThenElse(t1, t2, t3) => {
            if type_of(t1)? == Type::Bool {
                let typ_t2 = type_of(t2)?;
                if typ_t2 == type_of(t3)? {
                    Ok(typ_t2)
                } else {
                    Err(Error::TypeError)
                }
            } else {
                Err(Error::TypeError)
            }
        }
        AstNode::Zero => Ok(Type::Nat),
        AstNode::Succ(t1) | AstNode::Pred(t1) => {
            if type_of(t1)? == Type::Nat {
                Ok(Type::Nat)
            } else {
                Err(Error::TypeError)
            }
        }
        AstNode::IsZero(t1) => {
            if type_of(t1)? == Type::Nat {
                Ok(Type::Bool)
            } else {
                Err(Error::TypeError)
            }
        }
    }
}
