//! # grid-machine
//!
//! Library for creating cellular automata and complex automata in rust
//!
//! ## What is a Cellular Automaton?
//!
//! A cellular automaton is a grid of cells where each cell changes it's state depending on those
//! of its neighbors.
//!
//! ## What is a Complex Automaton?
//!
//! A complex automaton combines a cellular automaton with an agent-based simulation that operates
//! on the cellular automaton grid.
//!
//! The goal of this library is to offer a comprehensive tool chain for simulations based on both
//! of these models.

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

pub mod abm;
pub mod ca;

mod tests;

// TODO: Add RNG
// TODO: Add cell initialisation (method)
//       - maybe use constructor for use with a `LocatedCell` trait
// TODO: Add simple terminal-based visualisation
