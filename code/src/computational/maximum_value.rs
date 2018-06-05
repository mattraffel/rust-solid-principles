//!

use computational::computations::Computations;

/**
    This computation adds up amounts starting with the largest available amount
    until the sum exceeds the requested value.
*/
pub struct MaximumValueComputation {

}

impl Computations for MaximumValueComputation {

    fn get_name(&self) -> String {
        return "Maximum Value".to_string();
    }

    fn get_result(&self, requested_value: i32, inputs : Vec<i32>) -> Vec<i32> {
        let mut inputted = inputs;
        let mut sum: i32 = 0;

        inputted.sort_by(|a, b| b.cmp(a));

        let mut results:Vec<i32> = vec![];

        for value in inputted {
            sum += value;
            results.push(value);

            if sum >= requested_value {
                break;
            }
        }

        return results;
    }
}