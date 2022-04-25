use std::cmp;
use crate::types::{Mutation, EvaluationTree};

use super::{state_array::StateArray, constants::HEURISTIC_SEARCH_DEPTH};

pub fn rollout_strategy(
    state: StateArray,
    mutations: &Vec<Box<Mutation<StateArray>>>,
    tree : &Box<dyn EvaluationTree<StateArray>>,
    depth : usize,
    rollout_epsilon : f64,
    ) -> i32 {

    // Keep track of where we currently care so that it does not get stuck in local minima.
    let start_value = tree.evaluate(state);

    let mut best_value = i32::MIN;
    let mut best_index = 0;

    if depth <= 0 {
        return start_value;
    }

    for index in 0..mutations.len() {
        let new_state = mutations[index](state);

        let new_state_value = get_rollout_Value(new_state, mutations, tree, HEURISTIC_SEARCH_DEPTH, start_value);

        if new_state_value > best_value {
            best_value = new_state_value;
            best_index = index;
        }
    }

    cmp::max(
        start_value,

    rollout_strategy(
        mutations[best_index](state),
        mutations,
        tree,
        depth -1,
        rollout_epsilon
    ))
}

fn get_rollout_Value(
    state: StateArray,
    mutations: &Vec<Box<Mutation<StateArray>>>,
    tree : &Box<dyn EvaluationTree<StateArray>>,
    depth : usize,
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