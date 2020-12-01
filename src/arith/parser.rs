use crate::arith::AstNode;
use pest::Parser as PestParser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "arith/arith.pest"]
struct Parser;

pub fn parse(source: &str) -> Result<AstNode, pest::error::Error<Rule>> {
    Ok(Parser::parse(Rule::Term, source)?
        .next()
        .map(|term| term_to_ast(term))
        .unwrap())
}

fn term_to_ast(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::AppTerm => appterm_to_ast(pair),
        Rule::IfThenElse => {
            let mut pair = pair.into_inner();
            let t1 = Box::new(term_to_ast(pair.next().unwrap()));
            let t2 = Box::new(term_to_ast(pair.next().unwrap()));
            let t3 = Box::new(term_to_ast(pair.next().unwrap()));
            AstNode::IfThenElse(t1, t2, t3)
        }
        _ => unreachable!(),
    }
}

fn appterm_to_ast(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::ATerm => aterm_to_ast(pair),
        Rule::Succ => AstNode::Succ(Box::new(aterm_to_ast(pair.into_inner().next().unwrap()))),
        Rule::Pred => AstNode::Pred(Box::new(aterm_to_ast(pair.into_inner().next().unwrap()))),
        Rule::IsZero => AstNode::IsZero(Box::new(aterm_to_ast(pair.into_inner().next().unwrap()))),
        _ => unreachable!(),
    }
}

fn aterm_to_ast(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::True => AstNode::True,
        Rule::False => AstNode::False,
        Rule::Zero => AstNode::Zero,
        _ => term_to_ast(pair),
    }
}
