#![feature(pointer_methods)]

pub trait TrieData: Clone + Copy + Eq + PartialEq {}

impl<T> TrieData for T where T: Clone + Copy + Eq + PartialEq {}


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ContiguousTrie<T: TrieData> {
    pub data: Option<T>,
    depth: u32,
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
    pub fn new(children_offset: usize, data: Option<T>) -> Self {
        ContiguousTrie {
            data,
            depth: 0,
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
                let t = ContiguousTrie::new(ALLOCATOR_COUNTER * 16, Some(value));
                allocator[index] = Some(Box::new(t));
            }
        } else {
            let mut keep: Option<T> = None;
            let mut last_index = index;
            while allocator[index].is_some() {
                keep = allocator[index].as_ref().unwrap().data.clone();
                last_index = index;
                index = allocator[index].as_ref().unwrap().children_offset;
            }
            unsafe {
                ALLOCATOR_COUNTER += 1;
                let t1 = ContiguousTrie::new(ALLOCATOR_COUNTER * 16, Some(value));
                allocator[index] = Some(Box::new(t1));

                // copy the conflict data to children
                ALLOCATOR_COUNTER += 1;
                let t2 = ContiguousTrie::new(ALLOCATOR_COUNTER * 16, keep);
                allocator[index + 1] = Some(Box::new(t2));

                // remove conflict
                allocator[last_index].as_mut().unwrap().data = None;
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
        let mut index = compute_index(key);

        while allocator[index].is_some() {
            if allocator[index].as_ref().unwrap().data.is_some() {
                return allocator[index].as_ref().unwrap().data;
            } else {
                index = allocator[index].as_ref().unwrap().children_offset;
            }
        }

        None
    }
}

#[test]
fn test_new_contiguous_trie() {
    let trie = ContiguousTrie::<()>::new(0, None);
}

#[test]
fn test_insert_contiguous_trie() {
    let capacity = 4096;
    let mut allocator: Vec<Option<Box<ContiguousTrie<()>>>> = Vec::with_capacity(capacity);
    for i in 0..capacity {
        allocator.push(None);
    }
    let mut trie = ContiguousTrie::<()>::new(0, None);
    trie.insert(&mut allocator, (), &"0000000011111111".to_owned().into_bytes());
    trie.insert(&mut allocator, (), &"0000000111111111".to_owned().into_bytes());
    trie.insert(&mut allocator, (), &"0000001011111111".to_owned().into_bytes());
    trie.insert(&mut allocator, (), &"0000001111111111".to_owned().into_bytes());
    trie.insert(&mut allocator, (), &"0000010011111111".to_owned().into_bytes());
    trie.insert(&mut allocator, (), &"0000010111111111".to_owned().into_bytes());
}


#[test]
fn test_get_contiguous_trie() {
    let capacity = 4096;
    let mut allocator: Vec<Option<Box<ContiguousTrie<&str>>>> = Vec::with_capacity(capacity);
    for i in 0..capacity {
        allocator.push(None);
    }
    let mut trie = ContiguousTrie::<&str>::new(0, None);

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


fn main() {
    let capacity = 128;
    let mut allocator: Vec<Option<Box<ContiguousTrie<usize>>>> = Vec::with_capacity(capacity);
    for i in 0..capacity {
        allocator.push(None);
    }
    let mut trie = ContiguousTrie::new(0, None);    // set start position to zero
    trie.insert(&mut allocator, 1, &"0000000011111111".to_owned().into_bytes());
    trie.insert(&mut allocator, 10, &"0000000111111111".to_owned().into_bytes());
    trie.insert(&mut allocator, 100, &"0000001011111111".to_owned().into_bytes());
    trie.insert(&mut allocator, 1000, &"0000001111111111".to_owned().into_bytes());
//    trie.insert(&mut allocator, 2, &"0000000111111111".to_owned().into_bytes());
//    trie.insert(&mut allocator, 3, &"0000001011111111".to_owned().into_bytes());
//    trie.insert(&mut allocator, 4, &"0000001111111111".to_owned().into_bytes());
//    trie.insert(&mut allocator, 5, &"0000010011111111".to_owned().into_bytes());
//    trie.insert(&mut allocator, 6, &"0000010111111111".to_owned().into_bytes());

    println!("{:#?}", allocator);
//    println!("{}", trie.find_allocator_empty_index(allocator, &"0000000011111111".to_owned().into_bytes(), 0));

    let a = trie.contain(&allocator, &"0010001111111111".to_owned().into_bytes());
    println!("{}", a);

    let b = trie.get(&allocator, &"0000001111111111".to_owned().into_bytes());
    println!("{:#?}", b.unwrap());


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