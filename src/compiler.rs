use std::fmt::format;

use crate::codegenerator::Instruction;
use crate::parser::BaseExpr;
use crate::tac;
use crate::tokenizer::Error;
use crate::typechecker::FunctionType;
use crate::typechecker::Type;
use crate::registerallocation::registerallocator;
use crate::codegenerator;
use crate::livenessanalysis;
use crate::uniquify;
use crate::registerallocation::interferencegraph;
use crate::registerallocation::variableclassifier;
use crate::variablecollector;
use crate::instructionsimplifier;
use crate::instructionsimplifier::AssemblyInstruction;

pub fn compile(
    base_expressions: (Vec<BaseExpr<Type>>, Vec<FunctionType>),
) -> Result<Vec<AssemblyInstruction>, Error> {
    let tac_instructions = tac::generate_tac(base_expressions.0, base_expressions.1)?;

    let all_variable_names = variablecollector::collect_variable_names(&tac_instructions);
    let function_arguments = variableclassifier::get_function_arguments(&tac_instructions);

    let liveness = livenessanalysis::analyze_liveness(&tac_instructions);

    livenessanalysis::print_code_with_liveness(&tac_instructions, &liveness);

	let interference_graph = interferencegraph::build_interference_graph(&tac_instructions, &liveness, &all_variable_names);

    let register_allocation = registerallocator::allocate_registers(&interference_graph, &function_arguments);

    let instructions = codegenerator::generate_code(&tac_instructions, &register_allocation, &liveness)?;

    codegenerator::print_instructions(&instructions);

    let assembly_instructions = instructionsimplifier::to_assembly_instructions(&instructions);

    instructionsimplifier::print_assembly_instructions(&assembly_instructions);

    return Ok(assembly_instructions);
}

fn compile_base_expr(base_expr: BaseExpr<()>) -> Vec<Instruction> {
    match base_expr.data {
        _ => {
            unimplemented!("Only RecExpr is implemented in compile_base_expr");
        }
    }
}
