//!
#![allow(unused_imports)]
#![allow(dead_code)]

use super::data_source::*;
use super::in_memory_data_source::InMemoryDatasource;
use super::mysql::MySqlDatasource;

/**
    This implements a factory pattern for the trait DataSource.
    The factory knows how to get a implementation.  The factory uses inputted key values to determine
    which datasource to allocate
*/
pub struct DataSourceFactory {

}

impl DataSourceFactory {
    pub fn get(name : &str) -> DataSourceTypes {
        match name.as_ref() {
            "memory" => DataSourceTypes::InMemory(InMemoryDatasource {} ),
            "mysql" => DataSourceTypes::MySql( MySqlDatasource {} ),
            &_ => unimplemented!(),
        }
    }
}
