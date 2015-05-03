#![feature(core)]
#![feature(plugin)]
#![plugin(peg_syntax_ext)]
#![feature(collections)]
#![feature(str_char)]
#![feature(collections_drain)]

extern crate itertools;
extern crate simple_parallel;
extern crate rust_htslib as htslib;
extern crate tempdir;
extern crate bio;

pub type Prob = f64;
pub type LogProb = f64;
pub const EPSILON: Prob = 0.000001;

pub mod call;
pub mod utils;
pub mod query;
pub mod cli;
