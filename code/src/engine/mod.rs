//!
#![allow(unused_imports)]
#![allow(dead_code)]

pub mod configuration;
pub mod logger;

use data_sources::factory::DataSourceFactory;
use data_sources::data_source::{DataSourceTypes, Datasource};

use computational::computations::Computations;
use computational::factory::ComputationsFactory;

use engine::configuration::Configuration;


/**
   The purpose of this engine is to acquire a range of values (that represent UTXOs) and then apply
   a computation to select a subset of the values whose sum will exceed the value requested.

   The run(...) method shows two examples of inversion of control, aka the I in SOLID
   The engine needs a data source.  The engine doesn't allocate the data source, it asks
   for the data source to use.  Same for computations.

*/
pub struct Engine {
}

impl Engine {
    pub fn run(&self, config: Configuration) {
        println!("the engine is running");

        let db = DataSourceFactory::get(&config.get_data_source_name());
        let data = db.get_list();

        println!("the following \"UTXO\" are available {:?}", data);

        let compute = ComputationsFactory::get(&config.get_computation_name());

        println!("using \"{}\" computations to get results", compute.get_name());

        let results = compute.get_result(config.get_payment_amount(), data);

        println!("The \"UTXOs\" to be used to pay fee of {} is {:?}", pay_amount, results);
    }
}
