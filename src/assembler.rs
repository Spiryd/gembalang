use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use crate::ast::*;

use Register::*;

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

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Jumps are relative in our pseudo-Instructions
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
    Jump(i64),
    Jpos(i64),
    Jzero(i64),
    Strk,
    Jumpr,
    Halt,
    Mul,
    Div,
    Mod,
}

impl Instruction {
    fn len(&self) -> u64 {
        match self {
            Instruction::Mul => 18,
            Instruction::Div => 23,
            Instruction::Mod => 17,
            _ => 1,
        }
    }
}

#[derive(Debug)]
enum VariableVariant {
    Atomic(u64),
    Table(u64, u64),
}

#[derive(Debug)]
pub struct Assembler {
    pseudo_assembly: Vec<Instruction>,
    _procedures: HashMap<String, ()>,
    variables: HashMap<String, VariableVariant>,
    ast: Program,
}

impl Assembler {
    pub fn new(ast: Program) -> Assembler {
        let mut _procedures: HashMap<String, ()> = HashMap::new();
        if let Some(procedures_ast) = ast.0.clone() {
            for procedure in procedures_ast {
                _procedures.insert(procedure.0 .0.clone(), ());
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
            _procedures,
            variables,
            ast,
        }
    }
    pub fn assemble(&self) -> String {
        let mut assembly: Vec<String> = Vec::new();
        for instruction in &self.pseudo_assembly {
            match instruction {
                Instruction::Read => assembly.push("READ\n".to_string()),
                Instruction::Write => assembly.push("WRITE\n".to_string()),
                Instruction::Load(register) => assembly.push(format!("LOAD {register}\n")),
                Instruction::Store(register) => assembly.push(format!("STORE {register}\n")),
                Instruction::Add(register) => assembly.push(format!("ADD {register}\n")),
                Instruction::Sub(register) => assembly.push(format!("SUB {register}\n")),
                Instruction::Get(register) => assembly.push(format!("GET {register}\n")),
                Instruction::Put(register) => assembly.push(format!("PUT {register}\n")),
                Instruction::Rst(register) => assembly.push(format!("RST {register}\n")),
                Instruction::Inc(register) => assembly.push(format!("INC {register}\n")),
                Instruction::Dec(register) => assembly.push(format!("DEC {register}\n")),
                Instruction::Shl(register) => assembly.push(format!("SHL {register}\n")),
                Instruction::Shr(register) => assembly.push(format!("SHR {register}\n")),
                Instruction::Jump(offset) => {
                    assembly.push(format!("JUMP {}\n", offset + assembly.len() as i64))
                }
                Instruction::Jpos(offset) => {
                    assembly.push(format!("JPOS {}\n", offset + assembly.len() as i64))
                }
                Instruction::Jzero(offset) => {
                    assembly.push(format!("JZERO {}\n", offset + assembly.len() as i64))
                }
                Instruction::Strk => todo!(),
                Instruction::Jumpr => todo!(),
                Instruction::Halt => assembly.push("HALT\n".to_string()),
                Instruction::Mul => {
                    assembly.push(format!("PUT e\n")); // 0 1
                    assembly.push(format!("RST d\n"));
                    assembly.push(format!("GET c\n")); // 2 3
                    assembly.push(format!("JZERO {}\n", assembly.len() + 14)); // 3 4
                    assembly.push(format!("SHR e\n"));
                    assembly.push(format!("SHL e\n"));
                    assembly.push(format!("GET c\n"));
                    assembly.push(format!("SUB e\n"));
                    assembly.push(format!("JZERO {}\n", assembly.len() + 4)); // 8 9
                    assembly.push(format!("GET d\n"));
                    assembly.push(format!("ADD b\n"));
                    assembly.push(format!("PUT d\n"));
                    assembly.push(format!("SHL b\n"));
                    assembly.push(format!("SHR c\n"));
                    assembly.push(format!("GET c\n"));
                    assembly.push(format!("PUT e\n"));
                    assembly.push(format!("JUMP {}\n", assembly.len() - 14)); // 16  17
                    assembly.push(format!("GET d\n")); // 17 18
                }
                Instruction::Div => {
                    assembly.push(format!("RST d\n")); // 0 1
                    assembly.push(format!("JZERO {}\n", assembly.len() + 21)); // 1 2
                    assembly.push(format!("GET c\n")); // 2 3
                    assembly.push(format!("SUB b\n"));
                    assembly.push(format!("JPOS {}\n", assembly.len() + 17)); // 4 5
                    assembly.push(format!("GET c\n"));
                    assembly.push(format!("PUT e\n"));
                    assembly.push(format!("RST f\n"));
                    assembly.push(format!("INC f\n"));
                    assembly.push(format!("GET e\n")); // 9 10
                    assembly.push(format!("SUB b\n"));
                    assembly.push(format!("JPOS {}\n", assembly.len() + 9)); // 11 12
                    assembly.push(format!("GET b\n"));
                    assembly.push(format!("SUB e\n"));
                    assembly.push(format!("PUT b\n"));
                    assembly.push(format!("GET d\n"));
                    assembly.push(format!("ADD f\n"));
                    assembly.push(format!("PUT d\n"));
                    assembly.push(format!("SHL e\n"));
                    assembly.push(format!("SHL f\n"));
                    assembly.push(format!("JUMP {}\n", assembly.len() - 12)); // 20 21
                    assembly.push(format!("JUMP {}\n", assembly.len() - 20)); // 21 22
                    assembly.push(format!("GET d\n")); // 22 23
                }
                Instruction::Mod => {
                    assembly.push(format!("RST d\n"));
                    assembly.push(format!("JZERO {}\n", assembly.len() + 14)); // 1 2
                    assembly.push(format!("GET c\n"));
                    assembly.push(format!("SUB b\n"));
                    assembly.push(format!("JPOS {}\n", assembly.len() + 11)); // 4 5
                    assembly.push(format!("GET c\n"));
                    assembly.push(format!("PUT d\n")); // 6 7
                    assembly.push(format!("GET d\n"));
                    assembly.push(format!("SUB b\n"));
                    assembly.push(format!("JPOS {}\n", assembly.len() + 5)); // 9 10
                    assembly.push(format!("GET b\n"));
                    assembly.push(format!("SUB d\n"));
                    assembly.push(format!("PUT b\n"));
                    assembly.push(format!("SHL d\n"));
                    assembly.push(format!("JUMP {}\n", assembly.len() - 9)); // 14 15
                    assembly.push(format!("JUMP {}\n", assembly.len() - 15)); // 15 16
                    assembly.push(format!("GET b\n")); // 16 17
                }
            }
        }
        let mut assembled = "".to_string();
        for i in assembly {
            assembled += &i;
        }
        assembled
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
            // Optimise by deleting jump when no else
            Command::If(condition, commands, else_commands) => {
                let mut instructions: Vec<Instruction> = Vec::new();
                let mut sub_instuctions: Vec<Instruction> = Vec::new();
                for command in commands {
                    sub_instuctions.extend(self.construct_command(command));
                }
                let sub_instructions_length: u64 = sub_instuctions.iter().map(|i| i.len()).sum();
                let mut sub_else_instuctions: Vec<Instruction> = Vec::new();
                if let Some(else_commands) = else_commands {
                    for command in else_commands {
                        sub_else_instuctions.extend(self.construct_command(command));
                    }
                }
                let sub_else_instruction_length: u64 =
                    sub_else_instuctions.iter().map(|i| i.len()).sum();
                match condition {
                    Condition::Equal(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Put(C));
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions
                            .push(Instruction::Jpos(sub_instructions_length as i64 + 5));
                        cond_instructions.push(Instruction::Get(B));
                        cond_instructions.push(Instruction::Sub(C));
                        cond_instructions
                            .push(Instruction::Jpos(sub_instructions_length as i64 + 2));

                        instructions.extend(cond_instructions);
                        instructions.extend(sub_instuctions);
                        instructions
                            .push(Instruction::Jump(sub_else_instruction_length as i64 + 1));
                        instructions.extend(sub_else_instuctions);
                    }
                    Condition::NotEqual(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Put(C));
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions
                            .push(Instruction::Jpos(sub_else_instruction_length as i64 + 5));
                        cond_instructions.push(Instruction::Get(B));
                        cond_instructions.push(Instruction::Sub(C));
                        cond_instructions
                            .push(Instruction::Jpos(sub_else_instruction_length as i64 + 2));

                        instructions.extend(cond_instructions);
                        instructions.extend(sub_else_instuctions);
                        instructions.push(Instruction::Jump(sub_instructions_length as i64 + 1));
                        instructions.extend(sub_instuctions);
                    }
                    Condition::Greater(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions
                            .push(Instruction::Jpos(sub_else_instruction_length as i64 + 2));

                        instructions.extend(cond_instructions);
                        instructions.extend(sub_else_instuctions);
                        instructions.push(Instruction::Jump(sub_instructions_length as i64 + 1));
                        instructions.extend(sub_instuctions);
                    }
                    Condition::Lower(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions
                            .push(Instruction::Jpos(sub_else_instruction_length as i64 + 2));

                        instructions.extend(cond_instructions);
                        instructions.extend(sub_else_instuctions);
                        instructions.push(Instruction::Jump(sub_instructions_length as i64 + 1));
                        instructions.extend(sub_instuctions);
                    }
                    Condition::GreaterOrEqual(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions
                            .push(Instruction::Jpos(sub_instructions_length as i64 + 2));

                        instructions.extend(cond_instructions);
                        instructions.extend(sub_instuctions);
                        instructions
                            .push(Instruction::Jump(sub_else_instruction_length as i64 + 1));
                        instructions.extend(sub_else_instuctions);
                    }
                    Condition::LowerOrEqual(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions
                            .push(Instruction::Jpos(sub_instructions_length as i64 + 2));

                        instructions.extend(cond_instructions);
                        instructions.extend(sub_instuctions);
                        instructions
                            .push(Instruction::Jump(sub_else_instruction_length as i64 + 1));
                        instructions.extend(sub_else_instuctions);
                    }
                }
                instructions
            }
            Command::While(condition, commands) => {
                let mut instructions: Vec<Instruction> = Vec::new();
                let mut sub_instuctions: VecDeque<Instruction> = VecDeque::new();
                for command in commands {
                    sub_instuctions.extend(self.construct_command(command));
                }
                let sub_instructions_length: u64 = sub_instuctions.iter().map(|i| i.len()).sum();
                let cond_instructions = match condition {
                    Condition::Equal(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Put(C));
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions
                            .push(Instruction::Jpos(sub_instructions_length as i64 + 5));
                        cond_instructions.push(Instruction::Get(B));
                        cond_instructions.push(Instruction::Sub(C));
                        cond_instructions
                            .push(Instruction::Jpos(sub_instructions_length as i64 + 2));
                        cond_instructions
                    }
                    Condition::NotEqual(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Put(C));
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions
                            .push(Instruction::Jzero(sub_instructions_length as i64 + 5));
                        cond_instructions.push(Instruction::Get(B));
                        cond_instructions.push(Instruction::Sub(C));
                        cond_instructions
                            .push(Instruction::Jzero(sub_instructions_length as i64 + 2));
                        cond_instructions
                    }
                    Condition::Greater(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions.push(Instruction::Jpos(2));
                        cond_instructions
                            .push(Instruction::Jump(sub_instructions_length as i64 + 2));
                        cond_instructions
                    }
                    Condition::Lower(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions.push(Instruction::Jpos(2));
                        cond_instructions
                            .push(Instruction::Jump(sub_instructions_length as i64 + 2));
                        cond_instructions
                    }
                    Condition::GreaterOrEqual(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions
                            .push(Instruction::Jpos(sub_instructions_length as i64 + 2));
                        cond_instructions
                    }
                    Condition::LowerOrEqual(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions
                            .push(Instruction::Jpos(sub_instructions_length as i64 + 2));
                        cond_instructions
                    }
                };
                let cond_instructions_length: u64 = cond_instructions.iter().map(|i| i.len()).sum();
                instructions.extend(cond_instructions);
                instructions.extend(sub_instuctions);
                instructions.push(Instruction::Jump(
                    -((sub_instructions_length + cond_instructions_length) as i64),
                ));
                instructions
            }
            Command::Repeat(commands, condition) => {
                let mut instructions: Vec<Instruction> = Vec::new();
                let mut sub_instuctions: VecDeque<Instruction> = VecDeque::new();
                for command in commands {
                    sub_instuctions.extend(self.construct_command(command));
                }
                let sub_instructions_length: u64 = sub_instuctions.iter().map(|i| i.len()).sum();

                let cond_instructions = match condition {
                    Condition::Equal(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Put(C));
                        cond_instructions.push(Instruction::Sub(B));
                        let cond_instructions_length: u64 =
                            cond_instructions.iter().map(|i| i.len()).sum();
                        cond_instructions
                            .push(Instruction::Jpos(-((cond_instructions_length + sub_instructions_length) as i64)));
                        cond_instructions.push(Instruction::Get(B));
                        cond_instructions.push(Instruction::Sub(C));
                        cond_instructions
                            .push(Instruction::Jpos(-((cond_instructions_length + sub_instructions_length + 3) as i64)));
                        cond_instructions
                    }
                    Condition::NotEqual(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Put(C));
                        cond_instructions.push(Instruction::Sub(B));
                        let cond_instructions_length: u64 =
                            cond_instructions.iter().map(|i| i.len()).sum();
                        cond_instructions
                            .push(Instruction::Jzero(-((cond_instructions_length + sub_instructions_length) as i64)));
                        cond_instructions.push(Instruction::Get(B));
                        cond_instructions.push(Instruction::Sub(C));
                        cond_instructions
                            .push(Instruction::Jzero(-((cond_instructions_length + sub_instructions_length + 3) as i64)));
                        cond_instructions
                    }
                    Condition::Greater(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions.push(Instruction::Jpos(2));
                        let cond_instructions_length: u64 =
                            cond_instructions.iter().map(|i| i.len()).sum();
                        cond_instructions.push(Instruction::Jump(
                            -((cond_instructions_length + sub_instructions_length) as i64),
                        ));
                        cond_instructions
                    }
                    Condition::Lower(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions.push(Instruction::Jpos(2));
                        let cond_instructions_length: u64 =
                            cond_instructions.iter().map(|i| i.len()).sum();
                        cond_instructions.push(Instruction::Jump(
                            -((cond_instructions_length + sub_instructions_length) as i64),
                        ));
                        cond_instructions
                    }
                    Condition::GreaterOrEqual(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Sub(B));
                        let cond_instructions_length: u64 =
                            cond_instructions.iter().map(|i| i.len()).sum();
                        cond_instructions.push(Instruction::Jpos(
                            -((cond_instructions_length + sub_instructions_length) as i64),
                        ));
                        cond_instructions
                    }
                    Condition::LowerOrEqual(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_1));
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_0));
                        cond_instructions.push(Instruction::Sub(B));
                        let cond_instructions_length: u64 =
                            cond_instructions.iter().map(|i| i.len()).sum();
                        cond_instructions.push(Instruction::Jpos(
                            -((cond_instructions_length + sub_instructions_length) as i64),
                        ));
                        cond_instructions
                    }
                };
                instructions.extend(sub_instuctions);
                instructions.extend(cond_instructions);
                instructions
            }
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
            }
            Expression::Substract(value_0, value_1) => {
                let mut instructions = self.extract_value(value_1);
                instructions.push(Instruction::Put(B));
                instructions.extend(self.extract_value(value_0));
                instructions.push(Instruction::Sub(B));
                instructions
            }
            Expression::Multiply(value_0, value_1) => {
                let mut instructions = self.extract_value(value_0);
                instructions.push(Instruction::Put(B));
                instructions.extend(self.extract_value(value_1));
                instructions.push(Instruction::Put(C));
                instructions.push(Instruction::Mul);
                instructions
            }
            Expression::Divide(value_0, value_1) => {
                let mut instructions = self.extract_value(value_0);
                instructions.push(Instruction::Put(B));
                instructions.extend(self.extract_value(value_1));
                instructions.push(Instruction::Put(C));
                instructions.push(Instruction::Div);
                instructions
            }
            Expression::Modulo(value_0, value_1) => {
                let mut instructions = self.extract_value(value_0);
                instructions.push(Instruction::Put(B));
                instructions.extend(self.extract_value(value_1));
                instructions.push(Instruction::Put(C));
                instructions.push(Instruction::Mod);
                instructions
            }
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
