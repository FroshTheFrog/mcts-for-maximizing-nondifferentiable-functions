mod types;
mod implementations;
mod tree_search;

use crate::{types::State, implementations::{state_array, evaluations_tree, rollout_strategy::rollout_strategy}, tree_search::search};


fn main() {
    let start_state = state_array::StateArray::random_state();

    let tree = evaluations_tree::build_evaluations_tree(11);

    let (searched_state, tree_size) = search(start_state, rollout_strategy, &tree, 200000, 0.3, 2.0);

    println!("START: {}", tree.evaluate(start_state));

    println!("AFTER SEARCH: {}", tree.evaluate(searched_state));

    println!("WITH A TREE SIZE OF: {}", tree_size);
}
