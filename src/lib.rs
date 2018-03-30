extern crate rand;
// extern crate test;
// use test::Bencher;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::time::SystemTime;

pub trait TrieData: Clone + Copy + Eq + PartialEq {}

impl<T> TrieData for T where T: Clone + Copy + Eq + PartialEq {}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Trie<T: TrieData> {
    pub data: Option<T>,
    depth: u32,
    children: Vec<Option<Box<Trie<T>>>>,
}


#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum IndexStatus {
    FullMatch,
    StartingMatch,
    NoMatch,
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

impl<T: TrieData> Trie<T> {
    pub fn new() -> Self {
        let mut children = Vec::with_capacity(KEY_LEN);
        for _ in 0..KEY_LEN {
            children.push(None);
        }
        Trie { data: None, depth: 0, children: children }
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }

    // insert a generic value by a key, and the key should be in binary format
    pub fn insert(&mut self, value: T, key: &[u8]) -> u32 {
        if key.len() == 0 {
            self.data = Some(value);
            return 1;
        } else {
            let index = compute_index(key);

            // if the array has not been created, then create one
            if self.children[index].is_none() {
                // println!("create subtree");
                self.children[index] = Some(Box::new(Trie::new()));
            }
            let value = match key.len() {
                n if n >= KEY_GROUP => self.children[index].as_mut().map(|ref mut a| a.insert(value, &key[KEY_GROUP..])).unwrap_or(0),
                _ => 9999,  // TODO value should be Option
            };
            self.depth += value;
            return value;
        }
    }

    // get value from key
    pub fn get(&self, key: &[u8]) -> Option<T> {
        let result = self.get_sub_trie(key);

        match result {
            Some(trie) => match trie.data {
                Some(data) => return Some(data),
                _ => return None,
            }
            _ => return None,
        }
    }

    // return true if the key exists, otherwise, return false
    pub fn contain(&self, key: &[u8]) -> bool {
        let trie_op = self.get_sub_trie(key);
        match trie_op {
            Some(trie) => {
                if trie.data == None {
                    return false;
                } else {
                    return true;
                }
            }
            _ => return false,
        }
    }

    pub fn index_base(&self, key: &[u8]) -> IndexStatus {
        if key.len() == 0 {
            self.data.map(|_| IndexStatus::FullMatch).unwrap_or(IndexStatus::StartingMatch)
        } else {
            let index = compute_index(key);
            self.children[index].as_ref().map(|ref a| a.index_base(&key[KEY_GROUP..])).unwrap_or(IndexStatus::NoMatch)
        }
    }

    pub fn get_sub_trie<'a>(&'a self, key: &[u8]) -> Option<&'a Trie<T>> {
        let index = compute_index(key);
        match key.len() {
            n if n >= KEY_GROUP => self.children[index].as_ref().and_then(|ref a| a.get_sub_trie(&key[KEY_GROUP..])),
            _ => Some(&self),
        }
    }

    // TODO delete the data in the trie found by the key
    pub fn delete_key(&mut self, key: &[u8]) {
        if key.len() == 0 {
            self.data = None;
        } else {
            let index = compute_index(key);

            if index >= KEY_GROUP {
                self.children[index].as_mut().map(|ref mut a| a.delete_key(&key[KEY_GROUP..]));
            }
        }
    }
}

// Generate random string with KEY_LEN length
fn gen_rand_str() -> String {
    let mut rng = thread_rng();
    let mut res = String::with_capacity(KEY_LEN);
    for _ in 0..KEY_LEN {
        match rng.gen_weighted_bool(2) {
            true  => res.push('1'),
            false => res.push('0'),
        }
    }
    return res;
}

#[test]
fn test_new_trie() {
    let _trie = Trie::<()>::new();
}

#[test]
fn test_insert() {
    let mut base = Trie::new();

    base.insert((), &"1111111111111111".to_owned().into_bytes());
    base.insert((), &"1111101011111111".to_owned().into_bytes());
    base.insert((), &"1111111111111011".to_owned().into_bytes());
    base.insert((), &"1111111111111110".to_owned().into_bytes());
    base.insert((), &"1111111111111101".to_owned().into_bytes());
    base.insert((), &"1111111111111110".to_owned().into_bytes());
}

#[test]
fn test_index_base() {
    let mut base = Trie::new();

    base.insert((), &"1111111111111111".to_owned().into_bytes());
    base.insert((), &"0110111111111111".to_owned().into_bytes());

    assert_eq!(base.index_base(&"1111111111111111".to_owned().into_bytes()), IndexStatus::FullMatch);
    assert_eq!(base.index_base(&"1110111111111110".to_owned().into_bytes()), IndexStatus::NoMatch);
}

#[test]
fn test_get() {
    let mut base = Trie::new();

    base.insert("abc", &"1111111111111111".to_owned().into_bytes());
    base.insert("cde", &"0110111111111111".to_owned().into_bytes());

    let a = base.get(&"1111111111111111".to_owned().into_bytes());
    let ab = base.get(&"0110111111111111".to_owned().into_bytes());

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
fn test_contain() {
    let mut base = Trie::new();
    let key1 = &"1111111111111111".to_owned().into_bytes();
    let key2 = &"0110111111111111".to_owned().into_bytes();

    base.insert("abc", key1);
    base.insert("cde", key2);

    assert!(base.contain(key1));
    assert!(base.contain(key2));
}

#[test]
fn test_update() {
    let mut base = Trie::new();
    let key1 = &"1111111111111111".to_owned().into_bytes();
    base.insert(1, key1);
    base.insert(2, key1);
    base.insert(1, key1);
    base.insert(1, key1);
    base.insert(2, key1);
    let result = base.get(key1);

    assert!(base.contain(key1));

    match result {
        Some(d) => assert_eq!(d, 2),
        _ => assert!(false),
    }
}

#[test]
fn test_delete() {
    let mut base = Trie::new();
    let key1 = &"1111111111111111".to_owned().into_bytes();
    base.insert(1, key1);
    base.delete_key(key1);

    assert!(!base.contain(key1));
}

fn timer(f: &Fn(u32), val: u32) {
    let start = SystemTime::now();
    f(val);
    let end = SystemTime::now();
    let since = end.duration_since(start).expect("Time went backwards");
    println!("{:?}", since);
    
}

fn rust_insert(iter: u32) {
    let mut val : u32 = 0;
    let mut hsmp = HashMap::new();
    let start = SystemTime::now();
    for _ in 0..iter { hsmp.insert(gen_rand_str(), val); val += 1; }
    let end = SystemTime::now();
    let since = end.duration_since(start).expect("Time went backwards");
    println!("rust_insert{:?}", since);
}

fn hamt_insert(iter: u32) {
    let mut val : u32 = 0;
    let mut base = Trie::new();
    let start = SystemTime::now();
    for _ in 0..iter { base.insert(val, &gen_rand_str().into_bytes()); val += 1; }
    let end = SystemTime::now();
    let since = end.duration_since(start).expect("Time went backwards");
    println!("hamt_insert{:?}", since);
}

fn rust_contain(iter: u32) {
    let mut val : u32 = 0;
    let mut hsmp = HashMap::new();
    for _ in 0..iter { hsmp.insert(gen_rand_str(), val); val += 1; }

    let start = SystemTime::now();
    for _ in 0..iter { hsmp.contains_key(&gen_rand_str()); }
    let end = SystemTime::now();
    let since = end.duration_since(start).expect("Time went backwards");
    println!("rust_contain{:?}", since);
}

fn hamt_contain(iter: u32) {
    let mut val : u32 = 0;
    let mut base = Trie::new();
    for _ in 0..iter { base.insert(val, &gen_rand_str().into_bytes()); val += 1; }

    let start = SystemTime::now();
    for _ in 0..iter { base.contain(&gen_rand_str().into_bytes()); }
    let end = SystemTime::now();
    let since = end.duration_since(start).expect("Time went backwards");
    println!("hamt_contain{:?}", since);
}

fn rust_get(iter: u32) {
    let mut val : u32 = 0;
    let mut hsmp = HashMap::new();
    for _ in 0..iter { hsmp.insert(gen_rand_str(), val); val += 1; }

    let start = SystemTime::now();
    for _ in 0..iter { hsmp.get_mut(&gen_rand_str()); }
    let end = SystemTime::now();
    let since = end.duration_since(start).expect("Time went backwards");
    println!("rust_get{:?}", since);
}

fn hamt_get(iter: u32) {
    let mut val : u32 = 0;
    let mut base = Trie::new();
    for _ in 0..iter { base.insert(val, &gen_rand_str().into_bytes()); val += 1; }

    let start = SystemTime::now();
    for _ in 0..iter { base.get(&gen_rand_str().into_bytes()); }
    let end = SystemTime::now();
    let since = end.duration_since(start).expect("Time went backwards");
    println!("hamt_get{:?}", since);
}

fn rust_delete(iter: u32) {
    let mut val : u32 = 0;
    let mut hsmp = HashMap::new();
    let mut vec = Vec::new();
    for _ in 0..iter { 
        let tmp = gen_rand_str();
        vec.push(tmp.clone());
        hsmp.insert(tmp, val); 
        val += 1; 
    }

    let start = SystemTime::now();
    for _ in 0..iter {
        let tmp = vec.pop();
        hsmp.remove(&gen_rand_str()); 
    }
    let end = SystemTime::now();
    let since = end.duration_since(start).expect("Time went backwards");
    println!("hamt_delete{:?}", since);
}

fn hamt_delete(iter: u32) {
    let mut val : u32 = 0;
    let mut base = Trie::new();
    let mut vec = Vec::new();
    for _ in 0..iter { 
        let tmp = gen_rand_str();
        vec.push(tmp.clone());
        base.insert(val, &tmp.into_bytes()); 
        val += 1; 
    }

    let start = SystemTime::now();
    for _ in 0..iter {
        let tmp = vec.pop();
        base.delete_key(&tmp.unwrap().into_bytes()); 
    }
    let end = SystemTime::now();
    let since = end.duration_since(start).expect("Time went backwards");
    println!("hamt_delete{:?}", since);
}

fn main() {
    let iter : u32 = 50000;

    // timer(&rust_insert, iter);
    // timer(&hamt_insert, iter);
    
    rust_insert(iter);
    hamt_insert(iter);

    rust_contain(iter);
    hamt_contain(iter);

    rust_get(iter);
    hamt_get(iter);
    
    rust_delete(iter);
    hamt_delete(iter);
}

