extern crate cchamt;

extern crate test;
extern crate rand;

use test::Bencher;
use std::usize;
use std::collections::HashMap;
use rand::{Rng, thread_rng};
use cchamt::ContiguousTrie;


#[test]
fn test_new_contiguous_trie() {
    let trie = ContiguousTrie::<usize>::new();
}

#[test]
fn test_large_consecutive_insert() {
    let mut trie = ContiguousTrie::<usize>::new();

    for i in 0..65536 {
        let str = format!("{:#026b}", i);
        let arr = str.to_owned().into_bytes();
        trie.insert(i, &arr[2..]);
    }

    for i in 0..65536 {
        let str = format!("{:#026b}", i);
        let arr = str.to_owned().into_bytes();
        assert_eq!(trie.get(&arr[2..]).unwrap(), i);
    }
}

#[test]
fn test_very_large_consecutive_insert() {
    let mut trie = ContiguousTrie::<usize>::new();

    for i in 0..1000000 {
        let str = format!("{:#026b}", i);
        let arr = str.to_owned().into_bytes();
        trie.insert(i, &arr[2..]);
    }

    for i in 0..1000000 {
        let str = format!("{:#026b}", i);
        let arr = str.to_owned().into_bytes();
        assert_eq!(trie.get(&arr[2..]).unwrap(), i);
    }
}

#[bench]
fn bench_1k_get_trie(b: &mut Bencher) {
    let mut trie = ContiguousTrie::<usize>::new();
    let mut v: Vec<Vec<u8>> = Vec::new();
    let range = 1000;

    for i in 0..range {
        let str = format!("{:#026b}", i);
        let arr = str.to_owned().into_bytes();
        v.push(arr.clone());
        trie.insert(i, &arr[2..]);
    }

    b.iter(|| {
        for i in 0..range {
            let g = trie.get(&v[i][2..]);
        }
    });
}


#[bench]
fn bench_1k_get_hashmap(b: &mut Bencher) {
    let mut hash = HashMap::new();
    let range = 1000;
    for i in 0..range {
        hash.insert(i as usize, i as usize);
    }
    b.iter(|| {
        for i in 0..range {
            let g = hash.get(&i);
        }
    });
}

#[bench]
fn bench_100k_get_trie(b: &mut Bencher) {
    let mut trie = ContiguousTrie::<usize>::new();
    let mut v: Vec<Vec<u8>> = Vec::new();
    let range = 100000;

    for i in 0..range {
        let str = format!("{:#026b}", i);
        let arr = str.to_owned().into_bytes();
        v.push(arr.clone());
        trie.insert(i, &arr[2..]);
    }

    b.iter(|| {
        for i in 0..range {
            let g = trie.get(&v[i][2..]);
        }
    });
}


#[bench]
fn bench_100k_get_hashmap(b: &mut Bencher) {
    let mut hash = HashMap::new();
    let range = 100000;
    for i in 0..range {
        hash.insert(i as usize, i as usize);
    }
    b.iter(|| {
        for i in 0..range {
            let g = hash.get(&i);
        }
    });
}

#[bench]
fn bench_million_get_trie(b: &mut Bencher) {
    let mut trie = ContiguousTrie::<usize>::new();
    let mut v: Vec<Vec<u8>> = Vec::new();
    let range = 1000000;

    for i in 0..range {
        let str = format!("{:#026b}", i);
        let arr = str.to_owned().into_bytes();
        v.push(arr.clone());
        trie.insert(i, &arr[2..]);
    }

    b.iter(|| {
        for i in 0..range {
            let g = trie.get(&v[i][2..]);
        }
    });
}


#[bench]
fn bench_million_get_hashmap(b: &mut Bencher) {
    let mut hash = HashMap::new();
    let range = 10000000;
    for i in 0..range {
        hash.insert(i as usize, i as usize);
    }
    b.iter(|| {
        for i in 0..range {
            let g = hash.get(&i);
        }
    });
}


#[bench]
fn bench_10million_get_trie(b: &mut Bencher) {
    let mut trie = ContiguousTrie::<usize>::new();
    let mut v: Vec<Vec<u8>> = Vec::new();
    let range = 10000000;

    for i in 0..range {
        let str = format!("{:#026b}", i);
        let arr = str.to_owned().into_bytes();
        v.push(arr.clone());
        trie.insert(i, &arr[2..]);
    }

    b.iter(|| {
        for i in 0..range {
            let g = trie.get(&v[i][2..]);
        }
    });
}


#[bench]
fn bench_10million_get_hashmap(b: &mut Bencher) {
    let mut hash = HashMap::new();
    let range = 10000000;
    for i in 0..range {
        hash.insert(i as usize, i as usize);
    }
    b.iter(|| {
        for i in 0..range {
            let g = hash.get(&i);
        }
    });
}