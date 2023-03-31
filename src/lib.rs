//! Benchmark to find out how much time it takes to
//! erasure code an entire block instead of just batches
//! A shard here is 1280 bytes
#![feature(test)]
extern crate solana_reed_solomon_erasure;

use rand::{self, Rng};
use solana_reed_solomon_erasure::*;
use std::time::Instant;

pub fn create_block_data_shards(data_shards: u32) -> (Vec<Box<[u8]>>, ReedSolomon) {
    let r = ReedSolomon::new(128, 128).unwrap();

    // Construct the parity shards
    let mut original = vec![];
    for _ in 0..data_shards {
        let mut elem: [u8; 20000] = [0u8; 20000];
        for i in 0..20000 {
            elem[i] = rand::thread_rng().gen::<u8>()
        }
        original.push(elem.clone());
    }
    let mut boxed_master_copy: Vec<Box<[u8]>> = Vec::default();
    for i in 0..(original.len()) {
        boxed_master_copy.push(Box::new(original[i]));
    }
    return (boxed_master_copy, r);
}
pub fn simulate_loss(boxed_master_copy: Vec<Box<[u8]>>, range_end: u32) -> Vec<Option<Box<[u8]>>> {
    let mut recon_shards = shards_into_option_shards(boxed_master_copy.to_vec());
    for _ in 0..range_end {
        let index = rand::thread_rng().gen_range(0..range_end);
        recon_shards[index as usize] = None;
    }
    return recon_shards;
}
pub fn encode_block_gf8(mut boxed_master_copy: Vec<Box<[u8]>>, r: ReedSolomon) {
    r.encode_shards(&mut boxed_master_copy).unwrap();
}

pub fn recover_block_gf8(mut recon_shards: Vec<Option<Box<[u8]>>>, r: ReedSolomon) {
    r.reconstruct_shards(&mut recon_shards).unwrap();
}
