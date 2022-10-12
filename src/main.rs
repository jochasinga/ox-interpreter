pub mod lexer;

use lexer::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().into_iter();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(c) = chars.next() {
        match c {
            ' ' | '\n' => {
                continue;
            },
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '"' => {
                let mut s: Vec<char> = Vec::new();
                while let Some(cc) = chars.next() {
                    if cc == '"' {
                        tokens.push(Token::String(s.into_iter().collect()));
                        break;
                    }
                    s.push(cc);
                }
            },
            c => {
                let mut tok: Vec<char> = Vec::new();
                tok.push(c);
                while let Some(cc) = chars.next() {
                    if vec![' ', ')'].contains(&cc) {
                        let word: String = tok.iter().collect();
                        let i = word.parse::<i64>();
                        if i.is_ok() {
                            tokens.push(Token::Integer(i.unwrap()));
                        } else {
                            let i = word.parse::<f64>();
                            if i.is_ok() {
                                tokens.push(Token::Float(i.unwrap()));
                            } else {
                                tokens.push(Token::Symbol(word.to_string()));
                            }
                        }
                        if cc == ')' {
                            tokens.push(Token::RParen);
                        }
                        break;
                    } else {
                        println!("pushing {}", cc);
                        tok.push(cc);
                    }
                }
            }
        }
    }

    tokens
}

fn main() {

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_string() {
        let tokens_1 = tokenize("(\"foo\")");
        assert_eq!(
            tokens_1,
            vec![
                Token::LParen,
                Token::String("foo".to_string()),
                Token::RParen,
            ]
        );
        let tokens_2 = tokenize("(\"foo\" \"bar\" \"baz\")");
        assert_eq!(
            tokens_2,
            vec![
                Token::LParen,
                Token::String("foo".to_string()),
                Token::String("bar".to_string()),
                Token::String("baz".to_string()),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_add() {
        let tokens = tokenize("(+ 1 2)");
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_area_of_circle() {
        let program = "
            (
                (define r 10)
                (define pi 3.14)
                (* pi (* r r))
            )
        ";

        let tokens = tokenize(program);
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("r".to_string()),
                Token::Integer(10),
                Token::RParen,
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("pi".to_string()),
                Token::Float(3.14),
                Token::RParen,
                Token::LParen,
                Token::Symbol("*".to_string()),
                Token::Symbol("pi".to_string()),
                Token::LParen,
                Token::Symbol("*".to_string()),
                Token::Symbol("r".to_string()),
                Token::Symbol("r".to_string()),
                Token::RParen,
                Token::RParen,
                Token::RParen,
            ],
        );
    }
}
