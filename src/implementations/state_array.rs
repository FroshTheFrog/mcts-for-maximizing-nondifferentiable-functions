use crate::types::{self, Mutation};
use std::cmp;
use std::fmt;

use super::{
    constants::{STATE_SIZE, STATE_VALUE_MAX, STATE_VALUE_MIN},
    utils::random_array,
};

#[derive(Hash)]
pub struct StateArray(pub [i32; STATE_SIZE]);

fn make_mutation(index: usize, operation: fn(i32) -> i32) -> Box<Mutation<StateArray>> {
    let function = move |state: StateArray| {
        let mut new_state = state;

        let new_value = operation(new_state.0[index]);

        new_state.0[index] = cmp::max(STATE_VALUE_MIN, cmp::min(STATE_VALUE_MAX, new_value));

        new_state
    };

    Box::new(function)
}

impl types::State for StateArray {
    fn random_state() -> StateArray {
        StateArray(random_array(STATE_VALUE_MIN, STATE_VALUE_MAX))
    }

    fn get_possible_mutations() -> Vec<Box<Mutation<Self>>> {
        let mut mutations = Vec::new();

        for index in 0..STATE_SIZE {
            let add_one = make_mutation(index, |x| x + 1);
            let sub_one = make_mutation(index, |x| x - 1);

            let multiply_two = make_mutation(index, |x| x * 2);
            let divide_two = make_mutation(index, |x| x / 2);

            let multiply_three = make_mutation(index, |x| x * 3);
            let divide_three = make_mutation(index, |x| x / 3);

            let multiply_five = make_mutation(index, |x| x * 5);
            let divide_five = make_mutation(index, |x| x / 5);

            let multiply_seven = make_mutation(index, |x| x * 7);
            let divide_seven = make_mutation(index, |x| x / 7);

            let multiply_eleven = make_mutation(index, |x| x * 11);
            let divide_eleven = make_mutation(index, |x| x / 11);

            mutations.push(add_one);
            mutations.push(sub_one);

            mutations.push(multiply_two);
            mutations.push(divide_two);

            mutations.push(multiply_three);
            mutations.push(divide_three);

            mutations.push(multiply_five);
            mutations.push(divide_five);

            mutations.push(multiply_seven);
            mutations.push(divide_seven);

            mutations.push(multiply_eleven);
            mutations.push(divide_eleven);
        }

        mutations
    }
}

impl fmt::Display for StateArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Copy for StateArray {}

impl Clone for StateArray {
    fn clone(&self) -> StateArray {
        StateArray(self.0)
    }
}

impl PartialEq for StateArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for StateArray {}
