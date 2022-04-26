use std::collections::HashSet;

use crate::{
    implementations::constants::{ROLLOUT_DEPTH, LOOP_PRINT_INTERVAL},
    types::{EvaluationTree, Mutation, RollOut, State},
};

pub fn search<T>(
    start_state: T,
    rollout: RollOut<T>,
    tree: &Box<dyn EvaluationTree<T>>,
    loops: u32,
    rollout_epsilon: f64,
    uct_exploration: f64,
) -> (T, u32)
where
    T: State,
{
    let mutations = T::get_possible_mutations();

    let mut previous_states = HashSet::new();

    let mut base_node = TreeSearchNode::new(start_state, &mutations, &mut previous_states);

    for loop_number in 0..loops {
        
        if loop_number % LOOP_PRINT_INTERVAL == 0 {
            println!("Loop {}", loop_number);
        }

        base_node.run(uct_exploration, rollout, tree, rollout_epsilon, &mut previous_states);
    }

    let best_state = base_node.get_max_state(tree);
    let tree_size = base_node.get_tree_size();

    (best_state, tree_size)
}

struct TreeSearchNode<'a, T>
where
    T: State,
{
    times_visited: u32,
    average_evaluation: f64,
    state: T,
    children: Vec<TreeSearchNode<'a, T>>,
    mutations: &'a Vec<Box<Mutation<T>>>,
}

impl<'a, T> TreeSearchNode<'a, T>
where
    T: State,
{
    fn new(state: T, mutations: &'a Vec<Box<Mutation<T>>>, previous_states: &mut HashSet<T>) -> TreeSearchNode<'a, T> {
        let new_node = TreeSearchNode {
            times_visited: 0,
            average_evaluation: 0.0,
            state: state,
            children: Vec::new(),
            mutations: mutations,
        };

        previous_states.insert(state);

        new_node
    }

    fn run(
        &mut self,
        uct_exploration: f64,
        rollout: RollOut<T>,
        tree: &Box<dyn EvaluationTree<T>>,
        rollout_epsilon: f64,
        previous_states: &mut HashSet<T>,
    ) -> i32 {
        if self.children.len() == 0 {
            let (expanded, no_children) = self.expand(previous_states);

            if no_children {
                let value = -(self.average_evaluation as i32) / ((self.times_visited as i32) + 1);
                self.update_average(value);
                return value;
            }

            let value = expanded.simulate(rollout, tree, rollout_epsilon, previous_states);
            self.update_average(value);
            return value;
        }

        let best_index = self.best_ucb_score_index(uct_exploration);

        let value = self.children[best_index].run(uct_exploration, rollout, tree, rollout_epsilon, previous_states);
        self.update_average(value);
        value
    }

    fn expand(&mut self, previous_states: &mut HashSet<T>) -> (&TreeSearchNode<T>, bool) {
        if self.times_visited == 0 {
            return (self, false);
        }

        self.children = get_children_from_mutations(self.state, self.mutations, previous_states);

        if self.children.len() == 0 {
            return (self, true);
        }

        (&self.children[0], false)
    }

    fn simulate(
        &self,
        rollout: RollOut<T>,
        tree: &Box<dyn EvaluationTree<T>>,
        rollout_epsilon: f64,
        previous_states: &HashSet<T>
    ) -> i32 {
        rollout(
            self.state,
            &self.mutations,
            tree,
            ROLLOUT_DEPTH,
            rollout_epsilon,
            previous_states
        )
    }

    fn best_ucb_score_index(&self, uct_exploration: f64) -> usize {
        let mut best_ucb_score = 0.0;
        let mut best_index = 0;

        for index in 0..self.children.len() {
            let child = &self.children[index];

            let child_ubc_score = ucb(
                child.average_evaluation,
                uct_exploration,
                child.times_visited,
                self.times_visited,
            );

            if child_ubc_score > best_ucb_score {
                best_ucb_score = child_ubc_score;
                best_index = index;
            }
        }

        best_index
    }

    fn update_average(&mut self, value: i32) {
        self.times_visited += 1;
        self.average_evaluation +=
            (value as f64 - self.average_evaluation) / self.times_visited as f64;
    }

    fn get_max_state(&self, tree: &Box<dyn EvaluationTree<T>>) -> T {
        let mut best_state = self.state;

        for child in &self.children {
            let child_max_state = child.get_max_state(tree);

            if tree.evaluate(child_max_state) > tree.evaluate(best_state) {
                best_state = child_max_state;
            }
        }

        best_state
    }

    fn get_tree_size(&self) -> u32 {
        let mut size = 1;

        for child in &self.children {
            size += child.get_tree_size();
        }

        size
    }
}

fn ucb(
    average_evaluation: f64,
    uct_exploration: f64,
    times_visited: u32,
    total_times_visited: u32,
) -> f64 {
    if times_visited == 0 {
        return std::f64::MAX;
    }

    let log_term = (total_times_visited as f64).ln();
    average_evaluation + uct_exploration * (log_term / (times_visited as f64)).sqrt()
}

fn get_children_from_mutations<'a, T>(
    state: T,
    mutations: &'a Vec<Box<Mutation<T>>>,
    previous_states: &mut HashSet<T>
) -> Vec<TreeSearchNode<'a, T>>
where
    T: State,
{
    mutations.iter().filter_map(|mutation| {
        let child_state = mutation(state);
        if previous_states.contains(&child_state) {
            None
        } else {
            previous_states.insert(child_state);
            Some(TreeSearchNode::new(child_state, mutations, previous_states))
        }
    }).collect()
}
