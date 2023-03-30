# Erasure Block Bench

A benchmark to see how fast we can erasure code a block with 1D Reed Solomon erasure coding. SIMD Accelaration is turned off for arm cpus.

## Benchmakrs

**M1 Pro**
```
The size of data_shard is: 1280 bytes
n:k -> 2000:2000 (because gf16)
block size is 2.56 MB

Time elapsed in gf16 encode is: 32 ms
Time elapsed in gf16 decode is: 51 ms
Time elapsed in gf8 encoding is: 24 ms
Time elapsed in gf8 decoding is: 43 ms
```

**c3.large.x86**
```
The size of data_shard is: 1280 bytes
n:k -> 45:45 (because gf8)
block size is 0.576 MB (a quarter of a solana block)

Time elapsed in gf16 encode is: 33 ms
Time elapsed in gf16 decode is: 57 ms
Time elapsed in gf8 encoding is: 6 ms
Time elapsed in gf8 decoding is: 3 ms
```
