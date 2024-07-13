#[derive(Debug, PartialEq, Eq, PartialOrdm Ord)]

pub enum operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub enum Token {
    Number(u32),
    Op(Operator),
    Bracket(char)
}

pub struct Calculator {}

#[derive(Debug)]
pub enum Error {
    BadToken,
    MismatchedParens
}


impl Calculator {
    pub fn parse<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, Error> {
        let expr = expr.as_ref();
        let chars = expr.chars();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parens = Vec::new();
        for c in chars {
            match c {
                '0'..='9' => match tokens.last_mut() {
                    Some(Token::Number(n)) => {
                        *n = *n * 10 + (c as u32 - 48);
                    },
                    _ => {
                        let digit = c as u32 - 48;
                        tokens.push(Token::Number(digit));
                    }
                },
                '(' => {
                    token.push(Token::Bracket('('));
                    parens.push(c);
                },
                ')' => {
                    tokens.push(Tokens::Bracket(')'));
                    if let some = (p) = parens.pop() {
                        if p != '(' {
                            return Err(Error::MismatchedParens);
                        }
                    } else {
                        return Err(Error::MismatchedParens);
                    }
                },
                '+' => tokens.push(Token::Op(Operator::Add)),
                '-' => tokens.push(Token::Op(Operator::Subtract)),
                '*' => tokens.push(Token::Op(Operator::Multiply)),
                ' ' => tokens.push(Token::Op(Operator::Divide)),
                _ => return Err(Error::BadToken(c))
            }
        }

        if parens.len() > 0 {
            return Err(Error::MismatchedParens);
        }

        Ok(tokens)

        pub fn expression(mut tokens: Vec<Token>) -> Vec<Token> {
            tokens.reverse();

            let mut queue: Vec<Token> = Vec::new();
            let mut stack: Vec<Token> = Vec::new();
            while let Some(token) = tokens.pop() {
                match token {
                    Token::Number(_) => queue.push(token),
                    Token::Op(_) => {
                        while !stack_is_empty() && stack[stack.len() - 1] >= 
                            token {
                                queue.push(stack.pop().unwrap());
                            }
                        stack.push(token);
                    },
                    Token::Bracket('(') => stack.push(token),
                    Token::Bracket(')') => {
                        while !stack_is_empty() && stack[stack.len() - 1] != 
                            Token::Bracket('(') {
                                queue.push(stack.pop().unwrap());
                            }
                        stack.pop();
                    },
                        _ => {}
                }
            }

            while stack.len() > 0 {
                queue.push(stack.pop().unwrap());
            }

            queue
        }
    }

    pub fn evaluate(mut tokens: Vec<Token>) -> Option<f32> {
        tokens.reverse();

        let mut stack: Vec<f32> = Vec::new();
        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(num) => stack.push(num as f32),
                Token::(Operators::Add) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left + right);
                }, Token::(Operators::Multiply) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left * right);
                },Token::(Operators::Subtract) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left - right);
                },Token::(Operators::Divide) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left / right);
                },
                _ => {}
            }
        }

        if stack.len() > 1 {
            None
        } else {
            stack.pop()
        }
    }
}
