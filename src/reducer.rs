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

    pub fn reduce(mut term: parser::AST) -> parser::AST {
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
    use crate::parser;


    #[test]
    fn test_identity_reduction() {
        let identity = parser::AST::Abstraction { body: Box::new(parser::AST::Index(0)) };
        let term = parser::AST::Application {
            first: Box::new(identity.clone()),
            second: Box::new(identity.clone()),
        };

        let result = Reducer::reduce(term);
        assert_eq!(result, identity);
    }

    #[test]
    fn test_k_combinator_reduction() {
        let term = parser::AST::Application {
            first: Box::new(parser::AST::Application {
                first: Box::new(parser::AST::Abstraction {
                    body: Box::new(parser::AST::Abstraction {
                        body: Box::new(parser::AST::Index(1)),
                    }),
                }),
                second: Box::new(parser::AST::Abstraction { body: Box::new(parser::AST::Index(0)) }), // Argument a
            }),
            second: Box::new(parser::AST::Abstraction { body: Box::new(parser::AST::Index(0)) }), // Argument b
        };

        let result = Reducer::reduce(term);
        
        assert_eq!(result, parser::AST::Abstraction { body: Box::new(parser::AST::Index(0)) });
    }

    #[test]
    fn test_nested_reduction() {
        let id = parser::AST::Abstraction { body: Box::new(parser::AST::Index(0)) };
        
        let term = parser::AST::Application {
            first: Box::new(parser::AST::Abstraction {
                body: Box::new(parser::AST::Application {
                    first: Box::new(id.clone()),
                    second: Box::new(parser::AST::Index(0)),
                }),
            }),
            second: Box::new(id.clone()),
        };

        let result = Reducer::reduce(term);
        assert_eq!(result, id);
    }
}