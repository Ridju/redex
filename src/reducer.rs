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
