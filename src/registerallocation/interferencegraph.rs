use std::collections::HashSet;
use std::hash::Hash;
use crate::tac::TacInstruction;
use crate::tac::TacValue;
use crate::tac::VariableValue;

pub struct InterferenceGraph {
	pub adjacency_list: std::collections::HashMap<VariableValue, HashSet<VariableValue>>,
}

fn add_interference(graph: &mut InterferenceGraph, var1: &VariableValue, var2: &VariableValue) {
	graph.adjacency_list.entry(var1.clone()).or_default().insert(var2.clone());
	graph.adjacency_list.entry(var2.clone()).or_default().insert(var1.clone());
}

pub fn build_interference_graph(instructions: &Vec<TacInstruction>, liveness: &Vec<HashSet<VariableValue>>, all_variable_names: &HashSet<VariableValue>) -> InterferenceGraph {
	return build_interference_graph_naive(instructions, liveness, all_variable_names);
}

fn build_interference_graph_naive(instructions: &Vec<TacInstruction>, liveness: &Vec<HashSet<VariableValue>>, all_variable_names: &HashSet<VariableValue>) -> InterferenceGraph {
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