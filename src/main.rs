use std::{fmt, fmt::Display, process::ExitCode};

#[derive(Debug, PartialEq, PartialOrd)]
enum Operations {
    NoOp,
    OpenBracket,
    Subtraction,
    Addition,
    Multiply,
    Divide,
    CloseBracket,
}

impl Display for Operations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            Self::Addition => "+".to_owned(),
            Self::Subtraction => "-".to_owned(),
            Self::Multiply => "*".to_owned(),
            Self::Divide => "/".to_owned(),
            Self::OpenBracket => "(".to_owned(),
            Self::CloseBracket => ")".to_owned(),
            Self::NoOp => "nop".to_owned(),
        };
        write!(f, "{}", op_str)
    }
}

#[derive(Debug)]
enum Tokens {
    Number(u32),
    Operation(Operations),
    Unknown(String),
}

fn main() -> ExitCode {
    //let infix = "( 2 + 4 ) / 2 * 60";
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

fn postfix(eq: &str) -> Result<String, &'static str> {
    let mut out_queue = Vec::<String>::new();
    let mut stack = Vec::<Operations>::new();

    let split = eq.split(' ');
    for lexeme in split {
        let token = detect_token(lexeme);
        match token {
            Tokens::Number(x) => out_queue.push(x.to_string()),
            Tokens::Operation(op @ Operations::OpenBracket) => stack.push(op),
            Tokens::Operation(Operations::CloseBracket) => {
                while let Some(op) = stack.pop() {
                    if op != Operations::OpenBracket {
                        break;
                    }

                    out_queue.push(op.to_string());
                }
            }
            Tokens::Operation(op) => addop(op, &mut stack, &mut out_queue),
            Tokens::Unknown(op) => {
                return Err("Invalid token! {op}");
            }
        }
    }

    while let Some(c) = stack.pop() {
        out_queue.push(c.to_string())
    }

    Ok(out_queue.join(" "))
}

fn addop(op: Operations, stack: &mut Vec<Operations>, out_queue: &mut Vec<String>) {
    let top_op = stack.last().unwrap_or(&Operations::NoOp);
    if *top_op > op {
        out_queue.push(stack.pop().unwrap_or(Operations::NoOp).to_string());
    }
    stack.push(op);
}

fn detect_token(lexeme: &str) -> Tokens {
    let value = lexeme.parse::<u32>();
    let ret = match lexeme {
        "+" => Tokens::Operation(Operations::Addition),
        "-" => Tokens::Operation(Operations::Subtraction),
        "*" => Tokens::Operation(Operations::Multiply),
        "/" => Tokens::Operation(Operations::Divide),
        "(" => Tokens::Operation(Operations::OpenBracket),
        ")" => Tokens::Operation(Operations::CloseBracket),
        _ if value.is_ok() => Tokens::Number(value.unwrap()),
        x => Tokens::Unknown(x.to_owned()),
    };
    ret
}

#[test]
fn same_precedence() {
    let infix = "2 + 4 + 3";
    // TODO: Should be 2 4 + 3 +
    assert_eq!(postfix(infix), Ok("2 4 3 + +".to_owned()));
}
#[test]
fn bodmas_order_test_1() {
    let infix = "2 + 4 / 3 * 60";
    assert_eq!(postfix(infix), Ok("2 4 3 / 60 * +".to_owned()));
}

#[test]
fn bodmas_order_test_2() {
    let infix = "2 / 4 * 3 + 60 - 1";
    assert_eq!(postfix(infix), Ok("2 4 / 3 * 60 + 1 -".to_owned()));
}

#[test]
fn bodmas_order_test_3() {
    let infix = "2 - 4 + 3 * 60 / 1";
    // TODO: Should be 2 4 - 3 60 * 1 / +
    assert_eq!(postfix(infix), Ok("2 4 3 60 1 / * + -".to_owned()));
}
