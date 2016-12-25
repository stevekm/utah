#![feature(test)]
#![feature(custom_derive)]
#![feature(stmt_expr_attributes)]
#![feature(conservative_impl_trait)]
#![feature(specialization)]
#![recursion_limit = "1024"]

#[macro_use]


extern crate ndarray;
extern crate ndarray_rand;
extern crate rand;
extern crate test;
extern crate num;
extern crate chrono;
#[macro_use]
extern crate error_chain;
extern crate itertools;

/// # Utah
///
/// ## Table of contents
///
/// + [DataFrame](#dataframe)
/// + [Transformation](#transformation)
/// + [Aggregation](#aggregation)
/// + [Imputation](#imputation)

///
/// ## DataFrame
/// Utah is a dataframe crate for Rust.
/// ### What's a dataframe?
/// ### Why use this crate?

/// ## Transformation
///

/// ## Aggregation
///

/// ## Imputation

pub mod adapters;
pub mod dataframe;
pub mod tests;
#[macro_use]
pub mod util;
pub mod implement;
pub mod mixedtypes;
