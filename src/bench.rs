use rand::{Rng, thread_rng};
use test::Bencher;
use contiguous_cctrie_one_mutex::SubTrie;
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


#[bench]
fn bench_read(b: &mut Bencher) {
    let iter = 100000;
    for i in 0..iter {
        let str = binary_format!(i);
        let arr = str.to_owned().into_bytes();
        trie.insert(i, &arr[2..]);
    }

    let trie = Arc::new(RwContiguousTrie::<usize>::new(32, 8));
    let mut thread_handle: Vec<thread::JoinHandle<_>> = vec![];
    let step: usize = iter / NTHREAD;

    let start = SystemTime::now();

    for tid in 0..NTHREAD {
        let thread_trie = trie.clone();
        let begin = tid * step;
        let end = (tid + 1) * step;

        thread_handle.push(thread::spawn(move || {
//            println!("{:?}", trie_arc.get(allocator_arc.clone().as_ref(), &"0000001111111111".to_owned().into_bytes()));
            for i in begin..end {
                let str = binary_format!(i);
                let arr = str.to_owned().into_bytes();
                assert_eq!(thread_trie.get(&arr[2..]).unwrap(), i);
            }
        }));
    }

    for thread in thread_handle {
        thread.join();
    }

    let end = SystemTime::now();
    let since = end.duration_since(start).expect("Time went backwards");
    println!("cccctrie_get{:?}", since);
    assert!(false);
}

