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
        // Term: λz. x
        let term = AST::Abstraction {
            name: "z".to_string(),
            body: Box::new(AST::Atom("x".to_string())),
        };

        // Ersetze "x" durch "y"
        let replacement = AST::Atom("y".to_string());
        let mut reducer = Reducer {};
        let result = reducer.substitute(term, &"x".to_string(), &replacement);

        // Erwartetes Ergebnis: λz. y
        let expected = AST::Abstraction {
            name: "z".to_string(),
            body: Box::new(AST::Atom("y".to_string())),
        };

        assert_eq!(result, expected);
    }
}
