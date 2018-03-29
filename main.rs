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

impl<T: TrieData> Trie<T> {
    pub fn new() -> Self {
        let mut children = Vec::with_capacity(16);
        for _ in 0..16 {
            children.push(None);
        }
        Trie { data: None, depth: 0, children: children }
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub fn add_value(&mut self, value: T, key: &[u8]) -> u32 {
        if key.len() == 0 {
            self.data = Some(value);
            return 1;
        } else {
            // TODO index is the sum of binary in a group
            //let index = key[0] as usize - 'a' as usize;
            let mut id = 0;
            let length = key.len();
            if length < 4 {
                for i in 0..length {
                    let temp = key[i] as usize - '0' as usize;
                    println!("test tt {}", temp);
                    id += (temp << i);
                }
            } else {
                for i in 0..4 {
                    let temp = key[i] as usize - '0' as usize;
                    println!("test tt {}", temp);
                    id += (temp << i);
                }
            }

            let index = id as usize;
            println!("test index {}", index);

            // if the array has not been created, then create one
            if self.children[index].is_none() {
                println!("create subtree");
                self.children[index] = Some(Box::new(Trie::new()));
            }
            let value= match length {
                n if n >= 4 => self.children[index].as_mut().map(|ref mut a| a.add_value(value, &key[4..])).unwrap_or(0),
                _          => 9999,
            };
            self.depth += value;
            return value;
        }
    }

    pub fn index_base(&self, key: &[u8]) -> IndexStatus {
        if key.len() == 0 {
            self.data.map(|_| IndexStatus::FullMatch).unwrap_or(IndexStatus::StartingMatch)
        } else {
            let index = key[0] as usize - '0' as usize;
            self.children[index].as_ref().map(|ref a| a.index_base(&key[1..])).unwrap_or(IndexStatus::NoMatch)
        }
    }

    pub fn get_sub_trie<'a>(&'a self, key: &[u8]) -> Option<&'a Trie<T>> {
        let mut id = 0;
        let length = key.len();
        if length < 4 {
            for i in 0..length {
                let temp = key[i] as usize - '0' as usize;
                println!("test tt {}", temp);
                id += (temp << i);
            }
        } else {
            for i in 0..4 {
                let temp = key[i] as usize - '0' as usize;
                id += (temp << i);
            }
        }
        let index = id as usize;
        match length {
            n if n >= 4 => self.children[index].as_ref().and_then(|ref a| a.get_sub_trie(&key[4..])),
            _          => Some(&self),
        }
    }
}


#[test]
fn test_new_trie() {
    let _trie = Trie::<()>::new();
}

#[test]
fn test_add_value() {
    let mut base = Trie::new();

    base.add_value((), &"1111111111111111".to_owned().into_bytes());
    base.add_value((), &"1111101011111111".to_owned().into_bytes());
    base.add_value((), &"1111111111111011".to_owned().into_bytes());
    base.add_value((), &"1111111111111110".to_owned().into_bytes());
    base.add_value((), &"1111111111111101".to_owned().into_bytes());
    base.add_value((), &"1111111111111110".to_owned().into_bytes());
}

#[test]
fn test_index_base() {
//    let mut base = Trie::new();
//
//    base.add_value((), &"start".to_owned().into_bytes());
//    base.add_value((), &"starting".to_owned().into_bytes());
//    base.add_value((), &"red".to_owned().into_bytes());
//    base.add_value((), &"zzzzz".to_owned().into_bytes());
//    base.add_value((), &"aaaaaa".to_owned().into_bytes());
//    base.add_value((), &"redz".to_owned().into_bytes());
//
//    assert_eq!(base.index_base(&"start".to_owned().into_bytes()), IndexStatus::FullMatch);
//    assert_eq!(base.index_base(&"sta".to_owned().into_bytes()), IndexStatus::StartingMatch);
//    assert_eq!(base.index_base(&"started".to_owned().into_bytes()), IndexStatus::NoMatch);
//    assert_eq!(base.index_base(&"ggg".to_owned().into_bytes()), IndexStatus::NoMatch);
}
	
fn main() {
    let mut base = Trie::new();
    let a = 1;
    base.add_value(a, &"1111111111111111".to_owned().into_bytes());
    println!("end1");

    base.add_value(2, &"1111111111111101".to_owned().into_bytes());

    let result = base.get_sub_trie(&"1111111111111101".to_owned().into_bytes());
    println!("end2");

    match result {
        Some(trie) => match trie.data {
            Some(fuck) => println!("{}", fuck),
            _ => println!("not found 2"),
        }
        _ => println!("not found 1"),
    }

//
//    let test = &"start".to_owned().into_bytes();
//    for x in test {
//        println!("{}", x - 97); // x: i32
//    }
}
