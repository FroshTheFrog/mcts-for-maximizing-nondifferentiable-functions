mod types;
mod implementations;
mod tree_search;

use crate::{types::State, implementations::{state_array, evaluations_tree, rollout_strategy::rollout_strategy}, tree_search::search};

/*
    stat_state : T,
    rollout : RollOut<T>,
    tree : &Box<dyn EvaluationTree<T>>,
    loops : u32,
    rollout_epsilon : f64,
    uct_exploration : f64,
*/

fn main() {
    let start_state = state_array::StateArray::random_state();

    let tree = evaluations_tree::build_evaluations_tree(10);

    let searched_state = search(start_state, rollout_strategy, &tree, 500, 0.3, 2.0);

    println!("{}", tree.evaluate(start_state));
}
