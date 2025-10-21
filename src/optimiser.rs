use crate::instructionsimplifier::AssemblyInstruction;

pub fn optimise_assembly(assembly: &Vec<AssemblyInstruction>) -> Vec<AssemblyInstruction> {
	// Placeholder for optimisation logic
	println!("Optimising assembly with {} instructions.", assembly.len());
	// Example optimisation: Remove redundant NOPs (No Operation)
	let mut optimized_instructions = Vec::new();
	let mut previous_was_nop = false;

	for instr in assembly {
		match instr {
			AssemblyInstruction::Nop => {
				if !previous_was_nop {
					optimized_instructions.push(instr.clone());
					previous_was_nop = true;
				}
			}
			AssemblyInstruction::Mov(arg1, arg2) => {
				// Example: Remove redundant moves (e.g., MOV R1, R1)
				if arg1 != arg2 {
					optimized_instructions.push(instr.clone());
				}
				previous_was_nop = false;
			}
			_ => {
				optimized_instructions.push(instr.clone());
				previous_was_nop = false;
			}
		}
	}

	// TODO: remove unnecessary pushes and pops

	println!(
		"Optimisation complete. Reduced from {} to {} instructions.",
		assembly.len(),
		optimized_instructions.len()
	);

	return optimized_instructions;
}
