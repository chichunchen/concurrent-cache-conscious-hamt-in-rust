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
    - [ ] Bench with different reading sequence (now is consecutively, should try others)
    - [ ] Bench with different size on different amount of threads
- [ ] Concurrent by lock
- [ ] Concurrent by lock-free
- [ ] Every kind of optimization

## Bench
```
test cctrie_contiguous::bench_1k_get_hashmap        ... bench:        23,076 ns/iter (+/- 8,118)
test cctrie_contiguous::bench_1k_get_trie           ... bench:        34,739 ns/iter (+/- 9,631)
test cctrie_contiguous::bench_100k_get_hashmap      ... bench:     4,984,978 ns/iter (+/- 1,116,945)
test cctrie_contiguous::bench_100k_get_trie         ... bench:     3,545,234 ns/iter (+/- 1,194,679)
test cctrie_contiguous::bench_10million_get_hashmap ... bench: 1,223,923,991 ns/iter (+/- 251,666,506)
test cctrie_contiguous::bench_10million_get_trie    ... bench:   391,523,171 ns/iter (+/- 164,836,998)
```
