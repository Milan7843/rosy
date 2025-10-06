use std::fmt::format;

use crate::assembler;
use crate::assembler::Argument;
use crate::assembler::Instruction;
use crate::assembler::Register;
use crate::assembler::RegisterType;
use crate::parser;
use crate::parser::BaseExpr;
use crate::parser::BaseExprData;
use crate::parser::RecExpr;
use crate::parser::RecExprData;
use crate::tac;
use crate::tokenizer::Error;
use crate::typechecker;

pub fn compile(base_expressions: Vec<BaseExpr<()>>) -> Result<Vec<Instruction>, Error> {
    let tac_instructions = tac::generate_tac(base_expressions)?;

    let mut instructions = Vec::new();

    return Ok(instructions);
}

fn compile_base_expr(base_expr: BaseExpr<()>) -> Vec<Instruction> {
    match base_expr.data
    {
        _ =>
        {
            unimplemented!("Only RecExpr is implemented in compile_base_expr");
        }
    }
}
