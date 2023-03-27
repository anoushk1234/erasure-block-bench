# Erasure Block Bench

A benchmark to see how fast we can erasure code a block with 1D Reed Solomon erasure coding. SIMD Accelaration is turned off for arm cpus.

## Benchmakrs

**M1 Pro**
```
The size of data_shard is: 1280 bytes
Time elapsed in encode is: 32 ms
Time elapsed in decode is: 51 ms
Time elapsed in solana encoding is: 24 ms
Time elapsed in solana decoding is: 43 ms
```

**c3.large.x86**
```
The size of data_shard is: 1280 bytes
Time elapsed in encode is: 32 ms
Time elapsed in decode is: 57 ms
Time elapsed in solana encoding is: 10 ms
Time elapsed in solana decoding is: 13 ms
```