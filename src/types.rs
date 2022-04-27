use std::{collections::HashSet, fmt};

pub trait State: fmt::Display + Clone + Copy + Eq + std::hash::Hash {
    fn random_state() -> Self
    where
        Self: Sized;
    fn get_possible_mutations() -> Vec<Box<Mutation<Self>>>
    where
        Self: Sized;
}

pub type Mutation<T> = dyn Fn(T) -> T;

pub trait Evaluator<T>
where
    T: State,
{
    fn evaluate(&self, state: T) -> i32;
}

pub type RollOut<T> =
    fn(T, &Vec<Box<Mutation<T>>>, &dyn Evaluator<T>, usize, f64, &HashSet<T>) -> i32;
