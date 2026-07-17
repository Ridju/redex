pub mod Reducer {
    use crate::parser;

    fn shift(term: &parser::AST, depth: usize, inc: isize) -> parser::AST {
        match term {
            parser::AST::Index(i) => {
                if *i >= depth {
                    parser::AST::Index((*i as isize + inc) as usize)
                } else {
                    parser::AST::Index(*i)
                }
            }
            parser::AST::Abstraction { body } => parser::AST::Abstraction {
                body: Box::new(shift(body, depth + 1, inc)),
            },
            parser::AST::Application { first, second } => parser::AST::Application {
                first: Box::new(shift(first, depth, inc)),
                second: Box::new(shift(second, depth, inc)),
            },
        }
    }

    fn substitute(term: &parser::AST, target: usize, replacement: &parser::AST) -> parser::AST {
        match term {
            parser::AST::Index(i) => {
                if *i == target {
                    shift(replacement, 0, target as isize)
                } else {
                    parser::AST::Index(*i)
                }
            }
            parser::AST::Abstraction { body } => parser::AST::Abstraction {
                body: Box::new(substitute(body, target + 1, replacement)),
            },
            parser::AST::Application { first, second } => parser::AST::Application {
                first: Box::new(substitute(first, target, replacement)),
                second: Box::new(substitute(second, target, replacement)),
            },
        }
    }

    fn reduce(mut term: parser::AST) -> parser::AST {
        loop {
            let next_term = reduce_step(term.clone());
            if next_term == term {
                break;
            }
            term = next_term;
        }
        term
    }

    fn reduce_step(term: parser::AST) -> parser::AST {
        match term {
            // Redex gefunden: (λ. body) argument
            parser::AST::Application { first, second } => {
                if let parser::AST::Abstraction { body } = *first {
                    let result = substitute(&body, 0, &*second);
                    shift(&result, 0, -1)
                } else {
                    parser::AST::Application {
                        first: Box::new(reduce_step(*first)),
                        second: Box::new(reduce_step(*second)),
                    }
                }
            }
            parser::AST::Abstraction { body } => parser::AST::Abstraction {
                body: Box::new(reduce_step(*body)),
            },
            // Index kann nicht weiter reduziert werden
            _ => term,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::AST;

    #[test]
    fn test_simple_substitution() {
        let term = AST::Abstraction {
            name: "x".to_string(),
            body: Box::new(AST::Atom("x".to_string())),
        };

        let replacement = AST::Atom("y".to_string());

        let mut reducer = Reducer {};
        let result = reducer.substitute(term, &"x".to_string(), &replacement);

        let expected = AST::Abstraction {
            name: "x".to_string(),
            body: Box::new(AST::Atom("y".to_string())),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_free_variable_substitution() {
        let term = AST::Abstraction {
            name: "z".to_string(),
            body: Box::new(AST::Atom("x".to_string())),
        };

        let replacement = AST::Atom("y".to_string());
        let mut reducer = Reducer {};
        let result = reducer.substitute(term, &"x".to_string(), &replacement);

        let expected = AST::Abstraction {
            name: "z".to_string(),
            body: Box::new(AST::Atom("y".to_string())),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_reduction() {
        let term = AST::Application {
            first: Box::new(AST::Abstraction {
                name: "x".to_string(),
                body: Box::new(AST::Atom("x".to_string())),
            }),
            second: Box::new(AST::Atom("y".to_string())),
        };

        let mut reducer = Reducer {};
        let result = reducer.reduce(term);

        let expected = AST::Atom("y".to_string());

        assert_eq!(result, expected);
    }
}
