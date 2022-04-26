use crate::types::{self, Mutation};
use std::fmt;

use super::{constants::{STATE_VALUE_MIN, STATE_VALUE_MAX, STATE_SIZE}, utils::random_array};

#[derive(Hash)]
pub struct StateArray(pub [i32; STATE_SIZE]);

fn make_add_one_mutation(index : usize) -> Box<Mutation<StateArray>> {
    let function = move | state : StateArray | {

        let mut new_state = state;

        if new_state.0[index] < STATE_VALUE_MAX {
            new_state.0[index] += 1;
        }
        
        new_state
    };

    Box::new(function)
}

fn make_sub_one_mutation(index : usize) -> Box<Mutation<StateArray>> {
    let function = move | state : StateArray | {

        let mut new_state = state;

        if new_state.0[index] > STATE_VALUE_MIN {
            new_state.0[index] -= 1;
        }

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

            let add_one = make_add_one_mutation(index);
            let sub_one= make_sub_one_mutation(index);

            mutations.push(add_one);
            mutations.push(sub_one);
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
