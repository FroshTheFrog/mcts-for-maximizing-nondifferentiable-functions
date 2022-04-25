use std::fmt;

pub trait State : fmt::Display + Clone + Copy {
    fn random_state() -> Self where Self: Sized;
    fn get_possible_mutations() -> Vec<Box<Mutation<Self>>> where Self: Sized;
}

pub type Mutation<T> = dyn Fn(T) -> T;

pub trait EvaluationTree<T> where T : State {
    fn evaluate(&self, state : T) -> i32 where T : State;
}

type RollOut<T> = fn(T, Vec<Box<Mutation<T>>>, Box<dyn EvaluationTree<T>>) -> i32;