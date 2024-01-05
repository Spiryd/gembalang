use std::collections::{HashMap, HashSet};

use crate::ast::{self, Main, Procedure, Program};

//uses registers a,b,c,d,e result-d args-bc
const MUL: &str = r#""#;

enum Instruction {
    Read,
    Write,
    Load,
    Store,
    Add,
    Sub,
    Get,
    Put,
    Rst,
    Inc,
    Dec,
    Shl,
    Shr,
    Jump,
    Jpos,
    Jzero,
    Strk,
    Jumpr,
    Halt,
    Mul,
    Div,
    Mod,
}

enum VariableVariant {
    Atomic,
    Table(Option<u64>),
}

struct ProcedureAssembler {
    outer_vars: HashMap<String, VariableVariant>,
    inner_vars: HashMap<String, VariableVariant>,
}
impl ProcedureAssembler {
    fn new(proc: Procedure) -> ProcedureAssembler {
        let head = proc.0;
        let name = head.0;
        let mut varnames = HashSet::new();
        let mut outer_vars: HashMap<String, VariableVariant> = HashMap::new();
        let mut inner_vars: HashMap<String, VariableVariant> = HashMap::new();
        // TODO add chcecking for duplicate args
        for argument in head.1 {
            match argument {
                ast::ArgumentsDeclarationVariant::Base(id) => {
                    outer_vars.insert(id.clone(), VariableVariant::Atomic);
                    varnames.insert(id);
                },
                ast::ArgumentsDeclarationVariant::Table(id) => {
                    outer_vars.insert(id.clone(), VariableVariant::Table(None));
                    varnames.insert(id);
                },
            }
        }
        if let Some(declarations) = proc.1 {
            for declaration in declarations {
                match declaration {
                    ast::DeclarationVariant::Base(id) => {
                        inner_vars.insert(id.clone(), VariableVariant::Atomic);
                        varnames.insert(id);
                    },
                    ast::DeclarationVariant::NumIndexed(id, size) => {
                        inner_vars.insert(id.clone(), VariableVariant::Table(Some(size)));
                        varnames.insert(id);
                    },
                }
            }
        }
        ProcedureAssembler { outer_vars, inner_vars }
    }
}

struct MainAssembler {
    variables: HashMap<String, VariableVariant>,
}
impl MainAssembler {
    fn new(main: Main) -> MainAssembler {
        let mut variables: HashMap<String, VariableVariant> = HashMap::new();
        if let Some(vars) = main.0 {
            for var in vars {
                match var {
                    ast::DeclarationVariant::Base(id) => {
                        variables.insert(id, VariableVariant::Atomic);
                    },
                    ast::DeclarationVariant::NumIndexed(id, size) => {
                        variables.insert(id, VariableVariant::Table(Some(size)));
                    },
                }
            }
        }
        MainAssembler {
            variables,
        }
    }
}

pub struct PseudoAssembler {
    pseudo_assembly: Vec<Instruction>,
    procedures: HashMap<String, ProcedureAssembler>,
    main: MainAssembler,
}
impl PseudoAssembler {
    pub fn new(ast: Program) -> PseudoAssembler {
        let mut procedures: HashMap<String, ProcedureAssembler> = HashMap::new();
        if let Some(procedures_ast) = ast.0 {
            for procedure in procedures_ast {
                procedures.insert(procedure.0.0.clone(),  ProcedureAssembler::new(procedure));
            }
        }
        let main = MainAssembler::new(ast.1);
        PseudoAssembler {
            pseudo_assembly: vec![],
            procedures,
            main,
        }
    }
}
