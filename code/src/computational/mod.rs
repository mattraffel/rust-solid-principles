//!

// these mods are pub so that others can use them
pub mod computations;
pub mod factory;

// these mods are not.  because part of SOLID is hiding implementation details
// from the consumers, these should not be pub
mod closest_values;
mod least_to_greatest;
mod maximum_value;