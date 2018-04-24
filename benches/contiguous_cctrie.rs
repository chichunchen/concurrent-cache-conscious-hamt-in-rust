#![feature(test)]

#[macro_use]
extern crate cchamt;

extern crate test;

use test::Bencher;
use std::usize;
use std::collections::HashMap;
use cchamt::ContiguousTrie;


#[bench]
fn bench_1k_get_trie(b: &mut Bencher) {
    let mut trie = ContiguousTrie::<usize>::new(28, 4);
    let mut v: Vec<Vec<u8>> = Vec::new();
    let range = 1000;

    for i in 0..range {
        let str = binary_format!(i);
        let arr = str.to_owned().into_bytes();
        v.push(arr.clone());
        trie.insert(i, &arr[2..]);
    }

    b.iter(|| {
        for i in 0..range {
            let _g = trie.get(&v[i][2..]);
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
            let _g = hash.get(&i);
        }
    });
}

#[bench]
fn bench_100k_get_trie(b: &mut Bencher) {
    let mut trie = ContiguousTrie::<usize>::new(28, 4);
    let mut v: Vec<Vec<u8>> = Vec::new();
    let range = 100000;

    for i in 0..range {
        let str = binary_format!(i);
        let arr = str.to_owned().into_bytes();
        v.push(arr.clone());
        trie.insert(i, &arr[2..]);
    }

    b.iter(|| {
        for i in 0..range {
            let _g = trie.get(&v[i][2..]);
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
            let _g = hash.get(&i);
        }
    });
}

#[bench]
fn bench_million_get_trie(b: &mut Bencher) {
    let mut trie = ContiguousTrie::<usize>::new(28, 4);
    let mut v: Vec<Vec<u8>> = Vec::new();
    let range = 1000000;

    for i in 0..range {
        let str = binary_format!(i);
        let arr = str.to_owned().into_bytes();
        v.push(arr.clone());
        trie.insert(i, &arr[2..]);
    }

    b.iter(|| {
        for i in 0..range {
            let _g = trie.get(&v[i][2..]);
        }
    });
}


#[bench]
fn bench_million_get_hashmap(b: &mut Bencher) {
    let mut hash = HashMap::new();
    let range = 1000000;
    for i in 0..range {
        hash.insert(i as usize, i as usize);
    }
    b.iter(|| {
        for i in 0..range {
            let _g = hash.get(&i);
        }
    });
}


#[bench]
fn bench_10_million_get_trie(b: &mut Bencher) {
    let mut trie = ContiguousTrie::<usize>::new(28, 4);
    let mut v: Vec<Vec<u8>> = Vec::new();
    let range = 10000000;

    for i in 0..range {
        let str = binary_format!(i);
        let arr = str.to_owned().into_bytes();
        v.push(arr.clone());
        trie.insert(i, &arr[2..]);
    }

    b.iter(|| {
        for i in 0..range {
            let _g = trie.get(&v[i][2..]);
        }
    });
}


#[bench]
fn bench_10_million_get_hashmap(b: &mut Bencher) {
    let mut hash = HashMap::new();
    let range = 10000000;
    for i in 0..range {
        hash.insert(i as usize, i as usize);
    }
    b.iter(|| {
        for i in 0..range {
            let _g = hash.get(&i);
        }
    });
}

#[bench]
fn bench_100_million_get_trie(b: &mut Bencher) {
    let mut trie = ContiguousTrie::<usize>::new(28, 4);
    let mut v: Vec<Vec<u8>> = Vec::new();
    let range = 10000000;

    for i in 0..range {
        let str = binary_format!(i);
        let arr = str.to_owned().into_bytes();
        v.push(arr.clone());
        trie.insert(i, &arr[2..]);
    }

    b.iter(|| {
        for i in 0..range {
            let _g = trie.get(&v[i][2..]);
        }
    });
}


#[bench]
fn bench_100_million_get_hashmap(b: &mut Bencher) {
    let mut hash = HashMap::new();
    let range = 100000000;
    for i in 0..range {
        hash.insert(i as usize, i as usize);
    }
    b.iter(|| {
        for i in 0..range {
            let _g = hash.get(&i);
        }
    });
}