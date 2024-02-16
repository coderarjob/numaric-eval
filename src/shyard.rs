use std::{fmt, fmt::Display};

#[derive(Debug, PartialEq, PartialOrd)]
enum Operations {
    OpenBracket,
    Subtraction,
    Addition,
    Multiply,
    Divide,
    Exponent,
    CloseBracket,
}

enum OperatorAssociativity {
    Left,
    Right,
}

struct OperationProperties {
    symbol: char,
    associativity: OperatorAssociativity,
    precedence: u32,
}

impl Operations {
    const fn value(&self) -> OperationProperties {
        match self {
            Operations::OpenBracket => OperationProperties {
                symbol: '(',
                associativity: OperatorAssociativity::Left,
                precedence: 0,
            },
            Operations::Subtraction => OperationProperties {
                symbol: '-',
                associativity: OperatorAssociativity::Left,
                precedence: 1,
            },
            Operations::Addition => OperationProperties {
                symbol: '+',
                associativity: OperatorAssociativity::Left,
                precedence: 1,
            },
            Operations::Multiply => OperationProperties {
                symbol: '*',
                associativity: OperatorAssociativity::Left,
                precedence: 2,
            },
            Operations::Divide => OperationProperties {
                symbol: '/',
                associativity: OperatorAssociativity::Left,
                precedence: 2,
            },
            Operations::Exponent => OperationProperties {
                symbol: '^',
                associativity: OperatorAssociativity::Right,
                precedence: 3,
            },
            Operations::CloseBracket => OperationProperties {
                symbol: ')',
                associativity: OperatorAssociativity::Left,
                precedence: 4,
            },
        }
    }

    fn parse(symbol: &str) -> Result<Operations, ()> {
        match symbol {
            "+" => Ok(Operations::Addition),
            "-" => Ok(Operations::Subtraction),
            "*" => Ok(Operations::Multiply),
            "/" => Ok(Operations::Divide),
            "(" => Ok(Operations::OpenBracket),
            ")" => Ok(Operations::CloseBracket),
            "^" => Ok(Operations::Exponent),
            _ => Err(()),
        }
    }
}

impl Display for Operations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value().symbol)
    }
}

#[derive(Debug)]
enum Tokens {
    Number(i32),
    Operation(Operations),
    Unknown(String),
}

impl Tokens {
    fn detect_token(lexeme: &str) -> Tokens {
        let value = lexeme.parse::<i32>();
        let ret = match lexeme {
            c if Operations::parse(c).is_ok() => Tokens::Operation(Operations::parse(c).unwrap()),
            _ if value.is_ok() => Tokens::Number(value.unwrap()),
            x => Tokens::Unknown(x.to_owned()),
        };
        ret
    }
}

pub fn postfix(eq: &str) -> Result<String, String> {
    let mut out_queue = Vec::<String>::new();
    let mut stack = Vec::<Operations>::new();

    let split = eq.split(' ');
    for lexeme in split {
        let token = Tokens::detect_token(lexeme);
        match token {
            Tokens::Number(x) => out_queue.push(x.to_string()),
            Tokens::Operation(op) => add_operator(op, &mut stack, &mut out_queue),
            Tokens::Unknown(op) => {
                return Err(format!("Invalid token! {op}"));
            }
        }
    }

    while let Some(c) = stack.pop() {
        out_queue.push(c.to_string())
    }

    Ok(out_queue.join(" "))
}

fn handle_brackets(op: Operations, stack: &mut Vec<Operations>, out_queue: &mut Vec<String>) {
    match op {
        Operations::OpenBracket => stack.push(op),
        Operations::CloseBracket => {
            while let Some(op) = stack.pop() {
                if op == Operations::OpenBracket {
                    break;
                }

                out_queue.push(op.to_string());
            }
        }
        _ => assert!(false),
    }
}

fn add_operator(op: Operations, stack: &mut Vec<Operations>, out_queue: &mut Vec<String>) {
    let op_prop = op.value();

    if op == Operations::OpenBracket || op == Operations::CloseBracket {
        handle_brackets(op, stack, out_queue);
    } else {
        if let Some(top_op) = stack.last() {
            if top_op.value().precedence > op_prop.precedence
                || (top_op.value().precedence == op_prop.precedence
                    && matches!(op_prop.associativity, OperatorAssociativity::Left))
            {
                out_queue.push(stack.pop().unwrap().to_string());
            }
        }

        stack.push(op);
    }
}

#[test]
fn same_precedence() {
    let infix = "2 + 4 + 3";
    assert_eq!(postfix(infix), Ok("2 4 + 3 +".to_owned()));
}

#[test]
fn negative_number() {
    let infix = "-2 + 4 + 3";
    assert_eq!(postfix(infix), Ok("-2 4 + 3 +".to_owned()));
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
    assert_eq!(postfix(infix), Ok("2 4 - 3 60 * 1 / +".to_owned()));
}

#[test]
fn exponent() {
    let infix = "2 ^ 3 ^ 4";
    assert_eq!(postfix(infix), Ok("2 3 4 ^ ^".to_owned()));
}

#[test]
fn brackets_test_1() {
    let infix = "( 2 + 3 ) * 4";
    assert_eq!(postfix(infix), Ok("2 3 + 4 *".to_owned()));
}

#[test]
fn brackets_test_2() {
    let infix = "( ( 2 + 3 ) ) * 4";
    assert_eq!(postfix(infix), Ok("2 3 + 4 *".to_owned()));
}

#[test]
fn brackets_test_3() {
    let infix = "( 10 * ( 2 + 3 ) + 1 ) * 4";
    assert_eq!(postfix(infix), Ok("10 2 3 + * 1 + 4 *".to_owned()));
}

#[test]
fn invalid_token() {
    let infix = "2 + sine ( 60 )";
    assert_eq!(postfix(infix).is_err(), true);
}
