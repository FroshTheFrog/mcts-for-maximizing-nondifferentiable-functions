mod types;
mod implementations;

use crate::{types::State, implementations::{state_array, evaluations_tree}};

fn main() {
    let state = state_array::StateArray::random_state();

    println!("{}", state);

    let tree = evaluations_tree::build_evaluations_tree(10);

    println!("{}", tree.evaluate(state));
}
