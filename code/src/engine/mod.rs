//!
#![allow(unused_imports)]
#![allow(dead_code)]

pub mod logger;
pub mod closest_values;
pub mod computations;
pub mod least_to_greatest;
pub mod maximum_value;

use data_sources::factory::DataSourceFactory;
use data_sources::data_source::{DataSourceTypes, Datasource};

/**
   The purpose of this engine is to acquire a range of values and then apply
   a computation to select a subset of the values whose sum will exceed the value requested.

   This description is inherently vague so that different data sources and
   different data computations can be applied via SOLID principles without breaking
   any thing
*/
pub struct Engine {
}

impl Engine {
    pub fn run(&self) {
        println!("the engine is running");

        let db = DataSourceFactory::get("memory");
        let data = db.get_list();

        println!("data received is {:?}", data);
    }
}
