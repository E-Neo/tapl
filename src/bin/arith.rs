use rustyline::{error::ReadlineError, Editor};
use tapl::arith::{eval, parse};

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        match rl.readline("> ") {
            Ok(line) => match parse(line.as_str()) {
                Ok(ast) => match eval(&ast) {
                    Ok(val) => println!("{}", val),
                    Err(e) => eprintln!("{}", e),
                },
                Err(e) => eprintln!("{}", e),
            },
            Err(ReadlineError::Interrupted) => (),
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}
