pub enum Identifier {
    Base(String),
    NumIndexed(String, u64),
    PidIndexed(String, String),
}

pub enum Value {
    Num(u64),
    Id(Identifier),
}

pub enum ConditionOperation {
    Equal,
    NotEqal,
    Greater,
    Lower,
    GreaterOrEqual,
    LowerOrEqual,
}

pub enum Condition {
    Operation(Box<Value>, ConditionOperation, Box<Value>),
}

pub enum ExpressionOperation {
    Add,
    Substact,
    Multiply,
    Divide,
    Modulo,
}

pub enum Expression {
    Val(Value),
    Operation(Box<Value>, ExpressionOperation, Box<Value>),
}

pub type Args = Vec<String>;

pub enum DeclarationVariation {
    Basic(String),
    NumIndexed(String, u64),
}

pub type Declarations = Vec<DeclarationVariation>;

pub type ProcedureCall = (String, Args);

pub type ProcedureHead = (String, Args);

pub enum Commmand {
    Assign(Identifier, Expression),
}
