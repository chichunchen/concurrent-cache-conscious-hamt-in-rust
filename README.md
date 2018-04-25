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
    - [ ] Bench with different Key Length (16, 20, 24, 28, 32..., 28 and 32 may need lots of disk space)
    - [ ] Bench with different reading sequence (now is consecutively, should try others)
    - [ ] Bench with different size on different amount of threads
- [ ] Concurrent by lock
    - [x] Mutex per trie
    - [x] RwLock per trie
    - [ ] Mutex per element
    - [ ] RwLock per element
- [ ] Concurrent by lock-free
- [ ] Every kind of optimization

## Bench
- KEY_LENGTH=24
- opt=0
```
test cctrie_contiguous::bench_1k_get_hashmap        ... bench:        23,076 ns/iter (+/- 8,118)
test cctrie_contiguous::bench_1k_get_trie           ... bench:        34,739 ns/iter (+/- 9,631)
test cctrie_contiguous::bench_100k_get_hashmap      ... bench:     4,984,978 ns/iter (+/- 1,116,945)
test cctrie_contiguous::bench_100k_get_trie         ... bench:     3,545,234 ns/iter (+/- 1,194,679)
test cctrie_contiguous::bench_10million_get_hashmap ... bench: 1,223,923,991 ns/iter (+/- 251,666,506)
test cctrie_contiguous::bench_10million_get_trie    ... bench:   391,523,171 ns/iter (+/- 164,836,998)
```
- opt=3
```release
test bench_1k_get_hashmap         ... bench:      16,833 ns/iter (+/- 3,661)
test bench_1k_get_trie            ... bench:      31,253 ns/iter (+/- 5,836)
test bench_100k_get_hashmap       ... bench:   3,451,567 ns/iter (+/- 682,286)
test bench_100k_get_trie          ... bench:   3,656,802 ns/iter (+/- 778,616)
test bench_million_get_hashmap    ... bench:  45,478,866 ns/iter (+/- 29,094,426)
test bench_million_get_trie       ... bench:  36,220,772 ns/iter (+/- 9,297,379)
test bench_10_million_get_hashmap ... bench: 940,628,261 ns/iter (+/- 168,047,539)
test bench_10_million_get_trie    ... bench: 386,848,363 ns/iter (+/- 114,999,290)
```
- KEY_LENGTH=28
- opt=3
```
test bench_1k_get_hashmap         ... bench:      18,650 ns/iter (+/- 3,160)
test bench_1k_get_trie            ... bench:      53,035 ns/iter (+/- 8,726)
test bench_100k_get_hashmap       ... bench:   4,361,956 ns/iter (+/- 1,323,714)
test bench_100k_get_trie          ... bench:   5,574,079 ns/iter (+/- 2,566,200)
test bench_million_get_hashmap    ... bench:  47,687,657 ns/iter (+/- 29,172,924)
test bench_million_get_trie       ... bench:  56,702,678 ns/iter (+/- 15,974,176)
test bench_10_million_get_hashmap ... bench: 950,166,810 ns/iter (+/- 197,380,886)
test bench_10_million_get_trie    ... bench: 562,632,831 ns/iter (+/- 114,742,120)
```