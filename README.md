# rust-parallel-hamt-cache-conscious
[Proposal](https://github.com/chichunchen/rust-parallel-hamt-cache-conscious/blob/master/cache-conscious-concurrent.pdf)

## Instructions
- you need rust nightly for this repository currently
    - [install with rustup](https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust)
- cargo test

## TODO
- [X] Trie
    - [X] Insert, Get, Contain, Update (mutable insert)
    - [X] Delete key
- [ ] Benchmark
    - [ ] Compare the cctrie with vec, cctrie with allocate, and Rust HashMap
- [ ] Data Layout
- [ ] Concurrent by lock
- [ ] Concurrent by lock-free
- [ ] Every kind of optimization
