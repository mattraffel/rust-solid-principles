//!
//! The goal of this project is to show all of the principles of SOLID being used, particularly
//! focusing on LID part of SOLID.
//!
//! The demonstration contains an engine that gets data and runs several computations on the data
//!
//! The SOLID principles will be demonstrated by changing the source of the data and using multiple
//! computations WITHOUT any changes to the engine.

#![allow(unused_imports)]
#![allow(dead_code)]

extern crate ansi_term;
extern crate env_logger;
extern crate libc;
extern crate preferences;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

mod data_sources;
mod engine;

use engine::{Engine, logger};


use preferences::{AppInfo, PreferencesMap, Preferences};

fn save_config_file() {
    let APPINFO = AppInfo{name: "preferences", author: "999999"};
    let config_location = "99999999";

    let mut data: PreferencesMap<String> = PreferencesMap::new();
    data.insert("data_source".to_string(), "memory".to_string());
    data.insert("computation".to_string(), "max".to_string());
    data.insert("pay_this_amount".to_string(), "13".to_string());

    let save_result = data.save(&APPINFO, config_location);
    println!("result of save is: {:?}", save_result);
    assert!(save_result.is_ok());
}

fn main() {
    logger::init_log();

    save_config_file();

    let e: Engine = Engine { };
    e.run();
}
