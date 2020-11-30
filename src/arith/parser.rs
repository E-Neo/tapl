use crate::arith::AstNode;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "arith/arith.pest"]
struct ArithParser;

pub fn parse(source: &str) -> Result<AstNode, pest::error::Error<Rule>> {
    Ok(ArithParser::parse(Rule::Term, source)?
        .next()
        .map(|term| term_to_ast(term))
        .unwrap())
}

fn term_to_ast(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::Term => term_to_ast(pair.into_inner().next().unwrap()),
        Rule::True => AstNode::True,
        Rule::False => AstNode::False,
        Rule::IfThenElse => {
            let mut pair = pair.into_inner();
            let t1 = Box::new(term_to_ast(pair.next().unwrap()));
            let t2 = Box::new(term_to_ast(pair.next().unwrap()));
            let t3 = Box::new(term_to_ast(pair.next().unwrap()));
            AstNode::IfThenElse(t1, t2, t3)
        }
        Rule::Zero => AstNode::Zero,
        Rule::Succ => AstNode::Succ(Box::new(term_to_ast(pair.into_inner().next().unwrap()))),
        Rule::Pred => AstNode::Pred(Box::new(term_to_ast(pair.into_inner().next().unwrap()))),
        Rule::IsZero => AstNode::IsZero(Box::new(term_to_ast(pair.into_inner().next().unwrap()))),
        unknown_term => panic!("unknown term: {:?}", unknown_term),
    }
}
