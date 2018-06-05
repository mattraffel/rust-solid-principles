//!
#![allow(unused_imports)]
#![allow(dead_code)]

use data_sources::data_source::{DataSourceTypes};
use data_sources::in_memory_data_source::InMemoryDatasource;
use data_sources::mysql::MySqlDatasource;

/**
    This implements a factory pattern for the trait DataSource.
    The factory knows how to get a implementation.  The factory uses inputted key values to determine
    which datasource to allocate

    This factory solves for the I in SOLID.  Inversion of control.  Any code that needs a data source
    requests it from the factory and factory supplies the correct instance.
*/
pub struct DataSourceFactory {

}

impl DataSourceFactory {
    pub fn get(name : &str) -> DataSourceTypes {
        match name.as_ref() {
            "memory" => DataSourceTypes::InMemory(InMemoryDatasource {} ),
            "mysql" => DataSourceTypes::MySql( MySqlDatasource {} ),
            &_ =>{
                eprintln!("Error processing get for {}", name);
                panic!("Error in ComputationsFactory processing get request")}
            ,
        }
    }
}


#[cfg(test)]
mod factory_tests {

    use std::panic;
    use super::DataSourceFactory;

    #[test]
    fn errors_with_invalid_data_source() {

        let error = panic::catch_unwind(|| {
            DataSourceFactory::get("bob");
        });

        // test fails if no panic was caught
        if error.is_ok() {
            assert!(false);
        }
    }
}