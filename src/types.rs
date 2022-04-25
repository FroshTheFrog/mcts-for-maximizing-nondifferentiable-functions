use std::fmt;

pub trait State : fmt::Display + Clone + Copy {
    fn random_state() -> Self where Self: Sized;
    fn get_possible_mutations() -> Vec<Box<Mutation<Self>>> where Self: Sized;
}

pub type Mutation<T> = dyn Fn(T) -> T;

pub trait EvaluationTree<T> where T : State {
    fn evaluate(&self, state : T) -> usize;
}