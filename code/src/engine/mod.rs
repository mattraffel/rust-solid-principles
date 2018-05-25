//!

pub mod logger;

use data_sources::{Datasource, InMemoryDatasource};

pub struct Engine {
}

impl Engine {
    pub fn run(&self) {
        println!("the engine is running");
    }
}