#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Lambda,
    Dot,
    OpenBracket,
    ClosedBracket,
    Variable(String),
}

#[derive(PartialEq, Debug)]
pub enum ScannerError {
    UnexpectedCharacter(char),
}

pub mod scanner {
    use super::*;

    pub fn scan(source: &str) -> Result<Vec<Token>, ScannerError> {
        let mut chars = source.chars().peekable();
        let mut tokens = Vec::new();
        while let Some(&c) = chars.peek() {
            match c {
                '\\' => {
                    chars.next();
                    tokens.push(Token::Lambda);
                }
                '.' => {
                    chars.next();
                    tokens.push(Token::Dot);
                }
                '(' => {
                    chars.next();
                    tokens.push(Token::OpenBracket);
                }
                ')' => {
                    chars.next();
                    tokens.push(Token::ClosedBracket);
                }
                c if c.is_whitespace() => {
                    chars.next();
                }
                c if c.is_alphabetic() => {
                    let var_name = scan_variable(&mut chars);
                    tokens.push(Token::Variable(var_name));
                }
                _ => return Err(ScannerError::UnexpectedCharacter(c)),
            }
        }

        Ok(tokens)
    }

    fn scan_variable(iter: &mut std::iter::Peekable<std::str::Chars>) -> String {
        let mut result = String::new();
        while let Some(&c) = iter.peek() {
            if c == ' ' || c == '\\' || c == '.' || c == '(' || c == ')' {
                break;
            }

            if let Some(c) = iter.next() {
                result.push(c);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lambda_input() {
        let tokens = scanner::scan("\\").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Lambda));
    }

    #[test]
    fn test_dot_test() {
        let tokens = scanner::scan(".").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Dot));
    }

    #[test]
    fn test_open_bracket_input() {
        let tokens = scanner::scan("(").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::OpenBracket));
    }

    #[test]
    fn test_closing_bracket_input() {
        let tokens = scanner::scan(")").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::ClosedBracket));
    }

    #[test]
    fn test_variable_input() {
        let tokens = scanner::scan("test_var").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Variable(_)));
        if let Token::Variable(name) = &tokens[0] {
            assert_eq!(name, "test_var");
        }
    }

    #[test]
    fn test_full_lambda() {
        let tokens = scanner::scan("\\(x.x) y").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Lambda,
                Token::OpenBracket,
                Token::Variable("x".to_string()),
                Token::Dot,
                Token::Variable("x".to_string()),
                Token::ClosedBracket,
                Token::Variable("y".to_string()),
            ]
        );
    }
    #[test]
    fn test_error_handling() {
        let result = scanner::scan("!");

        assert_eq!(result, Err(ScannerError::UnexpectedCharacter('!')));
    }
}
