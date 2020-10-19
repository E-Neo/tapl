use std::io::{BufRead, Write};
use tapl::arith::{eval, parse};

fn main() {
    let stdin = std::io::stdin();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        println!(
            "{:?}",
            parse(&stdin.lock().lines().next().unwrap().unwrap())
                .map(|nodes| nodes.iter().map(|ast| eval(ast)).collect::<Vec<_>>())
        );
    }
}
