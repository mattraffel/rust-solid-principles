//!
#![allow(unused_imports)]
#![allow(dead_code)]

use super::computations::{Computations, ComputationTypes};
use super::closest_values::ClosestValuesComputation;
use super::least_to_greatest::LeastToGreatestComputation;
use super::maximum_value::MaximumValueComputation;


/**
    This implements a factory pattern for the trait DataSource.
    The factory knows how to get a implementation.  The factory uses inputted key values to determine
    which datasource to allocate
*/
pub struct ComputationsFactory {

}

impl ComputationsFactory {
    pub fn get(name : &str) -> ComputationTypes {
        match name.as_ref() {
            "least" => ComputationTypes::LeastToGreatest(LeastToGreatestComputation {} ),
            "closest" => ComputationTypes::ClosestValues(ClosestValuesComputation {} ),
            "max" => ComputationTypes::MaximumValue(MaximumValueComputation {} ),
            &_ => {
                println!("Error processing get for {}", name);
                unimplemented!()
            },
        }
    }
}


#[cfg(test)]
mod factory_tests {

    use std::panic;
    use super::ComputationsFactory;

    #[test]
    fn errors_with_invalid_engine_id() {

        let error = panic::catch_unwind(|| {
            ComputationsFactory::get("bob");
        });

        // test fails if no panic was caught
        if error.is_ok() {
            assert!(false);
        }
    }
}