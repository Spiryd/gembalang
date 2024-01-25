
# GEMBALANG
A simple imperative language compiler. INA, PWr, 2023/24

# Author

Maksymilian Neumann

# Requirements

Compilation requires the [rust toolchain](https://www.rust-lang.org/tools/install) and an internet connection.

# Usage

````
$ cargo run -r -- <input_file> <output_file>
````
or you can build it and use the binary.
````
$ cargo build -r
````
it outputs the binary in "path_to_project/target/relese" it should be called gembalang. <br>
usage:
````
$ ./gembalng <input_file> <output_file>
````

# File Description

All important source files are in the src directory

## main.rs

Just an entry point to the program. Uses all the other source files.

## lexparse.lalrpop

This is our lexer/parser. Made using [LALRPOP](https://github.com/lalrpop/lalrpop) crate wich is an rust alternative to lex/bison. It lexes and parses our input and outputs an AST.

## ast.rs

Defines the structure of the AST. Handles Errors.

## assembler.rs

This is where the magic happens. This defines a struct that is first used to build the AST into pseudo-assembly and than into the final file.

## Cargo.toml

Configuration file for cargo.

## build.rs

Custom build script for cargo to complie our lexer/parser.
