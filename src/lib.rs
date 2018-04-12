#![feature(ptr_internals)]
#![feature(allocator_api)]
#![feature(unique)]
#![feature(slice_get_slice)]

extern crate core;

mod cctrie;
mod cache_vec;
mod allocator;

pub use cctrie::{Trie, TrieData, IndexStatus};
pub use cache_vec::CCVec;
pub use allocator::{Allocator};