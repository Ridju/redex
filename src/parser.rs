use crate::scanner::Token;

#[derive(PartialEq, Debug)]
enum ParserError {
    UnexpectedToken { token: Token, msg: String },
    NotTerminatedAbstracion { msg: String },
}

enum AST {
    Abstraction { name: String, body: Box<AST> },
    Application { first: Box<AST>, second: Box<AST> },
    Atom(String),
}

pub mod parser {
    use super::*;

    pub fn parse(tokens: Vec<Token>) -> Result<AST, ParserError> {
        let mut token_iter = tokens.iter().peekable();

        let ast = parse_term(&mut token_iter)?;

        if token_iter.peek().is_some() {
            let token = token_iter.next().unwrap();
            return Err(ParserError::UnexpectedToken {
                token: token.clone(),
                msg: "No valid term found".to_string(),
            });
        }

        Ok(ast)
    }

    fn parse_term(
        token_iter: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<AST, ParserError> {
        match token_iter.peek() {
            Some(Token::Lambda) => parse_abstraction(token_iter),
            Some(Token::OpenBracket) => parse_application(token_iter),
            Some(Token::Variable(_)) => parse_application(token_iter),
            Some(&t) => Err(ParserError::UnexpectedToken {
                token: t.clone(),
                msg: "Expected lambda (\\) or open bracket".to_string(),
            }),
            _ => Err(ParserError::NotTerminatedAbstracion {
                msg: "Unexpected end of term".to_string(),
            }),
        }
    }

    fn parse_abstraction(
        token_iter: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<AST, ParserError> {
        token_iter.next();

        let var_name = match token_iter.next() {
            Some(Token::Variable(name)) => name.clone(),
            Some(t) => {
                return Err(ParserError::UnexpectedToken {
                    token: t.clone(),
                    msg: "Expected Variable".to_string(),
                });
            }
            None => {
                return Err(ParserError::NotTerminatedAbstracion {
                    msg: "Found unexpected end of the abstraction".to_string(),
                });
            }
        };

        let _dot = match token_iter.next() {
            Some(Token::Dot) => ".",
            Some(t) => {
                return Err(ParserError::UnexpectedToken {
                    token: t.clone(),
                    msg: "Expected dot".to_string(),
                });
            }
            None => {
                return Err(ParserError::NotTerminatedAbstracion {
                    msg: "Found unexpected end of the abstraction".to_string(),
                });
            }
        };

        let body = parse_term(token_iter)?;

        Ok(AST::Abstraction {
            name: var_name,
            body: Box::new(body),
        })
    }

    fn parse_application(
        token_iter: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<AST, ParserError> {
        let mut left = parse_atom(token_iter)?;

        while let Some(token) = token_iter.peek() {
            match token {
                Token::OpenBracket | Token::Variable(_) => {
                    let right = parse_atom(token_iter)?;
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

    fn parse_atom(
        token_iter: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<AST, ParserError> {
        match token_iter.peek() {
            Some(Token::Variable(name)) => Ok(AST::Atom(name.clone())),
            Some(Token::OpenBracket) => {
                token_iter.next();
                let left = parse_term(token_iter)?;
                match token_iter.next() {
                    Some(Token::ClosedBracket) => Ok(left),
                    Some(t) => {
                        return Err(ParserError::UnexpectedToken {
                            token: t.clone(),
                            msg: "Expected closing bracket".to_string(),
                        });
                    }
                    None => {
                        return Err(ParserError::NotTerminatedAbstracion {
                            msg: "Expected closing bracket, found not token".to_string(),
                        });
                    }
                }
            }
            None => {
                return Err(ParserError::NotTerminatedAbstracion {
                    msg: "Expected closing bracket, found not token".to_string(),
                });
            }
            _ => {
                return Err(ParserError::NotTerminatedAbstracion {
                    msg: "Expected closing bracket, found not token".to_string(),
                });
            }
        }
    }
}
