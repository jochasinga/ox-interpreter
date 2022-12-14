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
    let mut tokens = tokenize(expr);
    // TODO: A more efficient way to do this than reversing the stack.
    tokens.reverse();
    tokens.pop();
    let (obj, _) = parse_list(tokens, Vec::new())?;
    Ok(obj)
}

fn parse_list(
    mut tokens: Vec<Token>,
    mut list: Vec<Object>,
) -> Result<(Object, Vec<Token>), ParseError> {
    let token = tokens.pop();

    println!("token: {:?}", token);

    // base case
    if token.is_none() {
        return Ok((Object::List(list), tokens));
    }

    let t = token.unwrap();
    match t {
        Token::Lambda => {
            let (sub, tokens_) = parse_list(tokens, Vec::<Object>::new())?;
            match sub {
                Object::List(mut ls) => {
                    if let (Object::List(body), Object::List(params)) =
                        (ls.pop().unwrap(), ls.pop().unwrap())
                    {
                        let sparams = params
                            .iter()
                            .map(|obj| {
                                if let Object::Symbol(s) = obj {
                                    s.to_string()
                                } else {
                                    panic!("Expect Symbol. Found {}", obj)
                                }
                            })
                            .collect::<Vec<String>>();

                        let lambda = Object::Lambda(sparams, body);

                        list.push(lambda);
                        parse_list(tokens_, list)
                    } else {
                        Err(ParseError(format!("malformed lambda expression")))
                    }
                }
                _ => todo!(),
            }
        }
        Token::Integer(n) => {
            list.push(Object::Integer(n));
            parse_list(tokens, list)
        }
        Token::Float(n) => {
            list.push(Object::Float(n));
            parse_list(tokens, list)
        }
        Token::Symbol(s) => {
            if s == "#t".to_string() {
                list.push(Object::Bool(true))
            } else if s == "#f".to_string() {
                list.push(Object::Bool(false))
            } else {
                list.push(Object::Symbol(s));
            }
            parse_list(tokens, list)
        }
        Token::String(s) => {
            list.push(Object::String(s));
            parse_list(tokens, list)
        }
        Token::RParen => Ok((Object::List(list), tokens)),
        Token::LParen => {
            let (sub, tokens_) = parse_list(tokens, Vec::<Object>::new())?;
            list.push(sub);
            parse_list(tokens_, list)
        }
    }
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

    #[test]
    fn test_booleans() {
        let list = parse("(and #t #f)").unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::Symbol("and".to_string()),
                Object::Bool(true),
                Object::Bool(false),
            ]),
        );
    }

    #[test]
    fn test_mult() {
        let list = parse("(* 1.2 2.4)").unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::Symbol("*".to_string()),
                Object::Float(1.2),
                Object::Float(2.4),
            ])
        );
    }

    #[test]
    fn test_lambda() {
        let program = "(lambda (x y) (+ y x))";
        let list = parse(program).unwrap();
        assert_eq!(
            list,
            Object::List(vec![Object::Lambda(
                vec!["x".to_string(), "y".to_string()],
                vec![
                    Object::Symbol("+".to_string()),
                    Object::Symbol("y".to_string()),
                    Object::Symbol("x".to_string()),
                ]
            ),]),
        );
    }

    #[test]
    fn test_simple_nesting() {
        let program = "(* 1 (+ 2.3 5))";
        let list = parse(program).unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::Symbol("*".to_string()),
                Object::Integer(1),
                Object::List(vec![
                    Object::Symbol("+".to_string()),
                    Object::Float(2.3),
                    Object::Integer(5),
                ])
            ])
        );
    }

    #[test]
    fn test_list_of_lists() {
        let program = "((1 2 3) (4 5 6))";
        let list = parse(program).unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::List(vec![
                    Object::Integer(1),
                    Object::Integer(2),
                    Object::Integer(3),
                ]),
                Object::List(vec![
                    Object::Integer(4),
                    Object::Integer(5),
                    Object::Integer(6),
                ])
            ])
        );
    }

    #[test]
    fn test_area_of_a_circle() {
        let program = "(
      (define r 10)
      (define pi 3.14)
      (* pi (* r r))
    )";

        let list = parse(program).unwrap();
        assert_eq!(
            list,
            Object::List(vec![
                Object::List(vec![
                    Object::Symbol("define".to_string()),
                    Object::Symbol("r".to_string()),
                    Object::Integer(10),
                ]),
                Object::List(vec![
                    Object::Symbol("define".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::Float(3.14),
                ]),
                Object::List(vec![
                    Object::Symbol("*".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::List(vec![
                        Object::Symbol("*".to_string()),
                        Object::Symbol("r".to_string()),
                        Object::Symbol("r".to_string()),
                    ]),
                ]),
            ])
        );
    }
}
