//! InMemoryDatasource works on a hardcoded set of data
#![allow(unused_imports)]
#![allow(dead_code)]
use super::Data_Source::Datasource;

/**
    InMemoryDatasource
*/
pub struct InMemoryDatasource {

}

impl Datasource for InMemoryDatasource {

    fn get_list(&self) -> Vec<i32> {
        debug!("");                             // just to get the logging to appear

        let mut numbers: Vec<i32> = vec![];

        numbers.insert(1, 1);
        numbers.insert(1, 5);
        numbers.insert(1, 10);

        return numbers;
    }
}