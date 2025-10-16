use std::collections::{HashMap, HashSet};
use crate::tac::TacInstruction;
use crate::registerallocation::interferencegraph::InterferenceGraph;

/* Register overview:
- Caller-saved: rax(-1), rcx(0), rdx(1), rsi(2), rdi(3), r8(4), r9(5), r10(-4), r11(-5)
- Callee-saved: rbx(6), rbp(-
*/

struct AllocationNode {
	disallowed_registers: HashSet<isize>,
	assigned_register: Option<isize>,
	neighbors: HashSet<String>,
}

fn is_function_argument(var: &String, function_args: &HashMap<String, usize>) -> bool {
	return function_args.contains_key(var);
}

fn get_function_argument_index(var: &String, function_args: &HashMap<String, usize>) -> Option<usize> {
	return function_args.get(var).cloned();
}

pub fn allocate_registers(interference_graph: &InterferenceGraph, function_args: &HashMap<String, usize>) -> HashMap<String, isize> {

	let mut allocation_network = HashMap::new();
	for (var, neighbors) in interference_graph.adjacency_list.iter() {
		allocation_network.insert(var.clone(), AllocationNode {
			disallowed_registers: HashSet::new(),
			assigned_register: None,
			neighbors: neighbors.clone(),
		});
	}

	let mut allocation = HashMap::new();

	loop {
		let var_to_allocate = get_highest_saturation_variable(&allocation_network);
		let var_to_allocate = match var_to_allocate {
			Some(v) => v,
			// We have allocated all variables
			None => break,
		};

		let node = allocation_network.get(&var_to_allocate).unwrap();
		let mut register_number = 0;
		loop {
			// If we are allowed to allocate this register to this variable, do it
			if !node.disallowed_registers.contains(&register_number) {
				// Assign this register
				let node_mut = allocation_network.get_mut(&var_to_allocate).unwrap();
				node_mut.assigned_register = Some(register_number);
				
				allocation.insert(var_to_allocate.clone(), register_number);

				// Clone neighbors before borrowing mutably
				let neighbors = node_mut.neighbors.clone();

				// Update neighbors
				for neighbor in neighbors.iter() {
					let neighbor_node = allocation_network.get_mut(neighbor).unwrap();
					neighbor_node.disallowed_registers.insert(register_number);
				}
				break;
			}
			register_number += 1;
		}

		// print_allocation_network(&allocation_network);
	}

	print_allocation(&allocation);
	assert!(verify_allocation(&allocation, &interference_graph.adjacency_list));
	
	return allocation;
}

fn verify_allocation(allocation: &HashMap<String, isize>, interference_graph: &HashMap<String, HashSet<String>>) -> bool {
	for (var, reg) in allocation.iter() {
		// Get all neighbors of this variable
		let neighbors = interference_graph.get(var).unwrap();

		// Iterate over them
		for neighbor in neighbors.iter() {
			// If the neighbor is allocated to the same register, fail
			if let Some(neighbor_reg) = allocation.get(neighbor) {
				if reg == neighbor_reg {
					println!("Conflict: Var {} and Var {} both assigned to register {}", var, neighbor, print_register(reg));
					return false;
				}
			}
		}
	}
	return true;
}

fn get_highest_saturation_variable(allocation_network: &HashMap<String, AllocationNode>) -> Option<String> {
	let mut max_saturation = -1;
	let mut candidate: Option<String> = None;

	for (var, node) in allocation_network.iter() {
		if node.assigned_register.is_none() {
			let saturation = node.disallowed_registers.len() as isize;
			if saturation > max_saturation {
				max_saturation = saturation;
				candidate = Some(var.clone());
			}
		}
	}

	candidate
}

fn print_allocation_network(allocation_network: &HashMap<String, AllocationNode>) {
	for (var, node) in allocation_network.iter() {
		let assigned = match &node.assigned_register {
			Some(reg) => reg.clone(),
			None => -6,
		};
		let disallowed: Vec<isize> = node.disallowed_registers.iter().cloned().collect();
		let neighbors: Vec<String> = node.neighbors.iter().cloned().collect();
		println!("Var: {}, Assigned: {}, Disallowed: {:?}, Neighbors: {:?}", var, assigned, disallowed, neighbors);
	}
}

fn print_allocation(allocation: &HashMap<String, isize>) {
	println!("Register Allocation:");
	for (var, reg) in allocation.iter() {
		println!("Var {}: {}", var, print_register(reg));
	}
}

fn print_register(register_number : &isize) -> String {
	match register_number {
		0 => "rcx".to_string(),
		1 => "rdx".to_string(),
		2 => "rsi".to_string(),
		3 => "rdi".to_string(),
		4 => "r8".to_string(),
		5 => "r9".to_string(),
		6 => "r10".to_string(),
		7 => "rbx".to_string(),
		8 => "r12".to_string(),
		9 => "r13".to_string(),
		10 => "r14".to_string(),
		-1 => "rax".to_string(),
		-2 => "rsp".to_string(),
		-3 => "rbp".to_string(),
		-4 => "r11".to_string(),
		-5 => "r15".to_string(),
		-6 => "None".to_string(),
		_ if *register_number > 10 => format!("[rsp + {}]", (*register_number - 10) * 8),
		_ => panic!("Invalid register number"),
	}
}