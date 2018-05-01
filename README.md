# rust-parallel-hamt-cache-conscious
[Proposal](https://github.com/chichunchen/rust-parallel-hamt-cache-conscious/blob/master/cache-conscious-concurrent.pdf)

## Instructions
- you need rust nightly for this repository currently
    - [install with rustup](https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust)
- cargo test

## TODO
- [X] Trie
- [X] Contiguous Trie
- [ ] Benchmark
    - [X] Bench with different size (such as 1k, 10k, 1m, 10m...)
    - [X] Bench with different reading sequence (now is consecutively, should try others)
    - [ ] Bench with different size on different amount of threads
- [ ] Concurrent by lock
    - [x] Mutex per trie
    - [x] RwLock per trie
    - [ ] Mutex per element
    - [ ] RwLock per element
- [ ] Concurrent by lock-free
- [ ] Every kind of optimization

## Bench
- See [here](https://github.com/chichunchen/concurrent-cache-conscious-hamt-in-rust/blob/layout/Benchmark.ipynb).
    - In the contiguous base, sequential order such as ascending or descending performs very well when we have more than
10^5 elements
    - While other point that worth to talk about is when we read the element from hashmap randomly, the official hashmap
performs pretty bad, and our contiguous cctrie seems just become a little bit worse.