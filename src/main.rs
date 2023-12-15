pub mod ast;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub lexparse);

fn main() {    
    println!("{:?}", lexparse::ExpressionParser::new().parse("22").unwrap());
}
