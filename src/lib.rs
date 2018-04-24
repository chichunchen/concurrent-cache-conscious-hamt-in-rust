#![feature(test)]
#![allow(warnings)]

extern crate core;
extern crate rand;
extern crate test;
extern crate chashmap;
extern crate rayon;

mod cctrie;
mod contiguous_cctrie;
mod bench;

pub use cctrie::{Trie, TrieData, IndexStatus};
pub use contiguous_cctrie::{ContiguousTrie};