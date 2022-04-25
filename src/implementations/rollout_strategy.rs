use std::cmp;
use crate::types::{Mutation, EvaluationTree};

use super::{state_array::StateArray, constants::ROLLOUT_DEPTH};

pub fn rollout_strategy(
    state: StateArray,
    mutations: &Vec<Box<Mutation<StateArray>>>,
    tree : &Box<dyn EvaluationTree<StateArray>>
    ) -> i32 {

    // Keep track of where we currently care so that it does not get stuck in local minima.
    let start_value = tree.evaluate(state);

    lets_rollout(state, mutations, tree, ROLLOUT_DEPTH, start_value)
}

fn lets_rollout(
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

        let down_the_tree = lets_rollout(new_state, mutations, tree, depth - 1, start_value);

        best_value = cmp::max(best_value, down_the_tree);

        let current_state_value = tree.evaluate(new_state);

        if current_state_value > best_value && current_state_value != start_value {
            best_value = current_state_value;
        }
    }

    best_value
}