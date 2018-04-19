#![feature(ptr_internals)]
#![feature(allocator_api)]
#![feature(unique)]
#![feature(slice_get_slice)]
#![feature(test)]
#![allow(warnings)]

extern crate core;
extern crate rand;
extern crate test;
extern crate chashmap;
extern crate rayon;

mod cctrie;
mod cache_vec;
mod allocator;
mod cctrie_contiguous;
mod bench;
mod optimal_contiguous_cctrie;

pub use cctrie::{Trie, TrieData, IndexStatus};
pub use cache_vec::CCVec;
pub use allocator::{Allocator};
pub use cctrie_contiguous::SubTrie;
pub use optimal_contiguous_cctrie::ContiguousTrie;