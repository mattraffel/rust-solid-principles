//!
#![allow(unused_imports)]
#![allow(dead_code)]
use data_sources::in_memory_data_source::InMemoryDatasource;
use data_sources::mysql::MySqlDatasource;

/**
    A trait defines a contract.

    Equivalent to the interface in java or C#.   Unlike java or C#, traits can include
    implementations, and not just abstract interfaces, but here we are only defining the
    "contract" and not providing an implementation.

    In this case the contract is a method called get_list which returns Vec<i32>

*/
pub trait Datasource {
    fn get_list(&self) -> Vec<i32>;
}

/**
    This is a bit of trickery to get around rusts need to know memory sizes at compile time.
    Abstract traits (above) have no size.  Enums do.  And since we want to return something that
    is more abstract, the enum lets us get around that rust behavior
*/
pub enum DataSourceTypes {
    InMemory(InMemoryDatasource),
    MySql(MySqlDatasource),
}

/**
   This serves as an abstraction of the actual implementations InMemoryDatasource and MySqlDatasource.
   Its a bit of duplication since each actual type (In_Memory_DataSource) implements Datasource as well.
   We need this since the factory (see DataSourceFactory) return enums and we want to keep the abstraction
   so the consumer doesn't break with changes in actual types.
*/
impl Datasource for DataSourceTypes {
    fn get_list(&self) -> Vec<i32> {
        match *self {
            DataSourceTypes::InMemory(ref in_memory) => in_memory.get_list(),
            DataSourceTypes::MySql(ref my_sql) => my_sql.get_list(),
        }
    }
}
