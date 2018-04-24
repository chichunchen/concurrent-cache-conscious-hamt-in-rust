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
    - [ ] Bench with different size (such as 1k, 10k, 1m, 10m...)
    - [ ] Bench with different size on different amount of threads
- [ ] Concurrent by lock
- [ ] Concurrent by lock-free
- [ ] Every kind of optimization
