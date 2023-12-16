pub type Num = u64;

pub type Pidentifier = String;

#[derive(Debug)]
pub enum Identifier {
    Base(Pidentifier),
    NumIndexed(Pidentifier, Num),
    PidIndexed(Pidentifier, Pidentifier),
}

#[derive(Debug)]
pub enum Value {
    Num(Num),
    Id(Identifier),
}
#[derive(Debug)]
pub enum Condition {
    Equal(Value, Value),
    NotEqual(Value, Value),
    Greater(Value, Value),
    Lower(Value, Value),
    GreaterOrEqual(Value, Value),
    LowerOrEqual(Value, Value),
}

#[derive(Debug)]
pub enum Expression {
    Val(Value),
    Add(Value, Value),
    Substract(Value, Value),
    Multiply(Value, Value),
    Divide(Value, Value),
    Modulo(Value, Value),
}

pub type Args = Vec<Pidentifier>;

pub enum DeclarationVariation {
    Basic(Pidentifier),
    NumIndexed(Pidentifier, Num),
}

pub type Declarations = Vec<DeclarationVariation>;

pub type ProcedureCall = (Pidentifier, Args);

pub type ProcedureHead = (Pidentifier, Args);

pub enum Command {
    Assign(Identifier, Expression),
    IfElse(Condition, Vec<Command>, Vec<Command>),
    If(Condition, Vec<Command>),
    While(Condition, Vec<Command>),
    Repeat(Vec<Command>, Condition),
    ProcCall(ProcedureCall),
    Read(Identifier),
    Write(Value),
}

pub enum Main {
    WithDeclarations(Declarations, Vec<Command>),
    WithOutDeclarations(Vec<Command>),
}

pub enum Procedure {
    WithDeclarations(ProcedureHead, Declarations, Vec<Command>),
    WithOutDeclarations(ProcedureHead, Vec<Command>),
}

pub type Program = (Vec<Procedure>, Main);
