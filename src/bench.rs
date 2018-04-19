use rand::{Rng, thread_rng};
use test::Bencher;
use cctrie_contiguous::SubTrie;
use std::collections::HashMap;
use chashmap::CHashMap;
use std::thread;
use std::io;
use std::io::Write;
use std::sync::Arc;
use std::time::SystemTime;

const KEY_LEN: usize = 16;
const NTHREAD: usize = 8;

// Generate random string with KEY_LEN length
fn gen_rand_str() -> String {
    let mut rng = thread_rng();
    let mut res = String::with_capacity(KEY_LEN);
    for _ in 0..KEY_LEN {
        match rng.gen_weighted_bool(2) {
            true => res.push('1'),
            false => res.push('0'),
        }
    }
    return res;
}

//#[bench]
//fn bench_rayon(b: &mut Bencher) {
//    let iter = 1000;
//    let mut base = CHashMap::new();
//    let mut vector = Vec::new();
//    let mut val: usize = 0;
//    for _ in 0..iter {
//
//        let key = gen_rand_str().into_bytes();
//        vector.push(key.clone());
//        base.insert(key, val);
//        val += 1;
//    }
//
//    let start = SystemTime::now();
//            for i in begin..end {
//                let key = &vector_arc[i];
//                let res = base_arc.get(&key.to_owned());
//
//            }
//
//
//    let end = SystemTime::now();
//    let since = end.duration_since(start).expect("Time went backwards");
//    println!("hamt_chashmap{:?}", since);
//    assert!(false);
//
//}

//#[bench]
//fn bench_chashmap(b: &mut Bencher) {
//    let iter = 20000;
//    let mut base = CHashMap::new();
//    let mut vector = Vec::new();
//    let mut val: usize = 0;
//    for _ in 0..iter {
//        let key = gen_rand_str().into_bytes();
//        vector.push(key.clone());
//        base.insert(key, val);
//        val += 1;
//    }
//
//    let mut thread_handle: Vec<thread::JoinHandle<_>> = vec![];
//    let base_arc = Arc::new(base);
//    let vector_arc = Arc::new(vector);
//    let step: usize = iter / NTHREAD;
//
//    let start = SystemTime::now();
//
//    for tid in 0..NTHREAD {
//        let base_arc = base_arc.clone();
//        let vector_arc = vector_arc.clone();
//        let begin = tid * step;
//        let end = (tid + 1) * step;
//
//        thread_handle.push(thread::spawn(move || {
//            for i in begin..end {
//                let key = &vector_arc[i];
//                let res = base_arc.get(&key.to_owned());
//            }
//        }));
//    }
//
//    for thread in thread_handle {
//        thread.join();
//    }
//
//    let end = SystemTime::now();
//    let since = end.duration_since(start).expect("Time went backwards");
//    println!("hamt_chashmap{:?}", since);
//    assert!(false);
//}
//
//#[bench]
//fn bench_hashmap(b: &mut Bencher) {
//    let iter = 20000;
//    let mut base = HashMap::new();
//    let mut vector = Vec::new();
//    let mut val: usize = 0;
//    for _ in 0..iter {
//        let key = gen_rand_str().into_bytes();
//        vector.push(key.clone());
//        base.insert(key, val);
//        val += 1;
//    }
//
//    let mut thread_handle: Vec<thread::JoinHandle<_>> = vec![];
//    let base_arc = Arc::new(base);
//    let vector_arc = Arc::new(vector);
//    let step: usize = iter / NTHREAD;
//
//    let start = SystemTime::now();
//
//    for tid in 0..NTHREAD {
//        let base_arc = base_arc.clone();
//        let vector_arc = vector_arc.clone();
//        let begin = tid * step;
//        let end = (tid + 1) * step;
//
//        thread_handle.push(thread::spawn(move || {
//            for i in begin..end {
//                let key = &vector_arc[i];
//                let res = base_arc.get(&key.to_owned());
//            }
//        }));
//    }
//
//    for thread in thread_handle {
//        thread.join();
//    }
//
//    let end = SystemTime::now();
//    let since = end.duration_since(start).expect("Time went backwards");
//    println!("hamt_hashmap{:?}", since);
//    assert!(false);
//}
//
//#[bench]
//fn bench_read(b: &mut Bencher) {
//    let iter = 20000;
//
//    let capacity = 65536;
//    let mut allocator: Vec<Option<Box<SubTrie<usize>>>> = Vec::with_capacity(capacity);
//    for i in 0..capacity {
//        allocator.push(None);
//    }
//    let mut trie = SubTrie::<usize>::new();
//    let mut vector: Vec<String> = Vec::new();
//    let mut val: usize = 0;
//    for _ in 0..iter {
//        let key = gen_rand_str();
//        vector.push(key.clone());
////        trie.insert(&mut allocator, val, &key.into_bytes());
//        val += 1;
//    }
//
//    let mut thread_handle: Vec<thread::JoinHandle<_>> = vec![];
//    let allocator_arc = Arc::new(allocator);
//    let trie_arc = Arc::new(trie);
//    let vector_arc = Arc::new(vector);
//    let step: usize = iter / NTHREAD;
//
//    let start = SystemTime::now();
//
//    for tid in 0..NTHREAD {
//        let trie_arc = trie_arc.clone();
//        let allocator_arc = allocator_arc.clone();
//        let vector_arc = vector_arc.clone();
//        let begin = tid * step;
//        let end = (tid + 1) * step;
//
//        thread_handle.push(thread::spawn(move || {
////            println!("{:?}", trie_arc.get(allocator_arc.clone().as_ref(), &"0000001111111111".to_owned().into_bytes()));
//            for i in begin..end {
//                let key = &vector_arc[i];
//                trie_arc.get(allocator_arc.clone().as_ref(), &key.as_bytes().to_owned());
//            }
//        }));
//    }
//
//    for thread in thread_handle {
//        thread.join();
//    }
//
//    let end = SystemTime::now();
//    let since = end.duration_since(start).expect("Time went backwards");
//    println!("hamt_get{:?}", since);
//    assert!(false);
//}
//
