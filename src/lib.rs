#![feature(test)]
#![allow(warnings)]

extern crate core;
extern crate rand;
extern crate test;
extern crate chashmap;
extern crate rayon;

mod cctrie;
mod contiguous_cctrie;
mod contiguous_cctrie_one_mutex;
mod contiguous_cctrie_one_rwlock;
mod bench;

pub use cctrie::{Trie, TrieData, IndexStatus};
pub use contiguous_cctrie_one_mutex::MutexContiguousTrie;
pub use contiguous_cctrie_one_rwlock::RwContiguousTrie;
pub use contiguous_cctrie::ContiguousTrie;