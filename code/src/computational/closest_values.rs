//!

use computational::computations::Computations;

/**
    Looks for the value closest to the requested value, puts that in the result list
    and then looks for the next closest...etc etc...
*/
pub struct ClosestValuesComputation {

}

impl Computations for ClosestValuesComputation {
    fn get_name(&self) -> String {
        return "Closest Values".to_string();
    }

    fn get_result(&self, requested_value: i32, inputs : Vec<i32>) -> Vec<i32> {

        let min = 0;
        let max = inputs.len() as i32;
        let middle = max / 2;

        let mut results:Vec<i32> = vec![];
        let mut inputted = inputs;
        inputted.sort();
        debug!("sorted utxo {:?}", inputted);

        let mut sum : i32 = 0;
        let mut lower_bounds = middle;
        let mut upper_bounds = middle + 1;


        while sum < requested_value {
            if lower_bounds < 0 && upper_bounds > max {
                break;
            }

            if lower_bounds >= 0 && sum < requested_value {
                sum += inputted[lower_bounds as usize];
                results.push(inputted[lower_bounds as usize]);
                lower_bounds -= 1;
            }

            if upper_bounds <= max && sum < requested_value {
                sum += inputted[upper_bounds as usize];
                results.push(inputted[upper_bounds as usize]);
                upper_bounds += 1;
            }
        }

        return results;
    }
}