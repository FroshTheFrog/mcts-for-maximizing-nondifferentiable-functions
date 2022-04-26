mod implementations;
mod tree_search;
mod types;

use crate::{
    implementations::{evaluations_tree, rollout_strategy::rollout_strategy, state_array},
    tree_search::search,
    types::State,
};

fn main() {
    let start_state = state_array::StateArray::random_state();

    let tree = evaluations_tree::build_evaluations_tree(11);

    let (searched_state, tree_size) = search(
        start_state,
        rollout_strategy,
        &tree,
        100000,
        40,
        0.2,
        2.0,
        1.0,
    );

    println!("START: {}", tree.evaluate(start_state));

    println!("AFTER SEARCH: {}", tree.evaluate(searched_state));

    println!("WITH A TREE SIZE OF: {}", tree_size);
}
