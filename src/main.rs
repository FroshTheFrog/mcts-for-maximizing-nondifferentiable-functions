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

    let (smart_searched_state, smart_tree_size) = search(
        start_state,
        rollout_strategy,
        &tree,
        10000,
        40,
        0.2,
        2.0,
        1.0,
        false
    );

    let (random_searched_state, random_tree_size) = search(
        start_state,
        rollout_strategy,
        &tree,
        10000,
        40,
        0.2,
        2.0,
        1.0,
        true
    );
    println!("_______________________________");
    println!("\nSTART: {}\n", tree.evaluate(start_state));

    println!("AFTER SMART SEARCH: {}", tree.evaluate(smart_searched_state));
    println!("WITH A TREE SIZE OF: {}", smart_tree_size);

    println!("\nAFTER RANDOM SEARCH: {}", tree.evaluate(random_searched_state));
    println!("WITH A TREE SIZE OF: {}\n", random_tree_size);
    println!("_______________________________");
}
