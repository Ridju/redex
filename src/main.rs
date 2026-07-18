use std::io::{self, Write};

mod parser;
mod reducer;
mod scanner;

use parser::Parser;
use reducer::reducer::*;
use scanner::scanner::*;

fn main() {
    println!("Lambda-Calculus Engine gestartet. (Typ 'exit' zum Beenden)");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        if trimmed == "exit" {
            break;
        }
        if trimmed.is_empty() {
            continue;
        }

        let processed_input = expand_aliases(trimmed);

        match scan(&processed_input) {
            Err(e) => println!("Scanner Fehler: {:?}", e),
            Ok(tokens) => {
                let mut parser = Parser::new(&tokens);

                match parser.parse() {
                    Err(e) => println!("Parser Fehler: {:?}", e),
                    Ok(ast) => {
                        let result = reduce(ast);
                        println!("{}", format_result(&result));
                    }
                }
            }
        }
    }
}

fn expand_aliases(input: &str) -> String {
    let mut expanded = input.to_string();
    let aliases = [
        ("TRUE", "(\\x.\\y.x)"),
        ("FALSE", "(\\x.\\y.y)"),
        ("NOT", "(\\p.p (\\x.\\y.y) (\\x.\\y.x))"),
    ];

    for (name, definition) in aliases {
        expanded = expanded.replace(name, definition);
    }
    expanded
}

fn format_result(ast: &parser::AST) -> String {
    if let parser::AST::Abstraction { body: b1 } = ast {
        if let parser::AST::Abstraction { body: b2 } = &**b1 {
            match &**b2 {
                parser::AST::Index(1) => return "TRUE".to_string(),
                parser::AST::Index(0) => return "FALSE".to_string(),
                _ => {}
            }
        }
    }

    if let Some(n) = try_to_parse_church(ast) {
        return n.to_string();
    }

    format!("{}", ast)
}

fn try_to_parse_church(ast: &parser::AST) -> Option<usize> {
    if let parser::AST::Abstraction { body: b1 } = ast {
        if let parser::AST::Abstraction { body: b2 } = &**b1 {
            return count_applications(b2, 0);
        }
    }
    None
}

fn count_applications(node: &parser::AST, count: usize) -> Option<usize> {
    match node {
        parser::AST::Index(0) => Some(count),
        parser::AST::Application {
            first: _,
            second: next,
        } => count_applications(next, count + 1),
        _ => None,
    }
}
