use std::iter::Peekable;

use crate::scanner::Token;

#[derive(PartialEq, Debug)]
enum ParserError {
    UnexpectedToken { token: Token },
    NotTerminated,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AST {
    Abstraction { name: String, body: Box<AST> },
    Application { first: Box<AST>, second: Box<AST> },
    Atom(String),
}

struct Parser<'a> {
    iter: Peekable<std::slice::Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            iter: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<AST, ParserError> {
        let ast = self.parse_term()?;

        if self.iter.peek().is_some() {
            let token = self.iter.next().unwrap();
            return Err(ParserError::UnexpectedToken {
                token: token.clone(),
            });
        }

        Ok(ast)
    }

    fn parse_term(&mut self) -> Result<AST, ParserError> {
        match self.iter.peek() {
            Some(Token::Lambda) => self.parse_abstraction(),
            Some(Token::OpenBracket) | Some(Token::Variable(_)) => self.parse_application(),
            Some(&t) => Err(ParserError::UnexpectedToken { token: t.clone() }),
            _ => Err(ParserError::NotTerminated),
        }
    }

    fn parse_abstraction(&mut self) -> Result<AST, ParserError> {
        self.iter.next();
        let var_name = self.expect_variable()?;
        self.expect_dot()?;
        let body = self.parse_term()?;
        Ok(AST::Abstraction {
            name: var_name,
            body: Box::new(body),
        })
    }

    fn parse_application(&mut self) -> Result<AST, ParserError> {
        let mut left = self.parse_atom()?;

        while let Some(token) = self.iter.peek() {
            match token {
                Token::OpenBracket | Token::Variable(_) => {
                    let right = self.parse_atom()?;
                    left = AST::Application {
                        first: Box::new(left),
                        second: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_atom(&mut self) -> Result<AST, ParserError> {
        let token = self.iter.next().ok_or(ParserError::NotTerminated)?;

        match token {
            Token::Variable(name) => Ok(AST::Atom(name.clone())),
            Token::OpenBracket => {
                let left = self.parse_term()?;

                match self.iter.next() {
                    Some(Token::ClosedBracket) => Ok(left),
                    Some(t) => Err(ParserError::UnexpectedToken { token: t.clone() }),
                    None => Err(ParserError::NotTerminated),
                }
            }
            t => Err(ParserError::UnexpectedToken { token: t.clone() }),
        }
    }

    fn expect_variable(&mut self) -> Result<String, ParserError> {
        match self.iter.next() {
            Some(Token::Variable(name)) => Ok(name.clone()),
            Some(t) => Err(ParserError::UnexpectedToken { token: t.clone() }),
            None => Err(ParserError::NotTerminated),
        }
    }

    fn expect_dot(&mut self) -> Result<String, ParserError> {
        match self.iter.next() {
            Some(Token::Dot) => Ok(".".to_string()),
            Some(t) => {
                return Err(ParserError::UnexpectedToken { token: t.clone() });
            }
            None => {
                return Err(ParserError::NotTerminated);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abstraction() {
        let tokens = vec![
            Token::Lambda,
            Token::Variable("x".to_string()),
            Token::Dot,
            Token::Variable("x".to_string()),
        ];
        let mut parser = Parser::new(&tokens);

        let result = parser.parse().unwrap();

        assert_eq!(
            result,
            AST::Abstraction {
                name: "x".to_string(),
                body: Box::new(AST::Atom("x".to_string()))
            }
        );
    }

    #[test]
    fn test_fail_abstraction() {
        let tokens = vec![Token::Dot];
        let mut parser = Parser::new(&tokens);

        let result = parser.parse();

        assert!(matches!(
            result,
            Err(ParserError::UnexpectedToken { token: Token::Dot })
        ));
    }

    #[test]
    fn test_application_left_associativity() {
        let tokens = vec![
            Token::Variable("a".into()),
            Token::Variable("b".into()),
            Token::Variable("c".into()),
        ];
        let mut parser = Parser::new(&tokens);
        let result = parser.parse().unwrap();

        let expected = AST::Application {
            first: Box::new(AST::Application {
                first: Box::new(AST::Atom("a".into())),
                second: Box::new(AST::Atom("b".into())),
            }),
            second: Box::new(AST::Atom("c".into())),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_abstraction_application_mix() {
        let tokens = vec![
            Token::Lambda,
            Token::Variable("x".into()),
            Token::Dot,
            Token::Variable("x".into()),
            Token::Variable("y".into()),
        ];

        let result = Parser::new(&tokens).parse().unwrap();

        // Prüfe die Struktur: Abstraktion enthält Applikation
        if let AST::Abstraction { name, body } = result {
            assert_eq!(name, "x");
            assert!(matches!(*body, AST::Application { .. }));
        } else {
            panic!("Sollte eine Abstraktion sein");
        }
    }

    #[test]
    fn test_atom_variable() {
        let tokens = vec![Token::Variable("x".to_string())];
        let mut parser = Parser::new(&tokens);
        let result = parser.parse_atom().unwrap();

        assert_eq!(result, AST::Atom("x".to_string()));
    }
}
