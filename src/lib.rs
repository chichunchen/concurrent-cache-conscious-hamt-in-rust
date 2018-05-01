#![feature(test)]
#![allow(warnings)]
#![feature(ptr_internals)]
#![feature(allocator_api)]
#![feature(unique)]
#![feature(slice_get_slice)]
#![feature(box_syntax)]
#![feature(core_intrinsics)]
#![feature(libc)]
#![feature(integer_atomics)]

extern crate core;
extern crate rand;
extern crate test;
extern crate chashmap;
extern crate rayon;

mod cctrie;
mod contiguous_cctrie;
mod bench;
mod allocator;
mod lockfree_cctrie;

pub use cctrie::{Trie, TrieData, IndexStatus};
pub use contiguous_cctrie::ContiguousTrie;
pub use allocator::Allocator;
pub use lockfree_cctrie::LockfreeTrie;
