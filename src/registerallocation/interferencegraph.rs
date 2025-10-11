use std::collections::HashSet;
use std::hash::Hash;
use crate::tac::TacInstruction;
use crate::tac::TacValue;

pub struct InterferenceGraph {
	pub adjacency_list: std::collections::HashMap<String, HashSet<String>>,
}

fn add_interference(graph: &mut InterferenceGraph, var1: &String, var2: &String) {
	graph.adjacency_list.entry(var1.clone()).or_default().insert(var2.clone());
	graph.adjacency_list.entry(var2.clone()).or_default().insert(var1.clone());
}

pub fn build_interference_graph(instructions: &Vec<TacInstruction>, liveness: &Vec<HashSet<String>>, all_variable_names: &HashSet<String>) -> InterferenceGraph {
	return build_interference_graph_naive(instructions, liveness, all_variable_names);
	let mut graph = InterferenceGraph {
		adjacency_list: std::collections::HashMap::new(),
	};

	for (i, instruction) in instructions.iter().enumerate() {
		let live_after = &liveness[i+1];
		match instruction {
			TacInstruction::Assign(dest, source) => {
				match source {
					TacValue::Variable(src_var) => {
						for live_var in live_after {
							if live_var != dest && live_var != src_var {
								add_interference(&mut graph, dest, live_var);
							}
						}
					}
					TacValue::Constant(_) => {
						for live_var in live_after {
							if live_var != dest {
								add_interference(&mut graph, dest, live_var);
							}
						}
					}
					_ => {}
				}
			}
			TacInstruction::BinOp(dest, left, _, right) => {
				let mut sources = Vec::new();
				if let TacValue::Variable(var) = left {
					sources.push(var);
				}
				if let TacValue::Variable(var) = right {
					sources.push(var);
				}
				for live_var in live_after {
					if live_var != dest && !sources.contains(&live_var) {
						add_interference(&mut graph, dest, live_var);
					}
				}
			}
			TacInstruction::FunctionLabel(name, parameters) => {
				for param in parameters {
					for live_var in live_after {
						if live_var != param {
							add_interference(&mut graph, param, live_var);
						}
					}
				}
			}
			_ => { /* Handle other instruction types as needed */ }
		}
	}

	graph
}

fn build_interference_graph_naive(instructions: &Vec<TacInstruction>, liveness: &Vec<HashSet<String>>, all_variable_names: &HashSet<String>) -> InterferenceGraph {
	let mut graph = InterferenceGraph {
		adjacency_list: std::collections::HashMap::new(),
	};

	// First add all variables to the graph
	for var in all_variable_names {
		graph.adjacency_list.entry(var.clone()).or_default();
	}

	for (i, instruction) in instructions.iter().enumerate() {
		for live_var in liveness[i].iter() {
			for other_live_var in liveness[i].iter() {
				if live_var != other_live_var {
					add_interference(&mut graph, live_var, other_live_var);
				}
			}
		}
	}

	graph
}