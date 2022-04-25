use std::cmp;
use crate::types::{Mutation, EvaluationTree, State};

use super::{constants::HEURISTIC_SEARCH_DEPTH};

pub fn rollout_strategy<T>(
    state: T,
    mutations: &Vec<Box<Mutation<T>>>,
    tree : &Box<dyn EvaluationTree<T>>,
    depth : usize,
    rollout_epsilon : f64,
    ) -> i32 where T : State {

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

fn get_rollout_Value<T>(
    state: T,
    mutations: &Vec<Box<Mutation<T>>>,
    tree : &Box<dyn EvaluationTree<T>>,
    depth : usize,
    start_value : i32
    ) -> i32 where T : State {

    let mut best_value = tree.evaluate(state);

    if depth <= 0 {
        return best_value
    }

    for mutation in mutations {

        let new_state = mutation(state);

        let down_the_tree_value = get_rollout_Value(new_state, mutations, tree, depth - 1, start_value);

        if down_the_tree_value > best_value && down_the_tree_value != start_value {
            best_value = down_the_tree_value;
        }
    }

    best_value
}