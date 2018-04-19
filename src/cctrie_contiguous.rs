#![feature(pointer_methods)]

use std::thread;
use std::sync::Arc;
use std::time::Duration;
use std::io;
use std::io::Write;

#[derive(Debug)]
pub struct ContiguousTrie<T: TrieData> {
    memory: Vec<Option<Box<SubTrie<T>>>>,
    insert_counter: usize,
}

// TODO insert with different pattern

impl<T: TrieData> ContiguousTrie<T> {
    // Allocate memory at first
    pub fn new(capacity: usize) -> Self {
        let mut memory: Vec<Option<Box<SubTrie<T>>>> = Vec::with_capacity(capacity);
        for i in 0..capacity {
            memory.push(None);
        }
        ContiguousTrie {
            memory,
            insert_counter: 0,
        }
    }

    pub fn insert(&mut self, value: T, key: &[u8]) {
        let mut index = compute_index(key);

        // iterate until the index in allocator has allocated array trie
        if self.memory[index].is_none() {
            self.insert_counter += 1;
            let t = SubTrie::_new(Some(value), 0, self.insert_counter * 16);
            self.memory[index] = Some(Box::new(t));
        } else {
            let mut keep: Option<T> = None;
            let mut parent_index = index;
            let mut base_offset = 0;
            let mut current_index = index;

            // find the index that we want to insert data, and store it in current_index
            while self.memory[current_index].is_some() {
                keep = self.memory[index].as_ref().unwrap().data.clone(); // ()
                parent_index = index; // 0
                base_offset = self.memory[index].as_ref().unwrap().children_offset; // 16
                index = compute_index(&key[KEY_GROUP..]); // 8
                current_index = base_offset + index; // 24
            }
            println!("debug parent: {} base offset: {} current_index: {}", parent_index, base_offset, current_index);
            let curr_depth = self.memory[parent_index].as_ref().unwrap().depth;

            // deal with conflict
            if self.memory[parent_index].as_ref().unwrap().data != None {
                let orig = SubTrie::_new(keep, curr_depth + 1, self.insert_counter * 16);
                self.memory[base_offset] = Some(Box::new(orig));
                self.memory[parent_index].as_mut().unwrap().data = None;
            }

            // add new node
            self.insert_counter += 1;
            let new = SubTrie::_new(Some(value), curr_depth + 1, self.insert_counter * 16);
            self.memory[current_index] = Some(Box::new(new)); // TODO index -> current key group
//            println!("current_index {}", current_index);
        }
    }

    pub fn contain(&self, key: &[u8]) -> bool {
        let mut index = compute_index(key);

        while self.memory[index].is_some() {
            if self.memory[index].as_ref().unwrap().data.is_some() {
                return true;
            } else {
                index = self.memory[index].as_ref().unwrap().children_offset;
            }
        }

        false
    }

    pub fn get(&self, key: &[u8]) -> Option<T> {
        self._get(key, 0)
    }

    fn _get(&self, key: &[u8], offset: usize) -> Option<T> {
        let index = compute_index(key);
        match self.memory[offset + index].as_ref() {
            Some(T) => {
                match T.data {
                    Some(d) => Some(d),
                    None => self._get(&key[KEY_GROUP..], T.children_offset)
                }
            }
            None => None
        }
    }
}

pub trait TrieData: Clone + Copy + Eq + PartialEq {}

impl<T> TrieData for T where T: Clone + Copy + Eq + PartialEq {}


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SubTrie<T: TrieData> {
    pub data: Option<T>,
    depth: usize,
    children_offset: usize,    // the start position in allocator that place the array in hash trie
}


const KEY_LEN: usize = 32;
const KEY_GROUP: usize = 4;


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

impl<T: TrieData> SubTrie<T> {
    pub fn new() -> Self {
        SubTrie {
            data: None,
            depth: 0,
            children_offset: 0,
        }
    }

    pub fn _new(data: Option<T>, depth: usize, children_offset: usize) -> Self {
        SubTrie {
            data,
            depth,
            children_offset,
        }
    }
}

#[test]
fn test_new_contiguous_trie() {
    let trie = SubTrie::<usize>::new();
}

#[test]
fn test_insert_contiguous_trie() {
    let mut trie: ContiguousTrie<usize> = ContiguousTrie::new(100);
    trie.insert(1, &"0000000011111111".to_owned().into_bytes());
    trie.insert(2, &"0000000111111111".to_owned().into_bytes());
    trie.insert(3, &"0000001011111111".to_owned().into_bytes());
    trie.insert(4, &"0000001111111111".to_owned().into_bytes());
    trie.insert(5, &"0000010011111111".to_owned().into_bytes());
    trie.insert(6, &"0000010111111111".to_owned().into_bytes());
    trie.insert(7, &"0000011011111111".to_owned().into_bytes());
    trie.insert(8, &"0000011111111111".to_owned().into_bytes());
    println!("{:#?}", trie.get(&"0000000011111111".to_owned().into_bytes()));
    println!("{:#?}", trie);
    assert!(false);
}


#[test]
fn test_get_contiguous_trie() {
    let mut trie: ContiguousTrie<&str> = ContiguousTrie::new(65536);
    trie.insert("abc", &"1111111111111111".to_owned().into_bytes());
    trie.insert("cde", &"0110111111111111".to_owned().into_bytes());

    let a = trie.get(&"1111111111111111".to_owned().into_bytes());
    let ab = trie.get(&"0110111111111111".to_owned().into_bytes());
    assert_eq!(a.unwrap(), "abc");
    assert_ne!(ab.unwrap(), "abc");
}

#[test]
fn test_multithreaded_get() {
    let mut trie: ContiguousTrie<i32> = ContiguousTrie::new(65536);
    trie.insert(1, &"0000000011111111".to_owned().into_bytes());
    trie.insert(10, &"0000000111111111".to_owned().into_bytes());
    trie.insert(100, &"0000001011111111".to_owned().into_bytes());
    trie.insert(1000, &"0000001111111111".to_owned().into_bytes());

    let mut thread_handle: Vec<thread::JoinHandle<_>> = vec![];
    let trie_arc = Arc::new(trie);

    for tid in 0..8 {
        let trie_arc = trie_arc.clone();
        thread_handle.push(thread::spawn(move || {
            let result = trie_arc.get(&"0000001111111111".to_owned().into_bytes());
            assert_eq!(result.unwrap(), 1000);
        }));
    }

    for thread in thread_handle {
        thread.join();
    }
}

#[test]
fn test_large_size_trie() {
}

fn main() {
    let mut trie: ContiguousTrie<i32> = ContiguousTrie::new(100);
    let start = 0;
    for i in 4294967290..4294967296 {
        let s = &format!("{:#032b}", i)[2..];
        let bytes = &s.to_owned().into_bytes();
        println!("{}", s);
//        trie.insert(i, bytes);
    }
    println!("{:#?}", trie);

    // cannot borrow v as mutable more than once at a time
//    let mut v: Vec<usize> = Vec::with_capacity(4096);
//    for i in 0..4096 {
//        v.push(i);
//    }
//    let a = &mut v[0..15];
//    a[0] = 100;
//    println!("{:#?}", a);
//    let b = &mut v[15..31];
//    b[4] = 9;
//    println!("{:#?}", b);

//    let mut v: Vec<usize> = Vec::with_capacity(100);
//    for i in 0..100 {
//        v.push(i * i);
//    }
//    let ptr_v: *mut usize = v.as_mut_ptr();
//
//    unsafe {
//        let mut ptr_v_0 = ptr_v;
//        let mut ptr_v_1 = ptr_v.add(15);
//        *ptr_v_0 = 0;
//        *ptr_v_0.add(3) = 3;
//        let mut ptr_v_2 = ptr_v.add(31);
//        *ptr_v_2 = 10;
//        println!("{:#?}", v);
//    }
}
