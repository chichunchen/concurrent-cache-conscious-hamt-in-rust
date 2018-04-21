#![feature(test)]

extern crate test;
use test::Bencher;
use std::usize;
use std::collections::HashMap;
use rand::{Rng, thread_rng};

pub trait TrieData: Clone + Copy + Eq + PartialEq {}
impl<T> TrieData for T where T: Clone + Copy + Eq + PartialEq {}

const KEY_LEN: u32 = 18;

#[derive(Debug)]
pub struct ContiguousTrie<T: TrieData> {
    memory: Vec<Option<T>>,
}

// Since we have allocate a very large space for our hash in cctrie_contiguous
// why do we deal with conflict anymore?
// In this implementation, we
impl<T: TrieData> ContiguousTrie<T> {
    // Allocate memory at first
    pub fn new() -> Self {
        let capacity = 2usize.pow(KEY_LEN);
        let mut memory: Vec<Option<T>> = Vec::with_capacity(capacity);
        for i in 0..capacity {
            memory.push(None);
        }
        ContiguousTrie {
            memory,
        }
    }

    pub fn insert(&mut self, value: T, key: &usize) {
        self.memory[*key] = Some(value);
    }

    pub fn contain(&self, key: &usize) -> bool {
        match self.memory[*key] {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get(&self, key: &usize) -> Option<T> {
        self.memory[*key]
    }
}

#[test]
fn test_new_contiguous_trie() {
    let trie = ContiguousTrie::<usize>::new();
}

#[test]
fn test_insert_contiguous_trie() {
    let mut trie = ContiguousTrie::<usize>::new();
    for i in 0..10 {
        trie.insert(1, &i);
    }
}


#[test]
fn test_get_contiguous_trie() {
    let mut trie = ContiguousTrie::<&str>::new();
    trie.insert("abc", &0);
    trie.insert("cde", &1);

    let a = trie.get(&0);
    let ab = trie.get(&1);
    assert_eq!(a.unwrap(), "abc");
    assert_ne!(ab.unwrap(), "abc");
}

#[bench]
fn bench_large_size_trie(b: &mut Bencher) {
    let mut trie = ContiguousTrie::<usize>::new();
    let range = 2usize.pow(KEY_LEN);
    for i in 0..range {
        trie.insert(i, &i);
    }
    b.iter(|| {
        for i in 0..range {
            let g = trie.get(&i);
        }
    });
}


#[bench]
fn bench_large_size_reverse_trie(b: &mut Bencher) {
    let mut trie = ContiguousTrie::<usize>::new();
    let range = 2usize.pow(KEY_LEN);
    for i in 0..range {
        trie.insert(i, &i);
    }
    b.iter(|| {
        for i in 1..range {
            let x = range - i - 1;
            let g = trie.get(&x);
        }
    });
}

#[bench]
fn bench_large_size_hashmap(b: &mut Bencher) {
    let mut hash = HashMap::new();
    let range = 2usize.pow(KEY_LEN);
    for i in 0..range {
        hash.insert(i as usize, i as usize);
    }
    b.iter(|| {
        for i in 0..range {
            let g = hash.get(&i);
        }
    });
}