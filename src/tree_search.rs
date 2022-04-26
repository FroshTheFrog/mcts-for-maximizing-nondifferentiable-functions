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
) -> T
where
    T: State,
{
    let mutations = T::get_possible_mutations();

    let mut base_node = TreeSearchNode::new(start_state, &mutations);

    for loop_number in 0..loops {
        
        if loop_number % LOOP_PRINT_INTERVAL == 0 {
            println!("Loop {}", loop_number);
        }

        base_node.run(uct_exploration, rollout, tree, rollout_epsilon);
    }

    base_node.get_max_state(tree)
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
    fn new(state: T, mutations: &'a Vec<Box<Mutation<T>>>) -> TreeSearchNode<'a, T> {
        TreeSearchNode {
            times_visited: 0,
            average_evaluation: 0.0,
            state: state,
            children: Vec::new(),
            mutations: mutations,
        }
    }

    fn run(
        &mut self,
        uct_exploration: f64,
        rollout: RollOut<T>,
        tree: &Box<dyn EvaluationTree<T>>,
        rollout_epsilon: f64,
    ) -> i32 {
        if self.children.len() == 0 {
            let expanded = self.expand();
            let value = expanded.simulate(rollout, tree, rollout_epsilon);
            self.update_average(value);
            return value;
        }

        let best_index = self.best_ucb_score_index(uct_exploration);

        let value = self.children[best_index].run(uct_exploration, rollout, tree, rollout_epsilon);
        self.update_average(value);
        value
    }

    fn expand(&mut self) -> &TreeSearchNode<T> {
        if self.times_visited == 0 {
            return self;
        }

        self.children = get_children_from_mutations(self.state, self.mutations);

        &self.children[0]
    }

    fn simulate(
        &self,
        rollout: RollOut<T>,
        tree: &Box<dyn EvaluationTree<T>>,
        rollout_epsilon: f64,
    ) -> i32 {
        rollout(
            self.state,
            &self.mutations,
            tree,
            ROLLOUT_DEPTH,
            rollout_epsilon,
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

    fn update_average(&mut self, value: i32) {
        self.times_visited += 1;
        self.average_evaluation +=
            (value as f64 - self.average_evaluation) / self.times_visited as f64;
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
) -> Vec<TreeSearchNode<'a, T>>
where
    T: State,
{
    mutations
        .iter()
        .map(|mutation| {
            let child_state = mutation(state);
            TreeSearchNode::new(child_state, mutations)
        })
        .collect()
}
