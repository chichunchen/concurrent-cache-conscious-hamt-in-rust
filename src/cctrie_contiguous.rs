#![feature(pointer_methods)]

use std::thread;
use std::sync::Arc;
use std::time::Duration;
use std::io;
use std::io::Write;

pub trait TrieData: Clone + Copy + Eq + PartialEq {}

impl<T> TrieData for T where T: Clone + Copy + Eq + PartialEq {}


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ContiguousTrie<T: TrieData> {
    pub data: Option<T>,
    depth: usize,
    children_offset: usize,    // the start position in allocator that place the array in hash trie
}


const KEY_LEN: usize = 16;
const KEY_GROUP: usize = 4;


// index is the sum of binary in a group
fn compute_index(key: &[u8]) -> usize {
    let mut id = 0;
    let length = if key.len() > KEY_GROUP { KEY_GROUP } else { key.len() };
    for i in 0..length {
        let temp = key[i] as usize - '0' as usize;
        id += temp << i;
    }

    return id as usize;
}

impl<T: TrieData> ContiguousTrie<T> {
    pub fn new() -> Self {
        ContiguousTrie {
            data: None,
            depth: 0,
            children_offset: 0,
        }
    }

    pub fn _new(data: Option<T>, depth: usize, children_offset: usize) -> Self {
        ContiguousTrie {
            data,
            depth,
            children_offset,
        }
    }

    pub fn insert(&mut self, allocator: &mut Vec<Option<Box<ContiguousTrie<T>>>>, value: T, key: &[u8]) {
        // add counter whenever we need to allocate a new array hash map
        static mut ALLOCATOR_COUNTER: usize = 0;
        let mut index = compute_index(key);

        // iterate until the index in allocator has allocated array trie
        if allocator[index].is_none() {
            unsafe {
                ALLOCATOR_COUNTER += 1;
                let t = ContiguousTrie::_new(Some(value), 0, ALLOCATOR_COUNTER * 16);
                allocator[index] = Some(Box::new(t));
            }
        } else {
            let mut keep: Option<T> = None;
            let mut parent_index = index;
            let mut base_offset = 0;
            let mut current_index = index;
            // find the index that we want to insert data, and store it in current_index
            while allocator[current_index].is_some() {
                keep = allocator[index].as_ref().unwrap().data.clone(); // ()
                parent_index = index; // 0
                base_offset = allocator[index].as_ref().unwrap().children_offset; // 16
                index = compute_index(&key[KEY_GROUP..]); // 8
                current_index = base_offset + index; // 24

                println!("debug parent: {} base offset: {} current_index: {}", parent_index, base_offset, current_index);
            }
            unsafe {
                let curr_depth = allocator[parent_index].as_ref().unwrap().depth;

                // deal with conflict
                if allocator[parent_index].as_ref().unwrap().data != None {
                    let orig = ContiguousTrie::_new(keep, curr_depth + 1, ALLOCATOR_COUNTER * 16);
                    allocator[base_offset] = Some(Box::new(orig));
                    allocator[parent_index].as_mut().unwrap().data = None;
                }

                // add new node
                ALLOCATOR_COUNTER += 1;
                let new = ContiguousTrie::_new(Some(value), curr_depth + 1, ALLOCATOR_COUNTER * 16);
                allocator[current_index] = Some(Box::new(new)); // TODO index -> current key group
                println!("current_index {}", current_index);
            }
        }
    }

    pub fn contain(&self, allocator: &Vec<Option<Box<ContiguousTrie<T>>>>, key: &[u8]) -> bool {
        let mut index = compute_index(key);

        while allocator[index].is_some() {
            if allocator[index].as_ref().unwrap().data.is_some() {
                return true;
            } else {
                index = allocator[index].as_ref().unwrap().children_offset;
            }
        }

        false
    }

    pub fn get(&self, allocator: &Vec<Option<Box<ContiguousTrie<T>>>>, key: &[u8]) -> Option<T> {
        self._get(allocator, key, 0)
    }

    fn _get(&self, allocator: &Vec<Option<Box<ContiguousTrie<T>>>>, key: &[u8], offset: usize) -> Option<T> {
        let index = compute_index(key);
        match allocator[offset + index].as_ref() {
            Some(T) => {
                match T.data {
                    Some(d) => Some(d),
                    None => self._get(allocator, &key[KEY_GROUP..], T.children_offset)
                }
            }
            None => None
        }
    }

    // remove the given key and return the deleted value
//    pub fn remove(&mut self, allocator: &Vec<Option<Box<ContiguousTrie<T>>>>, key: &[u8]) -> Option<T> {
//
//    }
}

#[test]
fn test_new_contiguous_trie() {
    let trie = ContiguousTrie::<()>::new();
}

#[test]
fn test_insert_contiguous_trie() {
    let capacity = 200;
    let mut allocator: Vec<Option<Box<ContiguousTrie<()>>>> = Vec::with_capacity(capacity);
    for i in 0..capacity {
        allocator.push(None);
    }
    let mut trie = ContiguousTrie::<()>::new();
    trie.insert(&mut allocator, (), &"0000000011111111".to_owned().into_bytes());
    trie.insert(&mut allocator, (), &"0000000111111111".to_owned().into_bytes());
    trie.insert(&mut allocator, (), &"0000001011111111".to_owned().into_bytes());
    trie.insert(&mut allocator, (), &"0000001111111111".to_owned().into_bytes());
    trie.insert(&mut allocator, (), &"0000010011111111".to_owned().into_bytes());
    trie.insert(&mut allocator, (), &"0000010111111111".to_owned().into_bytes());
    trie.insert(&mut allocator, (), &"0000011111111111".to_owned().into_bytes());
    trie.insert(&mut allocator, (), &"0000100011111111".to_owned().into_bytes());
    trie.insert(&mut allocator, (), &"0000100111111111".to_owned().into_bytes());
}


#[test]
fn test_get_contiguous_trie() {
    let capacity = 65536;
    let mut allocator: Vec<Option<Box<ContiguousTrie<&str>>>> = Vec::with_capacity(capacity);
    for i in 0..capacity {
        allocator.push(None);
    }
    let mut trie = ContiguousTrie::<&str>::new();

    trie.insert(&mut allocator, "abc", &"1111111111111111".to_owned().into_bytes());
    trie.insert(&mut allocator, "cde", &"0110111111111111".to_owned().into_bytes());

    let a = trie.get(&allocator, &"1111111111111111".to_owned().into_bytes());
    let ab = trie.get(&allocator, &"0110111111111111".to_owned().into_bytes());
    match a {
        Some(d) => assert_eq!(d, "abc"),
        _ => println!("find none"),
    }

    match ab {
        Some(d) => assert_ne!(d, "add"),
        _ => println!("find none"),
    }
}

#[test]
fn test_multithreaded_get() {
    let capacity = 128;
    let mut allocator: Vec<Option<Box<ContiguousTrie<usize>>>> = Vec::with_capacity(capacity);
    for i in 0..capacity {
        allocator.push(None);
    }
    let mut trie = ContiguousTrie::new();    // set start position to zero
    trie.insert(&mut allocator, 1, &"0000000011111111".to_owned().into_bytes());
    trie.insert(&mut allocator, 10, &"0000000111111111".to_owned().into_bytes());
    trie.insert(&mut allocator, 100, &"0000001011111111".to_owned().into_bytes());
    trie.insert(&mut allocator, 1000, &"0000001111111111".to_owned().into_bytes());

    let mut thread_handle: Vec<thread::JoinHandle<_>> = vec![];
    let allocator_arc = Arc::new(allocator);
    let trie_arc = Arc::new(trie);

    for tid in 0..8 {
        let trie_arc = trie_arc.clone();
        let allocator_arc = allocator_arc.clone();

        thread_handle.push(thread::spawn(move || {
            let result = trie_arc.get(allocator_arc.clone().as_ref(), &"0000001111111111".to_owned().into_bytes());
            assert_eq!(result.unwrap(), 1000);
        }));
    }

    for thread in thread_handle {
        thread.join();
    }
}


fn main() {
    let capacity = 200;
    let mut allocator: Vec<Option<Box<ContiguousTrie<usize>>>> = Vec::with_capacity(capacity);
    for i in 0..capacity {
        allocator.push(None);
    }
    let mut trie = ContiguousTrie::<usize>::new();
    trie.insert(&mut allocator, 1, &"0000000011111111".to_owned().into_bytes());
    trie.insert(&mut allocator, 2, &"0000000111111111".to_owned().into_bytes());
    trie.insert(&mut allocator, 3, &"0000001011111111".to_owned().into_bytes());
    trie.insert(&mut allocator, 4, &"0000001111111111".to_owned().into_bytes());
    trie.insert(&mut allocator, 5, &"0000010011111111".to_owned().into_bytes());
    trie.insert(&mut allocator, 6, &"0000010111111111".to_owned().into_bytes());
    trie.insert(&mut allocator, 7, &"0000011011111111".to_owned().into_bytes());
    trie.insert(&mut allocator, 8, &"0000011111111111".to_owned().into_bytes());

    println!("{:#?}", allocator);

    println!("{:#?}", trie.get(&allocator, &"0000011111111111".to_owned().into_bytes()));

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