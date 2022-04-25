use std::option;

use crate::{types::{State, Mutation, RollOut, EvaluationTree}, implementations::constants::ROLLOUT_DEPTH};

/*
fn search(
    &self,
    rollout : RollOut<T>,
    tree : Box<dyn EvaluationTree<T>>,
    loops : u32, rollout_depth : u32,
    rollout_epsilon : f64,
    uct_exploration : f64,
) -> T where T : State {

}

*/

struct TreeSearchNode<'a, T> where T : State {
    parent : Option<&'a TreeSearchNode<'a, T>>,
    times_visited : u32,
    average_evaluation : f64,
    state : T,
    children : Vec<TreeSearchNode<'a, T>>,
    mutations : &'a Vec<Box<Mutation<T>>>
}

/*
1. Selection
Starting at root node R, recursively select optimal child nodes (explained below) until a leaf node L is reached.

2. Expansion
If L is a not a terminal node (i.e. it does not end the game) then create one or more child nodes and select one C.

3. Simulation
Run a simulated playout from C until a result is achieved.

4. Backpropagation
Update the current move sequence with the simulation result.
*/

impl<'a, T> TreeSearchNode<'a, T> where T : State {
    fn new(parent : Option<&'a TreeSearchNode<'a, T>>, state : T, mutations : &'a Vec<Box<Mutation<T>>>) -> TreeSearchNode<'a, T> {
        TreeSearchNode {
            parent,
            times_visited : 0,
            average_evaluation : 0.0,
            state : state,
            children : Vec::new(),
            mutations : mutations
        }
    }

    fn select(&self, uct_exploration : f64) -> &TreeSearchNode<T> {

        if self.children.len() == 0 {
            return self;
        }

        let mut best_ucb_score = 0.0;
        let mut best_node = &self.children[0];

        for child in &self.children {

            let child_ubc_score = ucb(
                child.average_evaluation,
                uct_exploration,
                child.times_visited,
                self.times_visited);

            if child_ubc_score > best_ucb_score {
                best_ucb_score = child_ubc_score;
                best_node = child;
            }
        }

        best_node.select(uct_exploration)
    }

    fn expand(&'a mut self) -> &TreeSearchNode<T> {

        if self.times_visited == 0 {
            return self;
        }

        self.children = get_children_from_mutations(Some(self), self.state, &self.mutations);

        &self.children[0]
    }


    fn update_average(&mut self, value : i32) {
        self.times_visited += 1;
        self.average_evaluation += (value as f64 - self.average_evaluation) / self.times_visited as f64;
    }

    fn simulate(&self, rollout : RollOut<T>, tree : &Box<dyn EvaluationTree<T>>, rollout_epsilon : f64,) -> i32 {
        rollout(self.state, &self.mutations, tree, ROLLOUT_DEPTH, rollout_epsilon)
    }


}


fn ucb(average_evaluation: f64, uct_exploration : f64, times_visited : u32, total_times_visited : u32) -> f64 {
    if times_visited == 0
    {
        return std::f64::MAX;
    }
    
    let log_term = (total_times_visited as f64).ln();
    average_evaluation + uct_exploration * (log_term / (times_visited as f64)).sqrt()
}


fn get_children_from_mutations<'a, T>(parent : Option<&'a TreeSearchNode<'a, T>>, state : T, mutations : &'a Vec<Box<Mutation<T>>>) -> Vec<TreeSearchNode<'a, T>> where T : State {
    mutations.iter().map(|mutation| {
        let child_state = mutation(state);
        TreeSearchNode::new(parent, child_state, mutations)
    }).collect()
}