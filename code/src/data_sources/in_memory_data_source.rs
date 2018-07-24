//! InMemoryDatasource works on a hardcoded set of data
#![allow(unused_imports)]
#![allow(dead_code)]

use data_sources::data_source::Datasource;

/**
    InMemoryDatasource

   The get_list(...) method also shows singular purpose, aka the S in SOLID.  It does one thing
   and one thing only....populate a vector of i32
*/
pub struct InMemoryDatasource {

}

impl Datasource for InMemoryDatasource {

    fn get_list(&self) -> Vec<i32> {
        debug!("InMemoryDatasource.get_list()");                             // just to get the logging to appear

        let mut numbers: Vec<i32> = Vec::new();

        numbers.push(1);
        numbers.push(10);
        numbers.push(11);
        numbers.push(2);
        numbers.push(5);
        numbers.push(1);
        numbers.push(19);

        return numbers;
    }
}