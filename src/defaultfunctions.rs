use core::num;

use crate::typechecker::FunctionType;
use crate::typechecker::Type;
use crate::tac::TacFunctionEnvironment;
use crate::tac::TacInstruction;
use crate::tac::TacFunction;
use crate::tac::TacValue;


pub fn is_default_function(name: &str) -> bool {
	match name {
		"print" | "println" => true,
		_ => false,
	}
}

pub fn add_default_functions(
    functions: Vec<FunctionType>,
    function_env: &mut TacFunctionEnvironment,
    instructions: &mut Vec<TacInstruction>,
    temp_counter: &mut i64,
    label_counter: &mut i64)
{
	for func in functions {
		if !func.is_used {
			continue;
		}
		if is_default_function(&func.name) {
			match func.name.as_str() {
				"print" => {
					default_print_function(func, function_env, instructions, temp_counter, label_counter);
				}
				"println" => {
					//default_println_function(func, function_env, instructions, temp_counter, label_counter);
				}
				_ => {}
			}
		}
	}
}

fn default_print_function(
	func: FunctionType,
	function_env: &mut TacFunctionEnvironment,
	instructions: &mut Vec<TacInstruction>,
	temp_counter: &mut i64,
	label_counter: &mut i64)
{
    let label = format!("func_print_int");

	let param_name = func.param_names[0].clone();
	function_env.functions.push(TacFunction {
		name: "print".to_string(),
		params: vec![Type::Integer],
		return_type: Type::Undefined,
		label: label.clone(),
	});

	instructions.push(TacInstruction::FunctionLabel(label.clone(), vec![param_name.clone()]));

	// GetStdHandle
	let stdhandle_temp = format!("stdhandle{}", temp_counter);
	*temp_counter += 1;
	let stdhandle_location_temp = format!("stdhandle{}", temp_counter);
	*temp_counter += 1;

	instructions.push(TacInstruction::Assign(stdhandle_temp.clone(), TacValue::Constant(-11)));
	instructions.push(TacInstruction::ExternCall("GetStdHandle".to_string(), vec![TacValue::Variable(stdhandle_temp)], Some(stdhandle_location_temp.clone())));

	// WriteFile
	let number_of_bytes_temp = format!("numberofbytes{}", temp_counter);
	*temp_counter += 1;
	let bytes_written_temp = format!("byteswritten{}", temp_counter);
	*temp_counter += 1;
	let lp_overlapped_temp = format!("lpoverlapped{}", temp_counter);
	*temp_counter += 1;
	let stack_offset_temp = format!("stackoffset{}", temp_counter);
	*temp_counter += 1;
	// Integer to write
	instructions.push(TacInstruction::Push(TacValue::Variable(param_name.clone())));
	instructions.push(TacInstruction::MovRSPTo(stack_offset_temp.clone()));
	// Number of bytes to write
	instructions.push(TacInstruction::Assign(number_of_bytes_temp.clone(), TacValue::Constant(4)));
	// Pointer to number of bytes written (NULL)
	instructions.push(TacInstruction::Assign(bytes_written_temp.clone(), TacValue::Constant(0)));
	// lpOverlapped (NULL)
	instructions.push(TacInstruction::Assign(lp_overlapped_temp.clone(), TacValue::Constant(0)));

	let argument_values = vec![
		TacValue::Variable(stdhandle_location_temp.clone()),
		TacValue::Variable(stack_offset_temp.clone()),
		TacValue::Variable(number_of_bytes_temp.clone()),
		TacValue::Variable(bytes_written_temp.clone()),
		TacValue::Variable(lp_overlapped_temp.clone()),
	];

	// Create 4 temporary variables for arguments
	let mut arg_temps = Vec::new();
	for (i, arg) in argument_values.iter().enumerate() {
		println!("Processing argument {}: {:?}", i, arg);
		if i >= 4 {
			break; // Only handle up to 4 arguments for now (since they go in registers)
		}
		let temp_var = format!("arg{}", temp_counter);
		*temp_counter += 1;
		instructions.push(TacInstruction::Assign(temp_var.clone(), arg.clone()));
		arg_temps.push(TacValue::Variable(temp_var));
	}

	// Call WriteFile
	instructions.push(TacInstruction::ExternCall("WriteFile".to_string(), vec![
		arg_temps[0].clone(),
		arg_temps[1].clone(),
		arg_temps[2].clone(),
		arg_temps[3].clone(),
		TacValue::Variable(lp_overlapped_temp),
	], None));

	// Clean up the stack
	instructions.push(TacInstruction::Pop(param_name));

	instructions.push(TacInstruction::Return(None));
}
