//!
#![allow(unused_imports)]
#![allow(dead_code)]

pub mod closest_values;
pub mod computations;
pub mod configuration;
pub mod factory;
pub mod least_to_greatest;
pub mod logger;
pub mod maximum_value;

use data_sources::factory::DataSourceFactory;
use data_sources::data_source::{DataSourceTypes, Datasource};

use self::computations::Computations;
use self::configuration::Configuration;
use self::factory::ComputationsFactory;

/**
   The purpose of this engine is to acquire a range of values (that represent UTXOs) and then apply
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

        let config: Configuration = Configuration::new();
        let db = DataSourceFactory::get(&config.data_source);
        let data = db.get_list();

        println!("the following \"UTXO\" are available {:?}", data);

        let compute = ComputationsFactory::get(&config.computation);

        println!("using \"{}\" computations to get results", compute.get_name());

        let pay_amount: i32 = config.pay_this_amount;
        let results = compute.get_result(pay_amount, data);

        println!("The \"UTXOs\" to be used to pay fee of {} is {:?}", pay_amount, results);
    }
}
