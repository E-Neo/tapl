use rustyline::{error::ReadlineError, Editor};
use tapl::{
    arith::{eval, parse},
    tyarith::type_of,
};

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        match rl.readline("> ") {
            Ok(line) => match parse(line.as_str()) {
                Ok(ast) => match type_of(&ast) {
                    Ok(typ) => match eval(&ast) {
                        Ok(val) => println!("{} : {}", val, typ),
                        Err(err) => eprintln!("{}", err),
                    },
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
