//!


/**
    trait defines a contract.

    In this case the contract is a method called get_list which returns Vec<i32>

*/
pub trait Datasource {
    fn new() -> Self;
    fn get_list(&self) -> Vec<i32>;
}

/**
    InMemoryDatasource
*/
pub struct InMemoryDatasource {

}

impl Datasource for InMemoryDatasource {
    fn new() -> Self {
        return InMemoryDatasource{};
    }

    fn get_list(&self) -> Vec<i32> {
        debug!("");                             // just to get the logging to appear

        let mut numbers: Vec<i32> = vec![];

        numbers.insert(1, 1);
        numbers.insert(1, 5);
        numbers.insert(1, 10);

        return numbers;
    }
}

/**
    MySqlData source
*/
pub struct MySqlDatasource {

}

impl Datasource for MySqlDatasource {
    fn new() -> Self {
        return MySqlDatasource{};
    }

    fn get_list(&self) -> Vec<i32> {
        debug!("");                             // just to get the logging to appear
        unimplemented!()
    }
}

