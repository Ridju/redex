# REDEX: Lambda-Calculus Interpreter

A complete interpreter for the untyped lambda calculus, written in Rust.

## Features

* Manual lexer, recursive descent parser, and reducer
* **De Bruijn Indices:** Uses indices instead of variable names in the AST to cleanly handle alpha-equivalence and substitution
* Automatic parsing of Church numerals and booleans for a more readable output
* Predefined macros like `TRUE`, `FALSE`, and `NOT` for an easier start in the REPL

## Usage

    cargo run --release

### Example in the REPL

    > (\x. x) y
    y

    > TRUE
    (λx0. (λx1. x0))

## Roadmap & Implementation

- [x] Manual lexer for tokens (`\`, `.`, parentheses, variables)
- [x] Recursive descent parser with De Bruijn index conversion
- [x] $\beta$-reduction including shifting and substitution
- [x] Pretty printer and automatic Church numeral parsing
- [x] REPL for interactive expression testing
- [x] Comprehensive unit tests (parser, reducer, shadowing)

## Tests

    cargo test