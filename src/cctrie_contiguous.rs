#![feature(test)]

extern crate test;
extern crate rand;

use test::Bencher;
use std::usize;
use std::collections::HashMap;
use rand::{Rng, thread_rng};

pub trait TrieData: Clone + Copy + Eq + PartialEq {}

impl<T> TrieData for T where T: Clone + Copy + Eq + PartialEq {}

const KEY_LEN: usize = 20;
const KEY_GROUP: usize = 4;

#[derive(Debug)]
pub struct ContiguousTrie<T: TrieData> {
    memory: Vec<Option<SubTrie<T>>>,
}


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SubTrie<T: TrieData> {
    pub data: Option<T>,
    depth: usize,
    children_offset: Option<usize>,    // the start position in allocator that place the array in hash trie
}

fn get_depth(index: usize) -> usize {
    let mut depth = 0;
    let mut multitude = KEY_LEN;
    let mut compare = KEY_LEN;

    while index >= compare {
        depth += 1;
        multitude *= KEY_LEN;
        compare += multitude;
    }
    depth
}

// return the index in the first <= 4 bits
// for instances: 0000 0000 -> 0
fn compute_index(key: &[u8]) -> usize {
    let mut id = 0;
    let length = if key.len() > KEY_GROUP { KEY_GROUP } else { key.len() };
    for i in 0..length {
        let temp = key[i] as usize - '0' as usize;
        id += temp << (length - i - 1);
    }
    return id as usize;
}

// Since we have allocate a very large space for our hash in cctrie_contiguous
// why do we deal with conflict anymore?
// In this implementation, we
impl<T: TrieData> ContiguousTrie<T> {
    pub fn new() -> Self {
        // init with three level of nodes
        let mut nodes_length = 0;   // = KEY_LEN + KEY_LEN * KEY_LEN + KEY_LEN * KEY_LEN * KEY_LEN;
        // 16,4 -> 0,1,2
        let mut multitude = KEY_LEN;
        for i in 0..(KEY_LEN/KEY_GROUP - 1) {
            nodes_length += multitude;
            multitude *= KEY_LEN;
        }
        let mut memory: Vec<Option<SubTrie<T>>> = Vec::with_capacity(nodes_length);

        for i in 0..nodes_length {
            let subtrie: SubTrie<T> = SubTrie {
                data: None,
                depth: get_depth(i),
                children_offset: Some((i + 1) * KEY_LEN),
            };
            memory.push(Some(subtrie));
        }

        ContiguousTrie {
            memory,
        }
    }

    // key should be 1-1 mapping to self memory array
    fn key2index(&self, key: &[u8]) -> usize {
        let mut current_index = compute_index(key);
        let mut key_start = 0;
        while self.memory.len() > current_index && self.memory[current_index].is_some() {
            match &self.memory[current_index] {
                Some(a) => {
                    match a.children_offset {
                        Some(b) => {
                            key_start += KEY_GROUP;
                            current_index = b + compute_index(&key[key_start..]);
//                            println!("comp_index {} ci {} {}", compute_index(&key[key_start..]), current_index, self.memory.len());
                        }
                        None => break,
                    }
                }
                None => break,
            }
        }
        current_index
    }

    pub fn insert(&mut self, value: T, key: &[u8]) {
        let current_index = self.key2index(key);
        if current_index >= self.memory.len() {
//            println!("debug {} {}", current_index, self.memory.len());
            let push_amount = current_index - self.memory.len() + 1;
            for _ in 0..push_amount {
                self.memory.push(None);
            }
        }
        if self.memory[current_index].is_some() {
            assert!(false);
        }
        self.memory[current_index] = Some(SubTrie {
            data: Some(value),
            depth: get_depth(current_index),
            children_offset: None,
        });
    }

//    pub fn contain(&self, key: &usize) -> bool {
//        match self.memory[*key] {
//            Some(_) => true,
//            None => false,
//        }
//    }

    pub fn get(&self, key: &[u8]) -> Option<T> {
        let current_index = self.key2index(key);
        if self.memory.len() <= current_index {
            return None;
        }
        match &self.memory[current_index] {
            Some(a) => {
                a.data
            },
            None => None,
        }
    }
}

#[test]
fn test_new_contiguous_trie() {
    let trie = ContiguousTrie::<usize>::new();
}

#[test]
fn test_insert_contiguous_trie() {
    let mut trie = ContiguousTrie::<usize>::new();
    trie.insert(1, &"0000000000000000".to_owned().into_bytes());
    trie.insert(10, &"0000000000000001".to_owned().into_bytes());
    trie.insert(100, &"0000000000000010".to_owned().into_bytes());
    trie.insert(100, &"0000000000000011".to_owned().into_bytes());
    trie.insert(1000, &"0000000000000100".to_owned().into_bytes());
    trie.insert(344, &"0000100000000011".to_owned().into_bytes());
    trie.insert(33, &"0000100000100011".to_owned().into_bytes());

    assert_eq!(trie.get(&"0000000000000000".to_owned().into_bytes()).unwrap(), 1);
    assert_eq!(trie.get(&"0000000000000001".to_owned().into_bytes()).unwrap(), 10);
    assert_eq!(trie.get(&"0000000000000010".to_owned().into_bytes()).unwrap(), 100);
    assert_eq!(trie.get(&"0000000000000011".to_owned().into_bytes()).unwrap(), 100);
    assert_eq!(trie.get(&"0000000000000100".to_owned().into_bytes()).unwrap(), 1000);
    assert_eq!(trie.get(&"0000100000000011".to_owned().into_bytes()).unwrap(), 344);
    assert_eq!(trie.get(&"0000100000100011".to_owned().into_bytes()).unwrap(), 33);
}


#[test]
fn test_get_contiguous_trie() {
    let mut trie = ContiguousTrie::<&str>::new();
    trie.insert("abc", &"0000000000000000".to_owned().into_bytes());
    trie.insert("cde", &"0000000000000001".to_owned().into_bytes());

    let a = trie.get(&"0000000000000000".to_owned().into_bytes());
    let ab = trie.get(&"0000000000000001".to_owned().into_bytes());
    assert_eq!(a.unwrap(), "abc");
    assert_ne!(ab.unwrap(), "abc");
}

#[test]
fn test_large_consecutive_insert() {
    let mut trie = ContiguousTrie::<usize>::new();

    for i in 0..65536 {
        let str = format!("{:#018b}", i);
        let arr = str.to_owned().into_bytes();
        trie.insert(i, &arr[2..]);
    }

    for i in 0..65536 {
        let str = format!("{:#018b}", i);
        let arr = str.to_owned().into_bytes();
        assert_eq!(trie.get(&arr[2..]).unwrap(), i);
    }
}

#[test]
fn test_very_large_consecutive_insert() {
    let mut trie = ContiguousTrie::<usize>::new();

    for i in 0..1000000 {
        let str = format!("{:#022b}", i);
        let arr = str.to_owned().into_bytes();
        trie.insert(i, &arr[2..]);
    }

    for i in 0..1000000 {
        let str = format!("{:#022b}", i);
        let arr = str.to_owned().into_bytes();
        assert_eq!(trie.get(&arr[2..]).unwrap(), i);
    }
}

//#[bench]
//fn bench_large_size_trie(b: &mut Bencher) {
//    let mut trie = ContiguousTrie::<usize>::new();
//    let range = 2usize.pow(KEY_LEN);
//    for i in 0..range {
//        trie.insert(i, &i);
//    }
//    b.iter(|| {
//        for i in 0..range {
//            let g = trie.get(&i);
//        }
//    });
//}
//
//
//#[bench]
//fn bench_large_size_reverse_trie(b: &mut Bencher) {
//    let mut trie = ContiguousTrie::<usize>::new();
//    let range = 2usize.pow(KEY_LEN);
//    for i in 0..range {
//        trie.insert(i, &i);
//    }
//    b.iter(|| {
//        for i in 1..range {
//            let x = range - i - 1;
//            let g = trie.get(&x);
//        }
//    });
//}
//
//#[bench]
//fn bench_large_size_hashmap(b: &mut Bencher) {
//    let mut hash = HashMap::new();
//    let range = 2usize.pow(KEY_LEN);
//    for i in 0..range {
//        hash.insert(i as usize, i as usize);
//    }
//    b.iter(|| {
//        for i in 0..range {
//            let g = hash.get(&i);
//        }
//    });
//}

fn main() {
    let mut trie = ContiguousTrie::<usize>::new();

    for i in 0..1000000 {
        let str = format!("{:#022b}", i);
        let arr = str.to_owned().into_bytes();
        trie.insert(i, &arr[2..]);
    }

    for i in 0..1000000 {
        let str = format!("{:#022b}", i);
        let arr = str.to_owned().into_bytes();
        assert_eq!(trie.get(&arr[2..]).unwrap(), i);
    }
}