use crate::types;
use std::fmt;

use super::constants::STATE_VALUE_MIN;
use super::constants::STATE_VALUE_MAX;
use super::constants::STATE_SIZE;
use super::utils::random_array;

pub struct StateArray(pub [usize; STATE_SIZE]);

impl types::State for StateArray {

    fn random_state() -> StateArray {
        StateArray(random_array(STATE_VALUE_MIN, STATE_VALUE_MAX))
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