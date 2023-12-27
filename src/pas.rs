use crate::ast;

const MEMORY_SIZE: u64 = 2_u64.pow(62);

const MUL: &str = r#"
READ
PUT b
READ
PUT c
RST d
HALT
"#;

pub struct PseudoAssembler {
    ast: ast::Program,
}
