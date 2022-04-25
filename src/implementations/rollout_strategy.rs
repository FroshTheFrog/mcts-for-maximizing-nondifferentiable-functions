use std::cmp;
use crate::types::{Mutation, EvaluationTree};

use super::{state_array::StateArray, constants::ROLLOUT_DEPTH};

pub fn rollout_strategy(
    state: StateArray,
    mutations: &Vec<Box<Mutation<StateArray>>>,
    tree : &Box<dyn EvaluationTree<StateArray>>
    ) -> usize {

    // Keep track of where we currently care so that it does not get stuck in local minima.
    let start_value = tree.evaluate(state);

    let mut best_value = i32::min_value();
    let mut best_index = 0;

    for index in 0..mutations.len() {
        let new_state = mutations[index](state);

        let state_value = get_rollout_Value(new_state, mutations, tree, ROLLOUT_DEPTH, start_value);

        if state_value > best_value {
            best_value = state_value;
            best_index = index;
        }
    }

    best_index
}

fn get_rollout_Value(
    state: StateArray,
    mutations: &Vec<Box<Mutation<StateArray>>>,
    tree : &Box<dyn EvaluationTree<StateArray>>,
    depth : i32,
    start_value : i32
) -> i32 {

    let mut best_value = i32::MIN;

    if depth <= 0 {
        return best_value
    }

    for mutation in mutations {
        let new_state = mutation(state);

        let current_state_value = tree.evaluate(new_state);

        if current_state_value > best_value && current_state_value != start_value {
            best_value = current_state_value;
        }

        let down_the_tree_value = get_rollout_Value(new_state, mutations, tree, depth - 1, start_value);

        best_value = cmp::max(best_value, down_the_tree_value);
    }

    best_value
}