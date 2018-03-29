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
                println!("create subtree");
                self.children[index] = Some(Box::new(Trie::new()));
            }
            let value = match key.len() {
                n if n >= KEY_GROUP => self.children[index].as_mut().map(|ref mut a| a.insert(value, &key[KEY_GROUP..])).unwrap_or(0),
                _ => 9999,
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

fn main() {
    let mut base = Trie::new();
    let a = 1;

    base.insert(a, &"1111111111111111".to_owned().into_bytes());
    base.insert(2, &"1111111111111101".to_owned().into_bytes());

    let op = base.get(&"1111111111111101".to_owned().into_bytes());
    match op {
        Some(d) => println!("{}", d),
        _ => println!("find none"),
    }
}
