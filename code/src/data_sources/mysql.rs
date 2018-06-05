//! MySqlDatasource will use data stored in mysql database

#![allow(unused_imports)]
#![allow(dead_code)]

use data_sources::data_source::Datasource;

/**
    MySqlData source
*/
pub struct MySqlDatasource {

}

impl Datasource for MySqlDatasource {

    fn get_list(&self) -> Vec<i32> {
        debug!("");                             // just to get the logging to appear
        unimplemented!()
    }
}


