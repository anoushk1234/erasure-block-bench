//! Benchmark to find out how much time it takes to
//! erasure code an entire block instead of just batches
//! A shard here is 1280 bytes
// #[macro_use(shards)]
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
    println!("The size of data_shard is: {} bytes", original[0].len());
    let start = Instant::now();
    let erasure_shards = reed_solomon_16::encode(
        2000,             // total number of original shards
        2000,             // total number of recovery shards
        original.clone(), // all original shards
    )
    .unwrap();

    let end = start.elapsed();
    println!("Time elapsed in encode is: {:?} ms", end.as_millis());
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
    let start = Instant::now();
    let restored = reed_solomon_16::decode(
        2000, // total number of original shards
        2000, // total number of recovery shards
        data_shard_tuple[0..1000].to_owned(),
        parity_shard_tuple[0..1000].to_owned(),
    )
    .unwrap();
    let end = start.elapsed();

    println!("Time elapsed in decode is: {:?} ms", end.as_millis());

    let r = ReedSolomon::new(45, 45).unwrap();

    // Construct the parity shards
    let mut original = vec![];
    for _ in 0..90 {
        let mut elem: [u8; 1280] = [0u8; 1280];
        for i in 0..1280 {
            elem[i] = rand::thread_rng().gen::<u8>()
        }
        original.push(elem.clone());
    }
    let mut boxed_master_copy: Vec<Box<[u8]>> = Vec::default();
    for i in 0..(original.len()) {
        boxed_master_copy.push(Box::new(original[i]));
    }

    let start = Instant::now();
    r.encode_shards(&mut boxed_master_copy).unwrap();
    let end = start.elapsed();
    println!(
        "Time elapsed in solana encoding is: {:?} ms",
        end.as_millis()
    );

    let mut recon_shards = shards_into_option_shards(boxed_master_copy.to_vec());
    for _ in 0..45 {
        let index = rand::thread_rng().gen_range(0..90);
        recon_shards[index as usize] = None;
    }
    let start = Instant::now();
    r.reconstruct_shards(&mut recon_shards).unwrap();
    let end = start.elapsed();
    println!(
        "Time elapsed in solana decoding is: {:?} ms",
        end.as_millis()
    );
}
