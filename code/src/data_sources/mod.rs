//!
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_snake_case)]

// these mods are pub so that others can use them
pub mod factory;
pub mod data_source;

// these mods are not.  because part of SOLID is hiding implementation details
// from the consumers, these should not be pub
mod in_memory_data_source;
mod mysql;


