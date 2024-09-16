use crate::config::Config;

#[derive(Debug)]
pub enum Token {
    VariableDeclaration, // let
    VariableInitialize, // =
    LineEnd, // ;
    Identifier(String),
    Number(f64)
}

fn check_starts_with(string: &str, starts_with: &str) -> bool {
    if string.starts_with(starts_with) {
        let mut iter = string.chars();
        match iter.skip(starts_with.len()).next() {
            Some(char) => match char {
                ' ' | '\n' => return true,
                _ => return false,
            }
            None => return true,
        }
    }
    return false;
}

pub fn tokenize(config: Config, string: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut index = 0;
    
    loop {
        let mut iter = string.chars().peekable();
        for _ in 0..index {
            iter.next();
        }
        if index == string.len() {
            break;
        }
        match iter.peek() {
            Some(' ' | '\n') => {
                index += 1;
                continue;
            }
            Some(char @ '0'..='9') => {
                let mut number = char.to_string();
                loop {
                    iter.next();
                    index += 1;
                    match iter.peek() {
                        Some(char @'0'..='9') => number.push(*char),
                        Some(_) => break,
                        None => break,
                    }
                }
                tokens.push(Token::Number(number.parse().unwrap()));
                continue;
            }
            Some(_) => {}
            None => panic!("how did we get here")
        }
        let slice = string.get(index..).unwrap();
        // println!("looking at slice {} with index {} with char {}", slice, index, iter.peek().unwrap());
        
        if check_starts_with(slice, &config.variable_declaration) {
            index += config.variable_declaration.len();
            tokens.push(Token::VariableDeclaration);
            continue;
        } else if check_starts_with(slice, &config.variable_initialize) {
            index += config.variable_initialize.len();
            tokens.push(Token::VariableInitialize);
            continue;
        } else if check_starts_with(slice, &config.line_end) {
            index += config.line_end.len();
            tokens.push(Token::LineEnd);
            continue;
        }
        
        match iter.next() {
            Some(char) => {
                let mut identifier = char.to_string();
                loop {
                    index += 1;
                    match iter.peek() {
                        Some(' ' | '\n') => break,
                        Some(char) => identifier.push(*char),
                        None => break,
                    }
                }
                tokens.push(Token::Identifier(identifier));
            }
            None => panic!("how")
        }
    }
    
    return tokens;
}
