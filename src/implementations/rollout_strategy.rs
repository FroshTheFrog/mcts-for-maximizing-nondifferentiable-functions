use std::{cmp, collections::HashSet};
use crate::types::{Mutation, EvaluationTree, State};
use rand::Rng;

use super::{constants::HEURISTIC_SEARCH_DEPTH};

pub fn rollout_strategy<T>(
    state: T,
    mutations: &Vec<Box<Mutation<T>>>,
    tree : &Box<dyn EvaluationTree<T>>,
    depth : usize,
    rollout_epsilon : f64,
    previous_states : &HashSet<T>,
    ) -> i32 where T : State {

    let start_value = tree.evaluate(state);

    let mut best_value = i32::MIN;
    let mut best_mutation = &mutations[0];

    if depth == 0 {
        return start_value;
    }

    for mutation in mutations {
        let new_state = mutation(state);

        let new_state_value = get_rollout_value(new_state, mutations, tree, HEURISTIC_SEARCH_DEPTH, previous_states);

        if new_state_value > best_value {
            best_value = new_state_value;
            best_mutation = mutation;
        }
    }

    let mut rng = rand::thread_rng();

    if rollout_epsilon <= rng.gen_range(0.0..1.0) {
        best_mutation = &mutations[rng.gen_range(0..mutations.len())];
    }

    cmp::max(
        start_value,

    rollout_strategy(
        best_mutation(state),
        mutations,
        tree,
        depth -1,
        rollout_epsilon,
        previous_states
    ))
}

fn get_rollout_value<T>(
    state: T,
    mutations: &Vec<Box<Mutation<T>>>,
    tree : &Box<dyn EvaluationTree<T>>,
    depth : usize,
    previous_states : &HashSet<T>,
    ) -> i32 where T : State {

    let mut best_value = tree.evaluate(state);

    if depth == 0 {
        return best_value
    }

    for mutation in mutations {

        let new_state = mutation(state);

        let down_the_tree_value = get_rollout_value(new_state, mutations, tree, depth - 1, previous_states);

        if down_the_tree_value > best_value && !previous_states.contains(&new_state) {
            best_value = down_the_tree_value;
        }
    }

    best_value
}