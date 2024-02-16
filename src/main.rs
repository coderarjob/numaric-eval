use eval::eval;
use shyard::postfix;
use std::env;
use std::process::ExitCode;

mod eval;
mod shyard;

fn main() -> ExitCode {
    let mut iter = env::args();

    let program = iter.next().unwrap();
    if env::args().count() < 2 {
        eprintln!("Usage: {program} infix equation");
        return ExitCode::FAILURE;
    }

    let equation = iter.next().unwrap();

    let postfix = postfix(&equation).unwrap();
    let result = eval(&postfix).unwrap();
    println!("Postfix: {postfix}");
    println!("Eval: {result}");
    ExitCode::SUCCESS
}
