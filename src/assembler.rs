use std::{collections::HashMap, fmt::Display};

use crate::ast::*;

use Register::*;


//MUL uses registers a,b,c,d,e result-d args-bc
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            A => write!(f, "a"),
            B => write!(f, "b"),
            C => write!(f, "c"),
            D => write!(f, "d"),
            E => write!(f, "e"),
            F => write!(f, "f"),
            G => write!(f, "g"),
            H => write!(f, "h"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Read,
    Write,
    Load(Register),
    Store(Register),
    Add(Register),
    Sub(Register),
    Get(Register),
    Put(Register),
    Rst(Register),
    Inc(Register),
    Dec(Register),
    Shl(Register),
    Shr(Register),
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

#[derive(Debug)]
enum VariableVariant {
    Atomic(u64),
    Table(u64, u64),
}

#[derive(Debug)]
pub struct Assembler {
    pseudo_assembly: Vec<Instruction>,
    procedures: HashMap<String, ()>,
    variables: HashMap<String, VariableVariant>,
    ast: Program,
}
impl Assembler {
    pub fn new(ast: Program) -> Assembler {
        let mut procedures: HashMap<String, ()> = HashMap::new();
        if let Some(procedures_ast) = ast.0.clone() {
            for procedure in procedures_ast {
                procedures.insert(procedure.0 .0.clone(), ());
            }
        }
        let mut memory_pointer = 0;
        let mut variables: HashMap<String, VariableVariant> = HashMap::new();
        if let Some(vars) = ast.1 .0.clone() {
            for var in vars {
                match var {
                    DeclarationVariant::Base(id) => {
                        variables.insert(id, VariableVariant::Atomic(memory_pointer));
                        memory_pointer += 1;
                    }
                    DeclarationVariant::NumIndexed(id, size) => {
                        variables.insert(id, VariableVariant::Table(memory_pointer, size));
                        memory_pointer += size;
                    }
                }
            }
        }
        Assembler {
            pseudo_assembly: vec![],
            procedures,
            variables,
            ast,
        }
    }
    pub fn assemble(&self) -> String {
        let mut assembly: String = "".to_string();
        for instruction in &self.pseudo_assembly {
            match instruction {
                Instruction::Read => assembly += "READ\n",
                Instruction::Write => assembly += "WRITE\n",
                Instruction::Load(register) => assembly += &format!("LOAD {register}\n"),
                Instruction::Store(register) => assembly += &format!("STORE {register}\n"),
                Instruction::Add(register) => assembly += &format!("ADD {register}\n"),
                Instruction::Sub(register) => assembly += &format!("SUB {register}\n"),
                Instruction::Get(register) => assembly += &format!("GET {register}\n"),
                Instruction::Put(register) => assembly += &format!("PUT {register}\n"),
                Instruction::Rst(register) => assembly += &format!("RST {register}\n"),
                Instruction::Inc(register) => assembly += &format!("INC {register}\n"),
                Instruction::Dec(register) => assembly += &format!("DEC {register}\n"),
                Instruction::Shl(register) => assembly += &format!("SHL {register}\n"),
                Instruction::Shr(register) => assembly += &format!("SHR {register}\n"),
                Instruction::Jump => todo!(),
                Instruction::Jpos => todo!(),
                Instruction::Jzero => todo!(),
                Instruction::Strk => todo!(),
                Instruction::Jumpr => todo!(),
                Instruction::Halt =>  assembly += "HALT\n",
                Instruction::Mul => {
                    assembly += &format!("PUT e\n"); // 0 1
                    assembly += &format!("RST d\n");
                    assembly += &format!("GET c\n"); // 2 3
                    assembly += &format!("JZERO {}\n", assembly.len() + 13); // 3 4
                    assembly += &format!("SHR e\n");
                    assembly += &format!("SHL e\n");
                    assembly += &format!("GET c\n");
                    assembly += &format!("SUB e\n");
                    assembly += &format!("JZERO {}\n", assembly.len() + 3); // 8 9
                    assembly += &format!("GET d\n");
                    assembly += &format!("ADD b\n");
                    assembly += &format!("PUT d\n");
                    assembly += &format!("SHL b\n");
                    assembly += &format!("SHR c\n");
                    assembly += &format!("GET c\n");
                    assembly += &format!("PUT e\n");
                    assembly += &format!("JUMP {}\n", assembly.len() - 14); // 16  17
                    assembly += &format!("GET d\n"); // 17 18
                },
                Instruction::Div => {
                    assembly += &format!("RST d\n"); // 0 1
                    assembly += &format!("JZERO {}\n", assembly.len() + 20); // 1 2
                    assembly += &format!("GET c\n"); // 2 3
                    assembly += &format!("SUB b\n");
                    assembly += &format!("JPOS {}\n", assembly.len() + 17); // 4 5
                    assembly += &format!("GET c\n");
                    assembly += &format!("PUT e\n");
                    assembly += &format!("RST f\n");
                    assembly += &format!("INC f\n");
                    assembly += &format!("GET e\n"); // 9 10
                    assembly += &format!("SUB b\n");
                    assembly += &format!("JPOS {}\n", assembly.len() + 9); // 11 12
                    assembly += &format!("GET b\n");
                    assembly += &format!("SUB e\n");
                    assembly += &format!("PUT b\n");
                    assembly += &format!("GET d\n");
                    assembly += &format!("ADD f\n");
                    assembly += &format!("PUT d\n");
                    assembly += &format!("SHL e\n");
                    assembly += &format!("SHL f\n");
                    assembly += &format!("JUMP {}\n", assembly.len() - 12); // 20 21
                    assembly += &format!("JUMP {}\n", assembly.len() - 20); // 21 22
                    assembly += &format!("GET d\n"); // 22 23
                },
                Instruction::Mod => {
                    assembly += &format!("RST d\n");
                    assembly += &format!("JZERO {}\n", assembly.len() + 14); // 1 2
                    assembly += &format!("GET c\n");
                    assembly += &format!("SUB b\n");
                    assembly += &format!("JPOS {}\n", assembly.len() + 11); // 4 5
                    assembly += &format!("GET c\n");
                    assembly += &format!("PUT d\n"); // 6 7
                    assembly += &format!("GET d\n");
                    assembly += &format!("SUB b\n");
                    assembly += &format!("JPOS {}\n", assembly.len() + 5); // 9 10
                    assembly += &format!("GET b\n");
                    assembly += &format!("SUB d\n");
                    assembly += &format!("PUT b\n");
                    assembly += &format!("SHL d\n");
                    assembly += &format!("JUMP {}\n", assembly.len() - 9); // 14 15
                    assembly += &format!("JUMP {}\n", assembly.len() - 15); // 15 16
                    assembly += &format!("GET b\n"); // 16 17
                },
            }
        }
        assembly
    }
    pub fn construct(&mut self) {
        self.construct_main();
        self.pseudo_assembly.push(Instruction::Halt);
    }
    fn construct_main(&mut self) {
        for command in self.ast.1 .1.clone() {
            self.pseudo_assembly.extend(self.construct_command(command));
        }
    }
    fn construct_command(&self, command: Command) -> Vec<Instruction> {
        match command {
            Command::Assign(identifier, expression) => {
                let mut instructions: Vec<Instruction> = Vec::new();
                instructions.extend(self.get_pointer_from_identifier(identifier));
                instructions.push(Instruction::Put(G));
                instructions.extend(self.construct_expression(expression));
                instructions.push(Instruction::Store(G));
                instructions
            }
            Command::If(_, _, _) => todo!(),
            Command::While(_, _) => todo!(),
            Command::Repeat(_, _) => todo!(),
            Command::ProcCall(_) => todo!(),
            Command::Read(identifier) => {
                let mut instructions: Vec<Instruction> = Vec::new();
                instructions.extend(self.get_pointer_from_identifier(identifier));
                instructions.push(Instruction::Put(G));
                instructions.push(Instruction::Read);
                instructions.push(Instruction::Store(G));
                instructions
            }
            Command::Write(value) => {
                let mut instructions: Vec<Instruction> = self.extract_value(value);
                instructions.push(Instruction::Write);
                instructions
            }
        }
    }
    /// Constructs expressions into PseudoAssembly
    fn construct_expression(&self, expression: Expression) -> Vec<Instruction> {
        match expression {
            Expression::Val(value) => self.extract_value(value),
            Expression::Add(value_0, value_1) => {
                let mut instructions = self.extract_value(value_0);
                instructions.push(Instruction::Put(B));
                instructions.extend(self.extract_value(value_1));
                instructions.push(Instruction::Add(B));
                instructions
            },
            Expression::Substract(value_0, value_1) => {
                let mut instructions = self.extract_value(value_1);
                instructions.push(Instruction::Put(B));
                instructions.extend(self.extract_value(value_0));
                instructions.push(Instruction::Sub(B));
                instructions
            },
            Expression::Multiply(value_0, value_1) => {
                let mut instructions = self.extract_value(value_0);
                instructions.push(Instruction::Put(B));
                instructions.extend(self.extract_value(value_1));
                instructions.push(Instruction::Put(C));
                instructions.push(Instruction::Mul);
                instructions
            },
            Expression::Divide(value_0, value_1) => {
                let mut instructions = self.extract_value(value_0);
                instructions.push(Instruction::Put(B));
                instructions.extend(self.extract_value(value_1));
                instructions.push(Instruction::Put(C));
                instructions.push(Instruction::Div);
                instructions
            },
            Expression::Modulo(value_0, value_1) => {
                let mut instructions = self.extract_value(value_0);
                instructions.push(Instruction::Put(B));
                instructions.extend(self.extract_value(value_1));
                instructions.push(Instruction::Put(C));
                instructions.push(Instruction::Mod);
                instructions
            },
        }
    }
    /// Gets the `value` and puts it into the `A` register
    fn extract_value(&self, value: Value) -> Vec<Instruction> {
        match value {
            Value::Num(num) => get_number(num),
            Value::Id(identifier) => {
                let mut sub_instructions = self.get_pointer_from_identifier(identifier);
                sub_instructions.push(Instruction::Load(A));
                sub_instructions
            }
        }
    }
    /// Puts the pointer to `identifier` into the `A` register. Sometimes uses the `H` register.
    fn get_pointer_from_identifier(&self, identifier: Identifier) -> Vec<Instruction> {
        match identifier {
            Identifier::Base(id) => match self.variables.get(&id).unwrap() {
                VariableVariant::Atomic(pointer) => get_number(*pointer),
                VariableVariant::Table(_, _) => panic!("`{id}` is not indexable"),
            },
            Identifier::NumIndexed(id, num) => {
                let (start, size) = match self.variables.get(&id).unwrap() {
                    VariableVariant::Atomic(_) => {
                        panic!("`{id}` is a array and need to be indexed")
                    }
                    VariableVariant::Table(pointer, size) => (*pointer, *size),
                };
                if num >= size {
                    panic!("Index outside of scope, array");
                }
                get_number(start + num)
            }
            Identifier::PidIndexed(id, index_id) => {
                let mut instructions: Vec<Instruction> = Vec::new();
                match self.variables.get(&index_id).unwrap() {
                    VariableVariant::Atomic(pointer) => instructions.extend(get_number(*pointer)),
                    VariableVariant::Table(_, _) => panic!("cannot index with array"),
                };
                instructions.push(Instruction::Load(A));
                instructions.push(Instruction::Put(H));
                match self.variables.get(&id).unwrap() {
                    VariableVariant::Atomic(_) => {
                        panic!("`{id}` is a array and need to be indexed")
                    }
                    VariableVariant::Table(pointer, _) => {
                        instructions.extend(get_number(*pointer));
                    }
                }
                instructions.push(Instruction::Add(H));
                instructions
            }
        }
    }
}

/// Puts the `num` into the `A` register
fn get_number(mut num: u64) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    instructions.push(Instruction::Rst(A));
    if num != 0 {
        instructions.push(Instruction::Inc(A));
        let mut sub_instructions: Vec<Instruction> = Vec::new();
        while num != 1 {
            if num % 2 == 1 {
                sub_instructions.push(Instruction::Inc(A));
                num -= 1;
            } else {
                sub_instructions.push(Instruction::Shl(A));
                num /= 2;
            }
        }
        sub_instructions.reverse();
        instructions.extend(sub_instructions);
    }
    instructions
}

#[cfg(test)]
mod pas_test {
    use super::{Instruction::*, *};
    #[test]
    fn get_number_testr() {
        assert_eq!(vec![Rst(A)], get_number(0));
        assert_eq!(vec![Rst(A), Inc(A)], get_number(1));
        assert_eq!(vec![Rst(A), Inc(A), Shl(A), Shl(A)], get_number(4));
        assert_eq!(vec![Rst(A), Inc(A), Shl(A), Inc(A), Shl(A)], get_number(6));
        assert_eq!(
            vec![Rst(A), Inc(A), Shl(A), Shl(A), Inc(A), Shl(A)],
            get_number(10)
        );
    }
}
