//! Benchmark to find out how much time it takes to
//! erasure code an entire block instead of just batches
//! A shard here is 1280 bytes
//! the n:k ratio is 2000:2000
#[macro_use(shards)]
extern crate solana_reed_solomon_erasure;

use rand::{self, Rng};
use solana_reed_solomon_erasure::*;
use std::time::Instant;

fn main() {
    let mut original = vec![];
    for _ in 0..2000 {
        let mut elem: [u8; 1280] = [0u8; 1280];
        for i in 0..1280 {
            elem[i] = rand::thread_rng().gen::<u8>()
        }
        original.push(elem.clone());
    }
    let start = Instant::now();
    let erasure_shards = reed_solomon_16::encode(
        2000,             // total number of original shards
        2000,             // total number of recovery shards
        original.clone(), // all original shards
    )
    .unwrap();

    let end = start.elapsed();

    let data_shard_tuple = original
        .clone()
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, [u8; 1280])>>();
    let parity_shard_tuple = erasure_shards
        .clone()
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, Vec<u8>)>>();
    let r_start = Instant::now();
    let restored = reed_solomon_16::decode(
        2000, // total number of original shards
        2000, // total number of recovery shards
        data_shard_tuple[0..1000].to_owned(),
        parity_shard_tuple[0..1000].to_owned(),
    )
    .unwrap();
    let r_end = r_start.elapsed();
    println!("Time elapsed in encode is: {:?} ms", end.as_millis());
    println!("Time elapsed in decode is: {:?} ms", r_end.as_millis());
    println!("The size of data_shard is: {} bytes", original[0].len());

    // let r = ReedSolomon::new(2000, 2000).unwrap();

    // // Construct the parity shards
    // r.encode_shards(
    //     &mut original
    //         .clone()
    //         .into_iter()
    //         .map(|shard| Box::new(shard))
    //         .collect::<Box<[u8]>>(),
    // )
    // .unwrap();
}
