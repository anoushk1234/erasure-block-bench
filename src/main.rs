//! Benchmark to find out how much time it takes to
//! erasure code an entire block instead of just batches
//! A shard here is 1280 bytes
//! the n:k ratio is 2000:2000

use rand::{self, Rng};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut original = vec![];
    for _ in 0..2000 {
        let mut elem: [u8; 1280] = [0u8; 1280];
        for i in 0..1280 {
            elem[i] = rand::thread_rng().gen::<u8>()
        }
        original.push(elem.clone());
    }
    let _erasure_shards = reed_solomon_16::encode(
        2000,             // total number of original shards
        2000,             // total number of recovery shards
        original.clone(), // all original shards
    )
    .unwrap();

    let end = start.elapsed();
    println!("Time elapsed in encode is: {:?} ms", end.as_millis());
    println!("The size of data_shard is: {} bytes", original[0].len());
}
