#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use computational::closest_values::ClosestValuesComputation;
use computational::least_to_greatest::LeastToGreatestComputation;
use computational::maximum_value::MaximumValueComputation;

pub trait Computations {
    fn get_name(&self) -> String;
    fn get_result(&self, requested_value: i32, inputs : Vec<i32>) -> Vec<i32>;
}

/**
*/
pub enum ComputationTypes {
    LeastToGreatest(LeastToGreatestComputation),
    ClosestValues(ClosestValuesComputation),
    MaximumValue(MaximumValueComputation),
}

impl Computations for ComputationTypes {
    fn get_name(&self) -> String {
        match *self {
            ComputationTypes::LeastToGreatest(ref least) => least.get_name(),
            ComputationTypes::ClosestValues(ref closest) => closest.get_name(),
            ComputationTypes::MaximumValue(ref max) => max.get_name(),
        }
    }

    fn get_result(&self, requested_value: i32, inputs : Vec<i32>) -> Vec<i32> {
        match *self {
            ComputationTypes::LeastToGreatest(ref least) => least.get_result(requested_value, inputs),
            ComputationTypes::ClosestValues(ref closest) => closest.get_result(requested_value, inputs),
            ComputationTypes::MaximumValue(ref max) => max.get_result(requested_value, inputs),
        }
    }
}

