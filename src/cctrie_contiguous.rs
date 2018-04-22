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

impl<T: TrieData> ContiguousTrie<T> {
    pub fn new(capacity: usize) -> Self {
        let mut memory: Vec<Option<Box<SubTrie<T>>>> = Vec::with_capacity(capacity);
        // allocate memory for level-1 trie array
        for _ in 0..KEY_LEN {
            memory.push(None);
        }
        ContiguousTrie {
            memory,
            insert_counter: 0,
        }
    }

    fn allocate_child_array(&mut self) {
        for _ in 0..KEY_LEN {
            self.memory.push(None);
        }
    }

    // always use it from 0 (find from root)
    // if found empty slot, then return it, else return the value of level that conflicts
    fn find_empty_slot(&self, key: &[u8], from: usize, level: usize) -> (Option<usize>, Option<usize>) {
        let index = from + compute_index(key);
        match &self.memory[index] {
            Some(t) => {
                match t.data {
                    Some(d) => (None, Some(level)),
                    None => self.find_empty_slot(&key[KEY_GROUP..], t.children_offset.unwrap(), level + 1)
                }
            },
            None => {
                (Some(index), None)
            },
        }
    }

    fn resolve_conflict(&mut self, value: T, key: [u8; KEY_LEN], from: usize) {
        let true_key = &key[from..];
        let index = compute_index(&true_key);

        // save old, and then update the conflict node to key = None
        let old = self.memory[index].clone();
        self.memory[index].as_mut().unwrap().key = None;
        self.memory[index].as_mut().unwrap().data = None;
        self.insert_counter += 1;
        self.memory[index].as_mut().unwrap().children_offset = Some(self.insert_counter << KEY_GROUP);

        // find where to insert the old one and the new one
        let mut i = KEY_GROUP + from * KEY_GROUP;
        let mut base = 0;
        let mut old_key_group = 0;
        let mut new_key_group = 0;
        let mut depth = 0;
        while i < KEY_LEN {
            old_key_group = compute_index(&old.as_ref().unwrap().key.unwrap()[i..]);
            new_key_group = compute_index(&key[i..]);
            println!("{} {} {:?} {:?}", old_key_group, new_key_group, old.as_ref().unwrap().key, key);
            if old_key_group != new_key_group {
                break;
            }
            i += KEY_GROUP;
            depth += 1;

            // at node for confliction
            self.allocate_child_array();
            base = self.insert_counter << KEY_GROUP + old_key_group;
            self.insert_counter += 1;
            self.memory[base] = Some(Box::new(SubTrie::new(None, depth, None, Some(self.insert_counter << KEY_GROUP))));
        }
        base = self.memory[base].as_ref().unwrap().children_offset.unwrap();

//            old.unwrap().as_mut().depth = depth;
        let old_key = old.as_ref().unwrap().key;
        let old_data = old.as_ref().unwrap().data;
        self.memory[base + old_key_group] = Some(Box::new(SubTrie::new(old_data, depth + 1, old_key, None)));

        self.memory[base + new_key_group] = Some(Box::new(SubTrie::new(Some(value), depth + 1, Some(key), None)));
    }

    pub fn insert(&mut self, value: T, key: [u8; KEY_LEN]) {
        let mut index = compute_index(&key);
        let mut depth = 0;
        let mut key_head = KEY_GROUP;
        let current_clone = self.memory[index].clone();
        match current_clone {
            Some(ref c) => {
                if c.data.is_none() { // data being none means we should find deeper
                    let empty_slot_pair = self.find_empty_slot(&key, 0, 0);
                    match empty_slot_pair.0 {
                        // find the actual position for node if not conflict
                        Some(id) => {
                            self.memory[id] = Some(Box::new(SubTrie::new(Some(value), 0, Some(key), None)));
                        },
                        // else resolve it
                        None => {
                            self.resolve_conflict(value, key, empty_slot_pair.1.unwrap() << KEY_GROUP);
                        }
                    }
                    println!("empty slot {:#?}", empty_slot_pair);

                } else { // resolve conflict by allocate memory and node, but need to check if this is the level
                    self.resolve_conflict(value, key, 0);
                }
            },
            None => {
                self.allocate_child_array();
                self.memory[index] = Some(Box::new(SubTrie::new(Some(value), 0, Some(key), None)));
            }
        }
    }
}

pub trait TrieData: Clone + Copy + Eq + PartialEq {}

impl<T> TrieData for T where T: Clone + Copy + Eq + PartialEq {}


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SubTrie<T: TrieData> {
    pub data: Option<T>,
    key: Option<[u8; KEY_LEN]>,
    depth: usize,
    children_offset: Option<usize>,    // the start position in allocator that place the array in hash trie
}


const KEY_LEN: usize = 16;
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
    pub fn new(value: Option<T>, depth: usize, key: Option<[u8; KEY_LEN]>, children_offset: Option<usize>) -> Self {
        SubTrie {
            data: value,
            depth,
            key,
            children_offset,
        }
    }
}

#[test]
fn test_insert_contiguous_trie() {
    let mut trie: ContiguousTrie<usize> = ContiguousTrie::new(100);
    trie.insert(1, &"0000000000000000".to_owned().into_bytes());
    trie.insert(2, &"0000000000000001".to_owned().into_bytes());
    trie.insert(3, &"0000000000000010".to_owned().into_bytes());
    println!("{:#?}", trie);
}


fn main() {
    let mut trie: ContiguousTrie<usize> = ContiguousTrie::new(65536);
    let a: [u8; KEY_LEN] = [48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48];
    let b: [u8; KEY_LEN] = [48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 49];
    let c: [u8; KEY_LEN] = [48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 49, 48];
    let d: [u8; KEY_LEN] = [48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 49, 49];
    trie.insert(1, a);
    trie.insert(2, b);
    trie.insert(3, c);
    trie.insert(4, d);
    println!("{:#?}", trie);
}
