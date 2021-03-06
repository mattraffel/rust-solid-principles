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

mod computational;
mod data_sources;
mod engine;

use engine::configuration::Configuration;
use engine::{Engine, logger};

fn main() {
    logger::init_log();

    // Configuration::save_config_file();

    let e: Engine = Engine { };

    let config: Configuration = Configuration::from_file();

    // here is a form of dependency injection.  Engine requires certain data and code
    // to run.  The engine doesn't allocate it, it gets it.  Changing the inputs changes
    // the system without breaking it.  Aka the L of SOLID
    e.run(config);
}
