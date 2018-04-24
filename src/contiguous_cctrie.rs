pub trait TrieData: Clone + Copy + Eq + PartialEq {}

impl<T> TrieData for T where T: Clone + Copy + Eq + PartialEq {}

/// Private Functions for this module
/// compute the depth in the trie using the array index of trie.memory
#[inline(always)]
fn get_depth(key_length: usize, index: usize) -> usize {
    let mut depth = 0;
    let mut multitude = key_length;
    let mut compare = key_length;

    while index >= compare {
        depth += 1;
        multitude *= key_length;
        compare += multitude;
    }
    depth
}

/// Core Data structure
#[derive(Debug)]
pub struct ContiguousTrie<T: TrieData> {
    memory: Vec<Option<SubTrie<T>>>,
    key_length: usize,
    key_group: usize,
}


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SubTrie<T: TrieData> {
    pub data: Option<T>,
    depth: usize,
    children_offset: Option<usize>,    // the start position in allocator that place the array in hash trie
}

// Contiguous store all the nodes contiguous with the sequential order of key
impl<T: TrieData> ContiguousTrie<T> {
    pub fn new(key_length: usize, key_group: usize) -> Self {
        let mut memory: Vec<Option<SubTrie<T>>>;
        // init with all nodes that is not leaf
        // length = summation of KEY_LEN^1 to KEY_LEN^(KEY_LEN/KEY_GROUP-1)
        {
            let mut nodes_length = 0;
            let mut multitude = key_length;
            for _ in 0..(key_length / key_group - 1) {
                nodes_length += multitude;
                multitude *= key_length;
            }
            memory = Vec::with_capacity(nodes_length);

            for i in 0..nodes_length {
                memory.push(Some(SubTrie {
                    data: None,
                    depth: get_depth(key_length, i),
                    children_offset: Some((i + 1) * key_length),
                }));
            }
        }

        ContiguousTrie {
            memory,
            key_length,
            key_group,
        }
    }

    // return the index in the first <= 4 bits
// for instances: 0000 0000 -> 0
    #[inline(always)]
    fn compute_index(&self, key: &[u8]) -> usize {
        let mut id = 0;
        let length = if key.len() > self.key_group { self.key_group } else { key.len() };
        for i in 0..length {
            let temp = key[i] as usize - '0' as usize;
            id += temp << (length - i - 1);
        }
        return id as usize;
    }

    // key should be 1-1 mapping to self memory array
    #[inline(always)]
    fn key2index(&self, key: &[u8]) -> usize {
        let mut current_index = self.compute_index(key);
        let mut key_start = 0;
        while self.memory.len() > current_index && self.memory[current_index].is_some() {
            match &self.memory[current_index] {
                Some(a) => {
                    match a.children_offset {
                        Some(b) => {
                            key_start += self.key_group;
                            current_index = b + self.compute_index(&key[key_start..]);
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
            depth: get_depth(self.key_length, current_index),
            children_offset: None,
        });
    }

    pub fn contain(&self, key: &[u8]) -> bool {
        let current_index = self.key2index(key);
        if self.memory.len() <= current_index {
            return false;
        }
        match &self.memory[current_index] {
            Some(_) => {
                true
            }
            None => false,
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<T> {
        let current_index = self.key2index(key);
        if self.memory.len() <= current_index {
            return None;
        }
        match &self.memory[current_index] {
            Some(a) => {
                a.data
            }
            None => None,
        }
    }
}

// TODO should change this to key_length+2, which is {:0key_length+2b}
#[macro_export]
macro_rules! binary_format {
    ($x:expr) => {
        format!("{:#030b}", $x)
    };
}


fn main() {
    let mut trie = ContiguousTrie::<usize>::new(28, 4);

    for i in 0..1000000 {
        let str = binary_format!(i);
        let arr = str.to_owned().into_bytes();
        trie.insert(i, &arr[2..]);
    }

    for i in 0..1000000 {
        let str = binary_format!(i);
        let arr = str.to_owned().into_bytes();
        assert_eq!(trie.get(&arr[2..]).unwrap(), i);
    }
}
