//!
#![allow(unused_imports)]
#![allow(dead_code)]

use super::Data_Source::*;
use super::InMemory_Data_Source::InMemoryDatasource;
use super::MySql::MySqlDatasource;

/**
    This implements a factory pattern.  The factory knows how to get a implementation
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
