use core::num;
use std::usize;

use crate::tac::BinOp;
use crate::typechecker::FunctionType;
use crate::typechecker::Type;
use crate::tac::TacFunctionEnvironment;
use crate::tac::TacInstruction;
use crate::tac::TacFunction;
use crate::tac::TacValue;
use crate::tac::VariableValue;
use crate::codegenerator::from_register;
use crate::codegenerator::Instruction;
use crate::codegenerator::Argument;
use crate::codegenerator::Register;
use crate::codegenerator::RegisterType;
use crate::codegenerator::RegisterSize;

fn add_direct(
	instructions: &mut Vec<TacInstruction>,
	instruction: Instruction)
{
	instructions.push(TacInstruction::DirectInstruction(instruction));
}

pub fn add_library_functions(
    functions: Vec<FunctionType>,
    function_env: &mut TacFunctionEnvironment,
    instructions: &mut Vec<TacInstruction>,
    temp_counter: &mut i64,
    label_counter: &mut i64)
{

}