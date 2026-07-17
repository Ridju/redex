use std::iter::Peekable;

use crate::scanner::Token;

#[derive(PartialEq, Debug)]
enum ParserError {
    UnexpectedToken { token: Token },
    NotTerminated,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AST {
    Abstraction { body: Box<AST> },
    Application { first: Box<AST>, second: Box<AST> },
    Index(usize),
}

struct Parser<'a> {
    iter: Peekable<std::slice::Iter<'a, Token>>,
    context: Vec<String>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            iter: tokens.iter().peekable(),
            context: Vec::new(),
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

        self.context.push(var_name);

        let body = self.parse_term()?;

        self.context.pop();

        Ok(AST::Abstraction {
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
            Token::Variable(name) => {
                if let Some(pos) = self.context.iter().rev().position(|n| n == name) {
                    Ok(AST::Index(pos))
                } else { 
                    Err(ParserError::UnexpectedToken { token: Token::Variable(name.clone()) })
                }
            }
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
    use crate::scanner::Token;

    #[test]
    fn test_simple_abstraction() {
        // λx. x  => Abstraktion(Index 0)
        let tokens = vec![
            Token::Lambda, Token::Variable("x".into()), Token::Dot, Token::Variable("x".into()),
        ];
        let result = Parser::new(&tokens).parse().unwrap();

        assert_eq!(result, AST::Abstraction { body: Box::new(AST::Index(0)) });
    }

    #[test]
    fn test_k_combinator() {
        // λx. λy. x => Abstraktion(Abstraktion(Index 1))
        let tokens = vec![
            Token::Lambda, Token::Variable("x".into()), Token::Dot,
            Token::Lambda, Token::Variable("y".into()), Token::Dot,
            Token::Variable("x".into()),
        ];
        let result = Parser::new(&tokens).parse().unwrap();

        assert_eq!(
            result,
            AST::Abstraction {
                body: Box::new(AST::Abstraction {
                    body: Box::new(AST::Index(1)),
                }),
            }
        );
    }

    #[test]
    fn test_variable_shadowing() {
        // λx. λx. x => Abstraktion(Abstraktion(Index 0))
        let tokens = vec![
            Token::Lambda, Token::Variable("x".into()), Token::Dot,
            Token::Lambda, Token::Variable("x".into()), Token::Dot,
            Token::Variable("x".into()),
        ];
        let result = Parser::new(&tokens).parse().unwrap();

        assert_eq!(
            result,
            AST::Abstraction {
                body: Box::new(AST::Abstraction {
                    body: Box::new(AST::Index(0)),
                }),
            }
        );
    }

    #[test]
    fn test_application() {
        // (λx. x) y  -> Da 'y' frei ist, wird das mit der aktuellen Logik einen Fehler werfen.
        // Das ist korrektes Verhalten für einen strikten Lambda-Interpreter!
        let tokens = vec![
            Token::OpenBracket,
            Token::Lambda, Token::Variable("x".into()), Token::Dot, Token::Variable("x".into()),
            Token::ClosedBracket,
            Token::Variable("y".into()),
        ];
        let result = Parser::new(&tokens).parse();
        assert!(result.is_err()); // 'y' ist im Kontext nicht gebunden
    }
}
