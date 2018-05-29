//!
#![allow(unused_imports)]
#![allow(dead_code)]

use super::Data_Source::*;
use super::InMemory_Data_Source::InMemoryDatasource;
use super::MySql::MySqlDatasource;

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
