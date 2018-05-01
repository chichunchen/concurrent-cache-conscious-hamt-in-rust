#![feature(test)]
#![allow(warnings)]
#![feature(ptr_internals)]
#![feature(allocator_api)]
#![feature(unique)]
#![feature(slice_get_slice)]
#![feature(box_syntax)]
#![feature(core_intrinsics)]
#![feature(libc)]

extern crate core;
extern crate rand;
extern crate test;
extern crate chashmap;
extern crate rayon;

mod hamt;
mod cchamt;
//mod bench;
mod allocator;
mod lockfree_cchamt;
mod mutex_cchamt;
mod rwlock_cchamt;

pub use hamt::{Trie, TrieData, IndexStatus};
pub use cchamt::ContiguousTrie;
pub use allocator::Allocator;
pub use lockfree_cchamt::LockfreeTrie;
pub use mutex_cchamt::MutexContiguousTrie;
pub use rwlock_cchamt::RwContiguousTrie;
