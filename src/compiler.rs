use std::fmt::format;

use crate::assembler::Instruction;
use crate::parser::BaseExpr;
use crate::tac;
use crate::tokenizer::Error;
use crate::typechecker::FunctionType;
use crate::typechecker::Type;
use crate::registerallocation::registerallocator;
use crate::codegenerator;

pub fn compile(
    base_expressions: (Vec<BaseExpr<Type>>, Vec<FunctionType>),
) -> Result<Vec<Instruction>, Error> {
    let tac_instructions = tac::generate_tac(base_expressions.0, base_expressions.1)?;

    let register_allocation = registerallocator::allocate_registers(&tac_instructions);

    let instructions = codegenerator::generate_code(tac_instructions, register_allocation)?;

    return Ok(instructions);
}

fn compile_base_expr(base_expr: BaseExpr<()>) -> Vec<Instruction> {
    match base_expr.data {
        _ => {
            unimplemented!("Only RecExpr is implemented in compile_base_expr");
        }
    }
}
