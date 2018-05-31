//!

use super::computations::Computations;

pub struct LeastToGreatestComputation {

}

impl Computations for LeastToGreatestComputation {

    fn get_name(&self) -> String {
        return "Least To Greatest".to_string();
    }

    fn get_result(&self, requested_value: i32, inputs : Vec<i32>) -> Vec<i32> {
        let mut inputted = inputs;
        let mut sum: i32 = 0;

        inputted.sort();

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
