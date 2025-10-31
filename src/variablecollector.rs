use crate::tac;

pub fn collect_variable_names(tac_instructions: &Vec<tac::TacInstruction>) -> std::collections::HashSet<tac::VariableValue> {
	let mut variable_names = std::collections::HashSet::new();

	for instruction in tac_instructions {
		match instruction {
			tac::TacInstruction::Assign(dest, value) => {
				variable_names.insert(dest.clone());
				if let tac::TacValue::Variable(var) = value {
					variable_names.insert(tac::VariableValue::Variable(var.clone()));
				}
			}
			tac::TacInstruction::BinOp(dest, left, _, right) => {
				variable_names.insert(dest.clone());
				if let tac::TacValue::Variable(var) = left {
					variable_names.insert(tac::VariableValue::Variable(var.clone()));
				}
				if let tac::TacValue::Variable(var) = right {
					variable_names.insert(tac::VariableValue::Variable(var.clone()));
				}
			}
			tac::TacInstruction::UnaryOp(dest, _, value) => {
				variable_names.insert(dest.clone());
				if let tac::TacValue::Variable(var) = value {
					variable_names.insert(tac::VariableValue::Variable(var.clone()));
				}
			}
			tac::TacInstruction::FunctionLabel(_, parameters) => {
				for param in parameters {
					variable_names.insert(tac::VariableValue::Variable(param.clone()));
				}
			}
			tac::TacInstruction::Call(_, _, Some(ret_var)) => {
				variable_names.insert(ret_var.clone());
			}
			tac::TacInstruction::ExternCall(_, _, Some(ret_var)) => {
				variable_names.insert(ret_var.clone());
			}
			_ => {}
		}
	}

	return variable_names;
}