use tapl::arith::{eval, parse, AstNode};

fn eval_line(source: &str) -> AstNode {
    let nodes = parse(source).unwrap();
    assert!(nodes.len() == 1);
    eval(nodes.first().unwrap()).unwrap()
}

#[test]
fn test() {
    assert_eq!(eval_line("true"), AstNode::True);
    assert_eq!(eval_line("if false then true else false"), AstNode::False);
    assert_eq!(eval_line("0"), AstNode::Zero);
    assert_eq!(
        eval_line("succ (pred 0)"),
        AstNode::Succ(Box::new(AstNode::Zero))
    );
    assert_eq!(eval_line("iszero (pred (succ (succ 0)))"), AstNode::False);
}
