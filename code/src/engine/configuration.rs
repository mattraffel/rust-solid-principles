//!


pub struct Configuration {
    pub data_source : String,
    pub computation : String,
    pub pay_this_amount : i32
}

impl Configuration {
    pub fn new() -> Self {
        return Configuration {
            data_source : "memory".to_string(),
            computation : "max".to_string(),
            pay_this_amount : 13
        };
    }

    pub fn from_file() -> Self {
        unimplemented!()
    }
}