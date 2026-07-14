use std::iter::Peekable;

use crate::scanner::Token;

#[derive(PartialEq, Debug)]
enum ParserError {
    UnexpectedToken { token: Token },
    NotTerminated,
}

enum AST {
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
                    Some(Token::OpenBracket) => Ok(left),
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
