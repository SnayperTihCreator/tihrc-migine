use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass_enum;

#[gen_stub_pyclass_enum]
#[pyclass(eq, ord, hash, from_py_object, frozen)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub enum Operators {
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    
    Comma,
    Equal,
    Colon,
    Semicolon,
    Ampersand,
    Pipe,
    Exclamation,
    RangeDots,
    Prime,

    LParen,
    RParen,
    LBracket,
    RBracket,

    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    DoubleEqual,
    NotEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(i64),
    Number(f64),
    Complex(f64, f64),
    Identifier(String),
    Operator(Operators),
    EOF,
}

pub fn tokenizez(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } 
        else if c == '.' && {
            let mut clone_chars = chars.clone();
            clone_chars.next();
            clone_chars.peek() == Some(&'.')
        } {
            chars.next();
            chars.next();
            tokens.push(Token::Operator(Operators::RangeDots));
        }
        else if c.is_digit(10) || c == '.' {
        let number_str = read_number_with_sci(&mut chars)?;
        let is_float = number_str.contains('.') || number_str.contains('e') || number_str.contains('E');

        if let Some(&next) = chars.peek() {
            if next == 'j' {
                chars.next();
                match number_str.parse::<f64>() {
                    Ok(im) => tokens.push(Token::Complex(0.0, im)),
                    Err(_) => return Err("Invalid complex number".to_string()),
                }
            } else if !is_float {
                match number_str.parse::<i64>() {
                    Ok(n) => tokens.push(Token::Integer(n)),
                    Err(_) => {
                        tokens.push(Token::Number(number_str.parse::<f64>().map_err(|_| "Invalid number")?))
                    }
                }
            } else {
                match number_str.parse::<f64>() {
                    Ok(n) => tokens.push(Token::Number(n)),
                    Err(_) => return Err("Invalid number".to_string()),
                }
            }
        } else {
            if !is_float {
                match number_str.parse::<i64>() {
                    Ok(n) => tokens.push(Token::Integer(n)),
                    Err(_) => tokens.push(Token::Number(number_str.parse::<f64>().map_err(|_| "Invalid number")?)),
                }
            } else {
                match number_str.parse::<f64>() {
                    Ok(n) => tokens.push(Token::Number(n)),
                    Err(_) => return Err("Invalid number".to_string()),
                }
            }
        }
    } else if c.is_alphabetic() {
            let mut id_str = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c.is_alphanumeric() || next_c == '_' {
                    id_str.push(next_c);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Identifier(id_str));
        } else {
            let operator = match c {
                '+' => Operators::Plus,
                '-' => Operators::Minus,
                '*' => Operators::Star,
                '/' => Operators::Slash,
                '^' => Operators::Caret,
                ',' => Operators::Comma,
                ';' => Operators::Semicolon,
                ':' => Operators::Colon,
                '(' => Operators::LParen,
                ')' => Operators::RParen,
                '[' => Operators::LBracket,
                ']' => Operators::RBracket,
                '&' => Operators::Ampersand,
                '|' => Operators::Pipe,
                '\'' => {
                    tokens.push(Token::Operator(Operators::Prime));
                    chars.next();
                    continue;
                }
                '!' => {
                    chars.next();
                    if let Some(&'=') = chars.peek() {
                        chars.next();
                        tokens.push(Token::Operator(Operators::NotEqual));
                        continue;
                    } else {
                        tokens.push(Token::Operator(Operators::Exclamation));
                        continue;
                    }
                }
                '<' => {
                    chars.next();
                    if let Some(&'=') = chars.peek() {
                        chars.next();
                        tokens.push(Token::Operator(Operators::LessEqual));
                        continue;
                    } else {
                        tokens.push(Token::Operator(Operators::LessThan));
                        continue;
                    }
                }
                '>' => {
                    chars.next();
                    if let Some(&'=') = chars.peek() {
                        chars.next();
                        tokens.push(Token::Operator(Operators::GreaterEqual));
                        continue;
                    } else {
                        tokens.push(Token::Operator(Operators::GreaterThan));
                        continue;
                    }
                }
                '=' => {
                    chars.next();
                    if let Some(&'=') = chars.peek() {
                        chars.next();
                        tokens.push(Token::Operator(Operators::DoubleEqual));
                        continue;
                    } else {
                        tokens.push(Token::Operator(Operators::Equal));
                        continue;
                    }
                }
                _ => return Err(format!("Unknown character: {}", c)),
            };
            tokens.push(Token::Operator(operator));
            chars.next();
        }
    }

    tokens.push(Token::EOF);
    Ok(tokens)
}

fn read_number_with_sci(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<String, String> {
    let mut num_str = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_digit(10) {
            num_str.push(c);
            chars.next();
        } else if c == '.' {
            let mut clone_chars = chars.clone();
            clone_chars.next();
            if clone_chars.peek() == Some(&'.') {
                break;
            }
            num_str.push(c);
            chars.next();
        } else {
            break;
        }
    }
    if let Some(&c) = chars.peek() {
        if c == 'e' || c == 'E' {
            num_str.push(c);
            chars.next();

            if let Some(&c) = chars.peek() {
                if c == '+' || c == '-' {
                    num_str.push(c);
                    chars.next();
                }
            }

            let mut has_digit = false;
            while let Some(&c) = chars.peek() {
                if c.is_digit(10) {
                    num_str.push(c);
                    chars.next();
                    has_digit = true;
                } else {
                    break;
                }
            }
            if !has_digit {
                return Err("Scientific notation lacks exponent digits".to_string());
            }
        }
    }
    Ok(num_str)
}