#![feature(ptr_internals)]
#![feature(allocator_api)]
#![feature(unique)]
#![feature(slice_get_slice)]

extern crate core;

mod cctrie;
mod cache_vec;
mod allocator;
mod cctrie_contiguous;

pub use cctrie::{Trie, TrieData, IndexStatus};
pub use cache_vec::CCVec;
pub use allocator::{Allocator};
pub use cctrie_contiguous::ContiguousTrie;