use std::{
    collections::{HashMap, HashSet, VecDeque}, fmt::Display
};

use crate::ast::*;

use Register::*;

#[derive(Debug, Clone)]
pub enum CompilerError {
    UndeclaredVariable(String, usize),
    UndeclaredProcedure(String, usize),
    IncorrectUseOfVariable(String, usize),
    IndexOutOfBounds(String, usize),
    ArrayUsedAsIndex(String, usize),
    WrongArgumentType(String, usize),
    DuplicateVariableDeclaration(String, usize),
    DuplicateProcedureDeclaration(String, usize),
    RecursiveProcedureCall(String, usize),
}

impl CompilerError {
    pub fn get_byte(&self) -> usize {
        match self {
            CompilerError::UndeclaredVariable(_, line) => *line,
            CompilerError::UndeclaredProcedure(_, line) => *line,
            CompilerError::IncorrectUseOfVariable(_, line) => *line,
            CompilerError::IndexOutOfBounds(_, line) => *line,
            CompilerError::ArrayUsedAsIndex(_, line) => *line,
            CompilerError::WrongArgumentType(_, line) => *line,
            CompilerError::DuplicateVariableDeclaration(_, line) => *line,
            CompilerError::DuplicateProcedureDeclaration(_, line) => *line,
            CompilerError::RecursiveProcedureCall(_, line) => *line,
        }
    }
}

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
            Instruction::Mod => 24,
            _ => 1,
        }
    }
}

#[derive(Debug)]
enum VariableVariant {
    Atomic(u64),
    Table(u64, u64),
}

#[derive(Debug, Clone)]
struct ProcedureBuilder {
    name: String,
    declared_arguments: Vec<ArgumentsDeclarationVariant>,
    declarations: Option<Declarations>,
    commands: Commands,
}

impl ProcedureBuilder {
    pub fn new(procedure: Procedure) -> ProcedureBuilder {
        let mut pb = ProcedureBuilder {
            name: procedure.0.0.0,
            declared_arguments: procedure.0 .1,
            declarations: procedure.1,
            commands: procedure.2,
        };
        pb.rename_commands();
        pb
    }
    fn rename_commands(&mut self){
        let new_commands: Vec<Command> = self.commands
            .iter()
            .cloned()
            .map(|com| self.rename_command(com))
            .collect();
        self.commands = new_commands;
    }
    fn rename_command(&self, command: Command) -> Command {
        match command {
            Command::Assign(id, expression) => {
                let new_id = self.rename_indentifier(id);
                let new_expression = match expression {
                    Expression::Val(value) => {
                        let new_value = self.rename_value(value);
                        Expression::Val(new_value)
                    }
                    Expression::Add(value0, value1) => {
                        let new_value0 = self.rename_value(value0);
                        let new_value1 = self.rename_value(value1);
                        Expression::Add(new_value0, new_value1)
                    }
                    Expression::Substract(value0, value1) => {
                        let new_value0 = self.rename_value(value0);
                        let new_value1 = self.rename_value(value1);
                        Expression::Substract(new_value0, new_value1)
                    }
                    Expression::Multiply(value0, value1) => {
                        let new_value0 = self.rename_value(value0);
                        let new_value1 = self.rename_value(value1);
                        Expression::Multiply(new_value0, new_value1)
                    }
                    Expression::Divide(value0, value1) => {
                        let new_value0 = self.rename_value(value0);
                        let new_value1 = self.rename_value(value1);
                        Expression::Divide(new_value0, new_value1)
                    }
                    Expression::Modulo(value0, value1) => {
                        let new_value0 = self.rename_value(value0);
                        let new_value1 = self.rename_value(value1);
                        Expression::Modulo(new_value0, new_value1)
                    }
                };
                Command::Assign(new_id, new_expression)
            }
            Command::If(condition, commands, else_commands) => {
                let new_condition = self.rename_condition(condition);
                let new_commands: Vec<Command> = commands
                    .iter()
                    .cloned()
                    .map(|com| self.rename_command(com))
                    .collect();
                let new_else_condition: Option<Vec<Command>> = else_commands.map(|else_commands| else_commands
                            .iter()
                            .cloned()
                            .map(|com| self.rename_command(com))
                            .collect());
                Command::If(new_condition, new_commands, new_else_condition)
            }
            Command::While(condition, commands) => {
                let new_condition = self.rename_condition(condition);
                let new_commands: Vec<Command> = commands
                    .iter()
                    .cloned()
                    .map(|com| self.rename_command(com))
                    .collect();
                Command::While(new_condition, new_commands)
            }
            Command::Repeat(commands, condition) => {
                let new_condition = self.rename_condition(condition);
                let new_commands: Vec<Command> = commands
                    .iter()
                    .cloned()
                    .map(|com| self.rename_command(com))
                    .collect();
                Command::Repeat(new_commands, new_condition)
            }
            Command::ProcCall((name, arguments)) => {
                let new_arguments: Vec<(String, usize)> = arguments.iter().map(|arg| (format!("{}@{}", arg.0, self.name), arg.1)).collect();
                Command::ProcCall((name, new_arguments))
            },
            Command::Read(identifier) => {
                let new_identifier = self.rename_indentifier(identifier);
                Command::Read(new_identifier)
            },
            Command::Write(value) => {
                let new_value = self.rename_value(value);
                Command::Write(new_value)
            },
        }
    }
    fn rename_condition(&self, condition: Condition) -> Condition {
        match condition {
            Condition::Equal(value0, value1) => {
                let new_value0 = self.rename_value(value0);
                let new_value1 = self.rename_value(value1);
                Condition::Equal(new_value0, new_value1)
            }
            Condition::NotEqual(value0, value1) => {
                let new_value0 = self.rename_value(value0);
                let new_value1 = self.rename_value(value1);
                Condition::NotEqual(new_value0, new_value1)
            }
            Condition::Greater(value0, value1) => {
                let new_value0 = self.rename_value(value0);
                let new_value1 = self.rename_value(value1);
                Condition::Greater(new_value0, new_value1)
            }
            Condition::Lower(value0, value1) => {
                let new_value0 = self.rename_value(value0);
                let new_value1 = self.rename_value(value1);
                Condition::Lower(new_value0, new_value1)
            }
            Condition::GreaterOrEqual(value0, value1) => {
                let new_value0 = self.rename_value(value0);
                let new_value1 = self.rename_value(value1);
                Condition::GreaterOrEqual(new_value0, new_value1)
            }
            Condition::LowerOrEqual(value0, value1) => {
                let new_value0 = self.rename_value(value0);
                let new_value1 = self.rename_value(value1);
                Condition::LowerOrEqual(new_value0, new_value1)
            }
        }
    }
    fn rename_value(&self, value: Value) -> Value {
        match value {
            Value::Num(_) => value.clone(),
            Value::Id(id) => Value::Id(self.rename_indentifier(id)),
        }
    }
    fn rename_indentifier(&self, identifier: Identifier) -> Identifier {
        match identifier {
            Identifier::Base(id) => Identifier::Base((format!("{}@{}", id.0, self.name), id.1)),
            Identifier::NumIndexed(id, num) => {
                Identifier::NumIndexed((format!("{}@{}", id.0, self.name), id.1), num)
            }
            Identifier::PidIndexed(id, index_id) => Identifier::PidIndexed(
                (format!("{}@{}", id.0, self.name), id.1),
                (format!("{}@{}", index_id.0, self.name), id.1),
            ),
        }
    }
}

#[derive(Debug)]
pub struct Assembler {
    pseudo_assembly: Vec<Instruction>,
    procedures: HashMap<String, ProcedureBuilder>,
    memory: HashMap<String, VariableVariant>,
    initialisated_variables: HashSet<String>,
    memory_pointer: u64,
    ast: Program,
}

impl Assembler {
    pub fn new(ast: Program) -> Result<Assembler, CompilerError> {
        let mut procedures: HashMap<String, ProcedureBuilder> = HashMap::new();
        if let Some(procedures_ast) = ast.0.clone() {
            for procedure in procedures_ast {
                if procedures.insert(procedure.0.0.0.clone(), ProcedureBuilder::new(procedure.clone())).is_some() {
                    Err(CompilerError::DuplicateProcedureDeclaration(procedure.0.0.0.clone(), procedure.0.0.1.clone()))?;
                }
            }
        }
        let mut memory_pointer: u64 = 0;
        let mut memory: HashMap<String, VariableVariant> = HashMap::new();
        if let Some(vars) = ast.1 .0.clone() {
            for var in vars {
                match var {
                    DeclarationVariant::Base(id) => {
                        memory.insert(id.0, VariableVariant::Atomic(memory_pointer));
                        memory_pointer += 1;
                    }
                    DeclarationVariant::NumIndexed(id, size) => {
                        memory.insert(id.0, VariableVariant::Table(memory_pointer, size));
                        memory_pointer += size;
                    }
                }
            }
        }
        Ok(Assembler {
            pseudo_assembly: vec![],
            procedures,
            memory,
            memory_pointer,
            ast,
            initialisated_variables: HashSet::new(),
        })
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
                    assembly.push("PUT e\n".to_string()); // 0 1
                    assembly.push("RST d\n".to_string());
                    assembly.push("GET c\n".to_string()); // 2 3
                    assembly.push(format!("JZERO {}\n", assembly.len() + 14)); // 3 4
                    assembly.push("SHR e\n".to_string());
                    assembly.push("SHL e\n".to_string());
                    assembly.push("GET c\n".to_string());
                    assembly.push("SUB e\n".to_string());
                    assembly.push(format!("JZERO {}\n", assembly.len() + 4)); // 8 9
                    assembly.push("GET d\n".to_string());
                    assembly.push("ADD b\n".to_string());
                    assembly.push("PUT d\n".to_string());
                    assembly.push("SHL b\n".to_string());
                    assembly.push("SHR c\n".to_string());
                    assembly.push("GET c\n".to_string());
                    assembly.push("PUT e\n".to_string());
                    assembly.push(format!("JUMP {}\n", assembly.len() - 14)); // 16  17
                    assembly.push("GET d\n".to_string()); // 17 18
                }
                Instruction::Div => {
                    assembly.push("RST d\n".to_string()); // 0 1
                    assembly.push(format!("JZERO {}\n", assembly.len() + 21)); // 1 2
                    assembly.push("GET c\n".to_string()); // 2 3
                    assembly.push("SUB b\n".to_string());
                    assembly.push(format!("JPOS {}\n", assembly.len() + 18)); // 4 5
                    assembly.push("GET c\n".to_string());
                    assembly.push("PUT e\n".to_string());
                    assembly.push("RST f\n".to_string());
                    assembly.push("INC f\n".to_string());
                    assembly.push("GET e\n".to_string()); // 9 10
                    assembly.push("SUB b\n".to_string());
                    assembly.push(format!("JPOS {}\n", assembly.len() + 10)); // 11 12
                    assembly.push("GET b\n".to_string());
                    assembly.push("SUB e\n".to_string());
                    assembly.push("PUT b\n".to_string());
                    assembly.push("GET d\n".to_string());
                    assembly.push("ADD f\n".to_string());
                    assembly.push("PUT d\n".to_string());
                    assembly.push("SHL e\n".to_string());
                    assembly.push("SHL f\n".to_string());
                    assembly.push(format!("JUMP {}\n", assembly.len() - 11)); // 20 21
                    assembly.push(format!("JUMP {}\n", assembly.len() - 19)); // 21 22
                    assembly.push("GET d\n".to_string()); // 22 23
                }
                Instruction::Mod => {
                    assembly.push("RST d\n".to_string()); // 0 1
                    assembly.push(format!("JZERO {}\n", assembly.len() + 21)); // 1 2
                    assembly.push("GET c\n".to_string()); // 2 3
                    assembly.push("SUB b\n".to_string());
                    assembly.push(format!("JPOS {}\n", assembly.len() + 19)); // 4 5
                    assembly.push("GET c\n".to_string());
                    assembly.push("PUT e\n".to_string());
                    assembly.push("RST f\n".to_string());
                    assembly.push("INC f\n".to_string());
                    assembly.push("GET e\n".to_string()); // 9 10
                    assembly.push("SUB b\n".to_string());
                    assembly.push(format!("JPOS {}\n", assembly.len() + 10)); // 11 12
                    assembly.push("GET b\n".to_string());
                    assembly.push("SUB e\n".to_string());
                    assembly.push("PUT b\n".to_string());
                    assembly.push("GET d\n".to_string());
                    assembly.push("ADD f\n".to_string());
                    assembly.push("PUT d\n".to_string());
                    assembly.push("SHL e\n".to_string());
                    assembly.push("SHL f\n".to_string());
                    assembly.push(format!("JUMP {}\n", assembly.len() - 11)); // 20 21
                    assembly.push(format!("JUMP {}\n", assembly.len() - 19)); // 21 22
                    assembly.push("RST b\n".to_string()); // 22 23
                    assembly.push("GET b\n".to_string()); // 23 24
                }
            }
        }
        let mut assembled = "".to_string();
        for i in assembly {
            assembled += &i;
        }
        assembled
    }
    pub fn construct(&mut self) -> Result<(), CompilerError>{
        self.construct_main()?;
        self.pseudo_assembly.push(Instruction::Halt);
        Ok(())
    }
    fn construct_main(&mut self) -> Result<(), CompilerError>{
        let commands = self.ast.1 .1.clone();
        let mut constructed_commands: Vec<Vec<Instruction>> = vec![];
        for command in commands {
            constructed_commands.push(self.construct_command(command)?)
        }
        for command in constructed_commands {
            self.pseudo_assembly.extend(command);
        }
        Ok(())
    }
    fn construct_command(&mut self, command: Command) -> Result<Vec<Instruction>, CompilerError> {
        match command {
            Command::Assign(identifier, expression) => {
                let mut instructions: Vec<Instruction> = Vec::new();
                let id = match identifier.clone() {
                    Identifier::Base(id) => id,
                    Identifier::NumIndexed(id, _) => id,
                    Identifier::PidIndexed(id, _) => id,
                };
                self.initialisated_variables.insert(id.0.clone());
                instructions.extend(self.get_pointer_from_identifier(identifier)?);
                instructions.push(Instruction::Put(G));
                instructions.extend(self.construct_expression(expression)?);
                instructions.push(Instruction::Store(G));
                Ok(instructions)
            }
            Command::If(condition, commands, else_commands) => {
                let mut instructions: Vec<Instruction> = Vec::new();
                let mut sub_instuctions: Vec<Instruction> = Vec::new();
                for command in commands {
                    sub_instuctions.extend(self.construct_command(command)?);
                }
                let sub_instructions_length: u64 = sub_instuctions.iter().map(|i| i.len()).sum();
                let mut sub_else_instuctions: Vec<Instruction> = Vec::new();
                if let Some(else_commands) = else_commands {
                    for command in else_commands {
                        sub_else_instuctions.extend(self.construct_command(command)?);
                    }
                }
                let sub_else_instruction_length: u64 =
                    sub_else_instuctions.iter().map(|i| i.len()).sum();
                match condition {
                    Condition::Equal(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1)?);
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
                        cond_instructions.extend(self.extract_value(value_0)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1)?);
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
                        cond_instructions.extend(self.extract_value(value_1)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_0)?);
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
                        cond_instructions.extend(self.extract_value(value_0)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1)?);
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
                        cond_instructions.extend(self.extract_value(value_0)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1)?);
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
                        cond_instructions.extend(self.extract_value(value_1)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_0)?);
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
                Ok(instructions)
            }
            Command::While(condition, commands) => {
                let mut instructions: Vec<Instruction> = Vec::new();
                let mut sub_instuctions: VecDeque<Instruction> = VecDeque::new();
                for command in commands {
                    sub_instuctions.extend(self.construct_command(command)?);
                }
                let sub_instructions_length: u64 = sub_instuctions.iter().map(|i| i.len()).sum();
                let cond_instructions = match condition {
                    Condition::Equal(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1)?);
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
                        cond_instructions.extend(self.extract_value(value_0)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1)?);
                        cond_instructions.push(Instruction::Put(C));
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions.push(Instruction::Jpos(5));
                        cond_instructions.push(Instruction::Get(B));
                        cond_instructions.push(Instruction::Sub(C));
                        cond_instructions.push(Instruction::Jpos(2));
                        cond_instructions
                            .push(Instruction::Jump(sub_instructions_length as i64 + 2));
                        cond_instructions
                    }
                    Condition::Greater(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_1)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_0)?);
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions.push(Instruction::Jpos(2));
                        cond_instructions
                            .push(Instruction::Jump(sub_instructions_length as i64 + 2));
                        cond_instructions
                    }
                    Condition::Lower(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1)?);
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions.push(Instruction::Jpos(2));
                        cond_instructions
                            .push(Instruction::Jump(sub_instructions_length as i64 + 2));
                        cond_instructions
                    }
                    Condition::GreaterOrEqual(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1)?);
                        cond_instructions.push(Instruction::Sub(B));
                        cond_instructions
                            .push(Instruction::Jpos(sub_instructions_length as i64 + 2));
                        cond_instructions
                    }
                    Condition::LowerOrEqual(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_1)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_0)?);
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
                Ok(instructions)
            }
            Command::Repeat(commands, condition) => {
                let mut instructions: Vec<Instruction> = Vec::new();
                let mut sub_instuctions: VecDeque<Instruction> = VecDeque::new();
                for command in commands {
                    sub_instuctions.extend(self.construct_command(command)?);
                }
                let sub_instructions_length: u64 = sub_instuctions.iter().map(|i| i.len()).sum();

                let cond_instructions = match condition {
                    Condition::Equal(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1)?);
                        cond_instructions.push(Instruction::Put(C));
                        cond_instructions.push(Instruction::Sub(B));
                        let cond_instructions_length: u64 =
                            cond_instructions.iter().map(|i| i.len()).sum();
                        cond_instructions.push(Instruction::Jpos(
                            -((cond_instructions_length + sub_instructions_length) as i64),
                        ));
                        cond_instructions.push(Instruction::Get(B));
                        cond_instructions.push(Instruction::Sub(C));
                        cond_instructions.push(Instruction::Jpos(
                            -((cond_instructions_length + sub_instructions_length + 3) as i64),
                        ));
                        cond_instructions
                    }
                    Condition::NotEqual(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_0)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1)?);
                        cond_instructions.push(Instruction::Put(C));
                        cond_instructions.push(Instruction::Sub(B));
                        let cond_instructions_length: u64 =
                            cond_instructions.iter().map(|i| i.len()).sum();
                        cond_instructions.push(Instruction::Jpos(5));
                        cond_instructions.push(Instruction::Get(B));
                        cond_instructions.push(Instruction::Sub(C));
                        cond_instructions.push(Instruction::Jpos(2));
                        cond_instructions.push(Instruction::Jump(
                            -((cond_instructions_length + sub_instructions_length) as i64),
                        ));
                        cond_instructions
                    }
                    Condition::Greater(value_0, value_1) => {
                        let mut cond_instructions: Vec<Instruction> = Vec::new();
                        cond_instructions.extend(self.extract_value(value_1)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_0)?);
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
                        cond_instructions.extend(self.extract_value(value_0)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1)?);
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
                        cond_instructions.extend(self.extract_value(value_0)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_1)?);
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
                        cond_instructions.extend(self.extract_value(value_1)?);
                        cond_instructions.push(Instruction::Put(B));
                        cond_instructions.extend(self.extract_value(value_0)?);
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
                Ok(instructions)
            }
            Command::ProcCall((procedure_id, arguments)) => {
                let mut instructions: Vec<Instruction> = Vec::new();
                let ids: Vec<String> = arguments.iter().map(|arg| arg.0.clone()).collect();
                for id in ids {
                    if  (&id).contains(&format!("@{}", procedure_id.0)) {
                        return Err(CompilerError::RecursiveProcedureCall(procedure_id.0, procedure_id.1));
                    }
                }
                
                let builder = self.procedures.clone().get(&procedure_id.0).ok_or(CompilerError::UndeclaredProcedure(procedure_id.0.clone(), procedure_id.1))?.clone();
                if let Some(declarations) = &builder.declarations {
                    for declaration in declarations {
                        match declaration {
                            DeclarationVariant::Base(id) => {
                                self.memory.insert(format!("{}@{}", id.0, procedure_id.0), VariableVariant::Atomic(self.memory_pointer));
                                self.memory_pointer += 1;
                            },
                            DeclarationVariant::NumIndexed(id, length) => {
                                self.memory.insert(format!("{}@{}", id.0, procedure_id.0), VariableVariant::Table(self.memory_pointer, *length));
                                self.memory_pointer += length;
                            },
                        }
                    }
                }

                for (argument, declared_argument) in arguments.iter().zip(&builder.declared_arguments) {
                    if let Some(declarations) = &builder.declarations{
                        for declaration in declarations {
                            let id = match declaration {
                                DeclarationVariant::Base(id) => id,
                                DeclarationVariant::NumIndexed(id, _) => id,
                            };
                            let arg_id = match declared_argument {
                                ArgumentsDeclarationVariant::Base(id) => id,
                                ArgumentsDeclarationVariant::Table(id) => id,
                            };
                            if id.0 == arg_id.0 {
                                return Err(CompilerError::DuplicateVariableDeclaration(id.0.clone(), id.1));
                            }
                        }
                    }                    
                    let pointee = self.memory.get(argument.0.as_str()).unwrap();
                    match declared_argument {
                        ArgumentsDeclarationVariant::Base(id) => {
                            match pointee {
                                VariableVariant::Atomic(pointer) => {
                                    self.initialisated_variables.insert(argument.0.clone());
                                    self.memory.insert(format!("{}@{}", id.0, procedure_id.0), VariableVariant::Atomic(*pointer));
                                    self.initialisated_variables.insert(format!("{}@{}", id.0, procedure_id.0));
                                },
                                VariableVariant::Table(_, _) => return Err(CompilerError::WrongArgumentType(id.0.clone(), id.1)),
                            }
                        },
                        ArgumentsDeclarationVariant::Table(id) => {
                            match pointee {
                                VariableVariant::Atomic(_) => return Err(CompilerError::WrongArgumentType(id.0.clone(), id.1)),
                                VariableVariant::Table(start, size) => {
                                    self.initialisated_variables.insert(argument.0.clone());
                                    self.memory.insert(format!("{}@{}", id.0, procedure_id.0), VariableVariant::Table(*start, *size));
                                    self.initialisated_variables.insert(format!("{}@{}", id.0, procedure_id.0));
                                },
                            }
                        },
                    }
                }
                for command in &builder.commands {
                    instructions.extend(self.construct_command(command.clone())?);
                }
                Ok(instructions)
            }
            Command::Read(identifier) => {
                let id = match identifier.clone() {
                    Identifier::Base(id) => id,
                    Identifier::NumIndexed(id, _) => id,
                    Identifier::PidIndexed(id, _) => id,
                };
                self.initialisated_variables.insert(id.0.clone());
                let mut instructions: Vec<Instruction> = Vec::new();
                instructions.extend(self.get_pointer_from_identifier(identifier)?);
                instructions.push(Instruction::Put(G));
                instructions.push(Instruction::Read);
                instructions.push(Instruction::Store(G));
                Ok(instructions)
            }
            Command::Write(value) => {
                let mut instructions: Vec<Instruction> = self.extract_value(value)?;
                instructions.push(Instruction::Write);
                Ok(instructions)
            }
        }
    }
    /// Constructs expressions into PseudoAssembly
    fn construct_expression(&self, expression: Expression) -> Result<Vec<Instruction>, CompilerError> {
        match expression {
            Expression::Val(value) => {
                self.check_if_initialised(value.clone());
                self.extract_value(value)
            },
            Expression::Add(value_0, value_1) => {
                self.check_if_initialised(value_0.clone());
                let mut instructions = self.extract_value(value_0)?;
                instructions.push(Instruction::Put(B));
                self.check_if_initialised(value_1.clone());
                instructions.extend(self.extract_value(value_1)?);
                instructions.push(Instruction::Add(B));
                Ok(instructions)
            }
            Expression::Substract(value_0, value_1) => {
                self.check_if_initialised(value_1.clone());
                let mut instructions = self.extract_value(value_1)?;
                instructions.push(Instruction::Put(B));
                self.check_if_initialised(value_0.clone());
                instructions.extend(self.extract_value(value_0)?);
                instructions.push(Instruction::Sub(B));
                Ok(instructions)
            }
            Expression::Multiply(value_0, value_1) => {
                self.check_if_initialised(value_0.clone());
                let mut instructions = self.extract_value(value_0)?;
                instructions.push(Instruction::Put(B));
                self.check_if_initialised(value_1.clone());
                instructions.extend(self.extract_value(value_1)?);
                instructions.push(Instruction::Put(C));
                instructions.push(Instruction::Mul);
                Ok(instructions)
            }
            Expression::Divide(value_0, value_1) => {
                self.check_if_initialised(value_0.clone());
                self.check_if_initialised(value_1.clone());
                let mut instructions = self.extract_value(value_0)?;
                instructions.push(Instruction::Put(B));
                instructions.extend(self.extract_value(value_1)?);
                instructions.push(Instruction::Put(C));
                instructions.push(Instruction::Div);
                Ok(instructions)
            }
            Expression::Modulo(value_0, value_1) => {
                self.check_if_initialised(value_0.clone());
                self.check_if_initialised(value_1.clone());
                let mut instructions = self.extract_value(value_0)?;
                instructions.push(Instruction::Put(B));
                instructions.extend(self.extract_value(value_1)?);
                instructions.push(Instruction::Put(C));
                instructions.push(Instruction::Mod);
                Ok(instructions)
            }
        }
    }
    /// Gets the `value` and puts it into the `A` register
    fn extract_value(&self, value: Value) -> Result<Vec<Instruction>, CompilerError> {
        match value {
            Value::Num(num) => Ok(get_number(num)),
            Value::Id(identifier) => {
                let mut sub_instructions = self.get_pointer_from_identifier(identifier)?;
                sub_instructions.push(Instruction::Load(A));
                Ok(sub_instructions)
            }
        }
    }
    /// Puts the pointer to `identifier` into the `A` register. Sometimes uses the `H` register.
    fn get_pointer_from_identifier(&self, identifier: Identifier) -> Result<Vec<Instruction>, CompilerError>{
        match identifier {
            Identifier::Base(id) => {
                let variable = self.memory.get(&id.0).ok_or(CompilerError::UndeclaredVariable(id.0.clone(), id.1))?;
                match variable {
                    VariableVariant::Atomic(pointer) => Ok(get_number(*pointer)),
                    VariableVariant::Table(_, _) => Err(CompilerError::IncorrectUseOfVariable(id.0, id.1)),
                }
            },
            Identifier::NumIndexed(id, num) => {
                let variable = self.memory.get(&id.0).ok_or(CompilerError::UndeclaredVariable(id.0.clone(), id.1))?;
                let (start, size) = match variable {
                    VariableVariant::Atomic(_) => {
                        return Err(CompilerError::IncorrectUseOfVariable(id.0, id.1));
                    }
                    VariableVariant::Table(pointer, size) => (*pointer, *size),
                };
                if num >= size {
                    return Err(CompilerError::IndexOutOfBounds(id.0, id.1));
                }
                Ok(get_number(start + num))
            }
            Identifier::PidIndexed(id, index_id) => {
                let mut instructions: Vec<Instruction> = Vec::new();
                if !self.initialisated_variables.contains(&index_id.0) {
                    let id_for_warning = index_id.0.split('@').next().unwrap().to_string();
                    println!("Warning: Variable {} used before initialisation", id_for_warning)
                }
                let variable = self.memory.get(&index_id.0).ok_or(CompilerError::UndeclaredVariable(index_id.0.clone(), index_id.1))?;
                match variable {
                    VariableVariant::Atomic(pointer) => instructions.extend(get_number(*pointer)),
                    VariableVariant::Table(_, _) => {
                        return Err(CompilerError::ArrayUsedAsIndex(id.0, id.1));
                    }
                };
                instructions.push(Instruction::Load(A));
                instructions.push(Instruction::Put(H));
                match self.memory.get(&id.0).unwrap() {
                    VariableVariant::Atomic(_) => {
                        return Err(CompilerError::IncorrectUseOfVariable(id.0, id.1));
                    }
                    VariableVariant::Table(pointer, _) => {
                        instructions.extend(get_number(*pointer));
                    }
                }
                instructions.push(Instruction::Add(H));
                Ok(instructions)
            }
        }
    }
    fn check_if_initialised(&self, value: Value) {
        match value {
            Value::Num(_) => {},
            Value::Id(identifier) => {
                let id = match identifier.clone() {
                    Identifier::Base(id) => id,
                    Identifier::NumIndexed(id, _) => id,
                    Identifier::PidIndexed(id, _) => id,
                };
                if !self.initialisated_variables.contains(&id.0) {
                    let id_for_warning = id.0.split('@').next().unwrap().to_string();
                    println!("Warning: Variable {} used before initialisation", id_for_warning)
                }
            },
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
