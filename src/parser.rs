use core::fmt;

use crate::lexer::{tokenize, Token};
use crate::object::Object;

#[derive(Debug, Clone)]
pub struct ParseError(pub String);
impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "invalid expression: {}", self.0)
  }
}

pub fn parse(expr: &str) -> Result<Object, ParseError> {
  let tokens = tokenize(expr);
  parse_list(tokens)
}

fn parse_list(mut tokens: Vec<Token>) -> Result<Object, ParseError> {
  // TODO: A more efficient way to do this than reversing the stack.
  tokens.reverse();
  let token = tokens.pop();

  if token != Some(Token::LParen) {
    return Err(ParseError(format!("Expected LParen, found {:?}", token)));
  }

  let mut list: Vec<Object> = Vec::new();
  while !tokens.is_empty() {
    let token = tokens.pop();
    if token.is_none() {
      return Err(ParseError("Insufficient tokens".to_string()));
    }

    let t = token.unwrap();
    match t {
      Token::Integer(n) => list.push(Object::Integer(n)),
      Token::Float(n) => list.push(Object::Float(n)),
      Token::Symbol(s) => list.push(Object::Symbol(s)),
      Token::String(s) => list.push(Object::String(s)),
      Token::LParen => {
        tokens.push(Token::LParen);
        let sub_list = parse_list(tokens.clone())?;
        list.push(sub_list);
      }
      Token::RParen => {
        return Ok(Object::List(list));
      },
    }
  }
  Ok(Object::List(list))
}


#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_add() {
    let list = parse("(+ 1 2)").unwrap();
    assert_eq!(
      list,
      Object::List(vec![
        Object::Symbol("+".to_string()),
        Object::Integer(1),
        Object::Integer(2),
      ])
    );
  }
}