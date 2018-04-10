#![feature(ptr_internals)]
#![feature(allocator_api)]
#![feature(unique)]

mod cctrie;
mod cache_vec;

pub use cctrie::{Trie, IndexStatus};
pub use cache_vec::CCVec;