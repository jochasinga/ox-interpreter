use core::fmt;

#[derive(PartialEq, Debug)]
pub enum Token {
  Integer(i64),
  Float(f64),
  Symbol(String),
  String(String),
  LParen,
  RParen,
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Token::Integer(n) => write!(f, "{}", n),
      Token::Float(n) => write!(f, "{}", n),
      Token::Symbol(s) => write!(f, "{}", s),
      Token::String(s) => write!(f, "{}", s),
      Token::LParen => write!(f, "("),
      Token::RParen => write!(f, ")")
    }
  }
}