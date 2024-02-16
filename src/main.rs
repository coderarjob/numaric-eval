use std::process::ExitCode;
use shyard::postfix;
mod shyard;

fn main() -> ExitCode {
    let infix = "( 2 + 4 ) / 2 * 60";

    match postfix(infix) {
        Err(error) => {
            println!("Error: {error}");
            ExitCode::FAILURE
        }
        Ok(postfix) => {
            println!("{postfix}");
            ExitCode::SUCCESS
        }
    }
}
