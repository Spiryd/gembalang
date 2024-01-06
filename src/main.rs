mod ast;
mod assembler;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub lexparse);

use std::env;
use std::fs;

use ast::Program;
use assembler::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        panic!("Supply 2 argumments");
    }
    let input_file_path = args.get(1).unwrap();
    let output_file_path = args.get(2).unwrap();
    let program = fs::read_to_string(input_file_path).unwrap();
    let ast: Program = lexparse::ProgramParser::new().parse(&program).unwrap();
    let mut pseudo_assembler = Assembler::new(ast);
    pseudo_assembler.construct();
    let ass = pseudo_assembler.assemble();
    fs::write(output_file_path, ass).expect("Unable to write file");
}

#[cfg(test)]
mod compiler_test {
    use super::*;
    #[test]
    fn exaple1_compile() {
        let program = fs::read_to_string("examples/example1.imp").unwrap();
        lexparse::ProgramParser::new().parse(&program).unwrap();
    }
    #[test]
    fn exaple2_compile() {
        let program = fs::read_to_string("examples/example2.imp").unwrap();
        lexparse::ProgramParser::new().parse(&program).unwrap();
    }
    #[test]
    fn exaple3_compile() {
        let program = fs::read_to_string("examples/example3.imp").unwrap();
        let ast: Program = lexparse::ProgramParser::new().parse(&program).unwrap();
        let mut pseudo_assembler = Assembler::new(ast);
        pseudo_assembler.construct();
        let ass: String = pseudo_assembler.assemble();
        println!("{}", ass);
    }
    #[test]
    fn exaple4_compile() {
        let program = fs::read_to_string("examples/example4.imp").unwrap();
        lexparse::ProgramParser::new().parse(&program).unwrap();
    }
    #[test]
    fn exaple5_compile() {
        let program = fs::read_to_string("examples/example5.imp").unwrap();
        lexparse::ProgramParser::new().parse(&program).unwrap();
    }
    #[test]
    fn exaple6_compile() {
        let program = fs::read_to_string("examples/example6.imp").unwrap();
        lexparse::ProgramParser::new().parse(&program).unwrap();
    }
    #[test]
    fn exaple7_compile() {
        let program = fs::read_to_string("examples/example7.imp").unwrap();
        lexparse::ProgramParser::new().parse(&program).unwrap();
    }
    #[test]
    fn exaple8_compile() {
        let program = fs::read_to_string("examples/example8.imp").unwrap();
        lexparse::ProgramParser::new().parse(&program).unwrap();
    }
    #[test]
    fn exaple9_compile() {
        let program = fs::read_to_string("examples/example9.imp").unwrap();
        lexparse::ProgramParser::new().parse(&program).unwrap();
    }
}
