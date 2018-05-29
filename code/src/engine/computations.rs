#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use super::closest_values::ClosestValuesComputation;
use super::least_to_greatest::LeastToGreatestComputation;
use super::maximum_value::MaximumValueComputation;

pub trait Computations {
    fn get_list(&self, requested_value: i32, inputs : Vec<i32>) -> Vec<i32>;
}

/**
*/
pub enum ComputationTypes {
    LeastToGreatest(LeastToGreatestComputation),
    ClosestValues(ClosestValuesComputation),
    MaximumValue(MaximumValueComputation),
}

impl Computations for ComputationTypes {
    fn get_list(&self, requested_value: i32, inputs : Vec<i32>) -> Vec<i32> {
        match *self {
            ComputationTypes::LeastToGreatest(ref least) => least.get_list(requested_value, inputs),
            ComputationTypes::ClosestValues(ref closest) => closest.get_list(requested_value, inputs),
            ComputationTypes::MaximumValue(ref max) => max.get_list(requested_value, inputs),
        }
    }
}

