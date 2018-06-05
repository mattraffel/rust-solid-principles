//! InMemoryDatasource works on a hardcoded set of data
#![allow(unused_imports)]
#![allow(dead_code)]

use data_sources::data_source::Datasource;

/**
    InMemoryDatasource
*/
pub struct InMemoryDatasource {

}

impl Datasource for InMemoryDatasource {

    fn get_list(&self) -> Vec<i32> {
        debug!("");                             // just to get the logging to appear

        let mut numbers: Vec<i32> = vec![];

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