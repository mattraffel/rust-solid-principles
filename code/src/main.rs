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

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

mod data_sources;
mod engine;

use engine::{Engine, logger};

fn main() {
    logger::init_log();

    let e: Engine = Engine { };
    e.run();
}
