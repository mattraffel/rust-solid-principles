//!
#![allow(unused_imports)]
#![allow(dead_code)]

pub mod logger;

use data_sources::Factory::DataSourceFactory;
use data_sources::Data_Source::*;

pub struct Engine {
}

impl Engine {
    pub fn run(&self) {
        println!("the engine is running");

        let dataSource = DataSourceFactory::get("memory");
        let data = dataSource.get_list();

        println!("data received is {:?}", data);
    }
}