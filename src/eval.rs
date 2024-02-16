#[derive(Debug, PartialEq, PartialOrd)]
enum Operations {
    Subtraction,
    Addition,
    Multiply,
    Divide,
    Exponent,
}

impl Operations {
    fn parse(symbol: &str) -> Result<Operations, ()> {
        match symbol {
            "+" => Ok(Operations::Addition),
            "-" => Ok(Operations::Subtraction),
            "*" => Ok(Operations::Multiply),
            "/" => Ok(Operations::Divide),
            "^" => Ok(Operations::Exponent),
            _ => Err(()),
        }
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

pub fn eval(postfix: &str) -> Result<i32, String> {
    let mut stack = Vec::<i32>::new();

    let split = postfix.split(' ');
    for lexeme in split {
        let token = Tokens::detect_token(lexeme);
        match token {
            Tokens::Number(n) => stack.push(n),
            Tokens::Operation(op) => {
                let result = process_operation(op, &mut stack)?;
                stack.push(result);
            }
            Tokens::Unknown(op) => {
                return Err(format!("Invalid token! {op}"));
            }
        }
    }

    Ok(stack.pop().unwrap_or_default())
}

fn process_operation(op: Operations, stack: &mut Vec<i32>) -> Result<i32, String> {
    let (first, second) = match op {
        Operations::Subtraction
        | Operations::Addition
        | Operations::Multiply
        | Operations::Divide
        | Operations::Exponent => {
            let second = stack.pop().ok_or_else(|| "Operator expected".to_owned())?;
            let first = stack.pop().ok_or_else(|| "Operator expected".to_owned())?;
            (first, second)
        }
    };

    match op {
        Operations::Subtraction => Ok(first - second),
        Operations::Addition => Ok(first + second),
        Operations::Multiply => Ok(first * second),
        Operations::Divide => Ok(first / second),
        Operations::Exponent => Ok(first.pow(second as u32)),
    }
}

#[test]
fn sum_two_op() {
    let postfix = "2 4 +";
    assert_eq!(eval(postfix), Ok(6));
}

#[test]
fn sum_four_op() {
    let postfix = "2 4 + 3 + 1 +";
    assert_eq!(eval(postfix), Ok(10));
}

#[test]
fn negative_num() {
    let postfix = "-10 1 - 4 +";
    assert_eq!(eval(postfix), Ok(-10 - 1 + 4));
}
