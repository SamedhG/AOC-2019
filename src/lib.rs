//! This crate contains solutions to the `2019 Advent Of Code` problems
//! I am mainly doing this for learning rust and getting used to working
//! with rust code which looks too different from any C-style or
//! Functional Code that I normally write.
//!
//! For now the code will be structured into modules named `dayx`
//! each containing a p1 and p2 public function
//! In general each function will look for input data in `resources/dayx`
//! I am hoping to standardize reading data into a utils.rs ile but that will
//! depend on the structure of the diffenret problems and how differen
//! they end up being. In this file ill just be exporting these public
//! modules to be called externally for anyone looking to use the code

pub mod day1;
pub mod day2;
pub mod day3;
