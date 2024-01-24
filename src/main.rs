mod assembler;
mod ast;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub lexparse);

use std::env;
use std::fs;

use assembler::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        panic!("Supply 2 argumments");
    }
    let input_file_path = args.get(1).unwrap();
    let output_file_path = args.get(2).unwrap();
    let compilee = fs::read_to_string(input_file_path).unwrap();

    let parser_output = lexparse::ProgramParser::new().parse(&compilee);
    match parser_output {
        Ok(ast) => {
            let mut pseudo_assembler = Assembler::new(ast).unwrap();
            pseudo_assembler.construct().unwrap();
            let ass = pseudo_assembler.assemble();
            fs::write(output_file_path, ass).expect("Syntax Error");
        },
        Err(_) => {
            println!("Error parsing file")
        },
    };
}

#[cfg(test)]
mod compiler_test {
    use ast::Program;
    use super::*;
    #[test]
    fn exaple1_compile() {
        let program = fs::read_to_string("examples/gembala/example1.imp").unwrap();
        let ast: Program = lexparse::ProgramParser::new().parse(&program).unwrap();
        let mut pseudo_assembler = Assembler::new(ast).unwrap();
        pseudo_assembler.construct().unwrap();
        pseudo_assembler.assemble();
    }
    #[test]
    fn exaple2_compile() {
        let program = fs::read_to_string("examples/gembala/example2.imp").unwrap();
        let ast: Program = lexparse::ProgramParser::new().parse(&program).unwrap();
        let mut pseudo_assembler = Assembler::new(ast).unwrap();
        pseudo_assembler.construct().unwrap();
        pseudo_assembler.assemble();
    }
    #[test]
    fn exaple3_compile() {
        let program = fs::read_to_string("examples/gembala/example3.imp").unwrap();
        let ast: Program = lexparse::ProgramParser::new().parse(&program).unwrap();
        let mut pseudo_assembler = Assembler::new(ast).unwrap();
        pseudo_assembler.construct().unwrap();
        pseudo_assembler.assemble();
    }
    #[test]
    fn exaple4_compile() {
        let program = fs::read_to_string("examples/gembala/example4.imp").unwrap();
        let ast: Program = lexparse::ProgramParser::new().parse(&program).unwrap();
        let mut pseudo_assembler = Assembler::new(ast).unwrap();
        pseudo_assembler.construct().unwrap();
        pseudo_assembler.assemble();
    }
    #[test]
    fn exaple5_compile() {
        let program = fs::read_to_string("examples/gembala/example5.imp").unwrap();
        let ast: Program = lexparse::ProgramParser::new().parse(&program).unwrap();
        let mut pseudo_assembler = Assembler::new(ast).unwrap();
        pseudo_assembler.construct().unwrap();
        pseudo_assembler.assemble();
    }
    #[test]
    fn exaple6_compile() {
        let program = fs::read_to_string("examples/gembala/example6.imp").unwrap();
        let ast: Program = lexparse::ProgramParser::new().parse(&program).unwrap();
        let mut pseudo_assembler = Assembler::new(ast).unwrap();
        pseudo_assembler.construct().unwrap();
        pseudo_assembler.assemble();
    }
    #[test]
    fn exaple7_compile() {
        let program = fs::read_to_string("examples/gembala/example7.imp").unwrap();
        let ast: Program = lexparse::ProgramParser::new().parse(&program).unwrap();
        let mut pseudo_assembler = Assembler::new(ast).unwrap();
        pseudo_assembler.construct().unwrap();
        pseudo_assembler.assemble();
    }
    #[test]
    fn exaple8_compile() {
        let program = fs::read_to_string("examples/gembala/example8.imp").unwrap();
        let ast: Program = lexparse::ProgramParser::new().parse(&program).unwrap();
        let mut pseudo_assembler = Assembler::new(ast).unwrap();
        pseudo_assembler.construct().unwrap();
        pseudo_assembler.assemble();
    }
    #[test]
    fn exaple9_compile() {
        let program = fs::read_to_string("examples/gembala/example9.imp").unwrap();
        let ast: Program = lexparse::ProgramParser::new().parse(&program).unwrap();
        let mut pseudo_assembler = Assembler::new(ast).unwrap();
        pseudo_assembler.construct().unwrap();
        pseudo_assembler.assemble();
    }
}
