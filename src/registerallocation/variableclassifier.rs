use std::collections::HashMap;
use crate::tac::TacInstruction;

// Get whether a variable is an argument to a function, and if so, its index
pub fn get_function_arguments(instructions: &Vec<TacInstruction>) -> HashMap<String, usize> {
	let mut function_arguments = HashMap::new();
	for instruction in instructions {
		if let TacInstruction::FunctionLabel(_, parameters) = instruction {
			for (i, param) in parameters.iter().enumerate() {
				function_arguments.insert(param.clone(), i);
			}
		}
	}
	return function_arguments;
}