//!

use super::computations::Computations;


pub struct ClosestValuesComputation {

}

impl Computations for ClosestValuesComputation {
    fn get_name(&self) -> String {
        return "Closest Values".to_string();
    }

    fn get_result(&self, requested_value: i32, inputs : Vec<i32>) -> Vec<i32> {
        unimplemented!()
    }
}