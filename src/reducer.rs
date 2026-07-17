use crate::parser;

#[derive(PartialEq, Debug)]
enum ReducerError {
    GeneritcError,
}

struct Reducer {}

impl Reducer {
    fn new() -> Self {
        todo!("write constructor");
    }

    fn substitute(
        &mut self,
        term: parser::AST,
        target: &String,
        replacement: &parser::AST,
    ) -> parser::AST {
        match term {
            parser::AST::Atom(name) => {
                if &name == target {
                    replacement.clone()
                } else {
                    parser::AST::Atom(name)
                }
            }
            parser::AST::Abstraction { name, body } => {
                if &name == target {
                    parser::AST::Abstraction { name: name, body }
                } else {
                    let new_body = self.substitute(*body, target, replacement);
                    parser::AST::Abstraction {
                        name: name,
                        body: Box::new(new_body),
                    }
                }
            }
            parser::AST::Application { first, second } => {
                let left = self.substitute(*first, target, replacement);
                let right = self.substitute(*second, target, replacement);
                parser::AST::Application {
                    first: Box::new(left),
                    second: Box::new(right),
                }
            }
        }
    }

    fn reduce(&mut self, mut term: parser::AST) -> parser::AST {
        loop {
            let next_term = self.reduce_step(term.clone());
            if next_term == term {
                break;
            }
            term = next_term;
        }
        term
    }

    fn reduce_step(&mut self, term: parser::AST) -> parser::AST {
        match term {
            parser::AST::Application { first, second } => match *first {
                parser::AST::Abstraction { name, body } => self.substitute(*body, &name, &*second),
                _ => {
                    let new_first = self.reduce_step(*first);
                    let new_second = self.reduce_step(*second);
                    parser::AST::Application {
                        first: Box::new(new_first),
                        second: Box::new(new_second),
                    }
                }
            },
            parser::AST::Abstraction { name, body } => parser::AST::Abstraction {
                name,
                body: Box::new(self.reduce_step(*body)),
            },
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
