#![feature(allocator_api)]

extern crate cchamt;

use cchamt::Allocator;
use std::heap::{Alloc, Layout, Heap};
use cchamt::{Trie, TrieData};

#[test]
fn test_allocator_new() {
    let a: Allocator<i32> = Allocator::with_capacity(4096);
}

#[test]
fn test_allocator_from_box() {
    let g: Box<[i32; 3]> = Box::new([1, 2, 3]);
    let a: Allocator<i32> = Allocator::from_box(g);
}

#[test]
fn test_allocator_into_box() {
    let a: Allocator<i32> = Allocator::with_capacity(4096);
    let b: Box<[i32]> = unsafe { a.into_box() };
}

#[test]
fn test_allocator_push() {
    let mut a: Allocator<i32> = Allocator::with_capacity(4096);
    a.update(0, 200);
}

#[test]
fn test_index() {
    let mut a: Allocator<i32> = Allocator::with_capacity(4096);
    a.update(0, 100);
    a.update(1, 1000);
    assert_eq!(a[0], 100);
    assert_eq!(a[1], 1000);
}

#[test]
fn test_practical() {
    let mut a: Allocator<Option<Box<Trie<()>>>> = Allocator::with_capacity(4096);
    for i in 0..16 {
        a.update(i, None);
    }
    assert!(a[5].is_none());
    a[4] = Some(Box::new(Trie::new()));

    let q = a[4].as_mut();
    //.map(|ref mut a| a.insert(value, &key[KEY_GROUP..])).unwrap_or(0)
}