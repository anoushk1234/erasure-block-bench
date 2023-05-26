//! Benchmark to find out how much time it takes to
//! erasure code an entire block instead of just batches
//! A shard here is 1280 bytes
#![feature(test)]
// pub mod macros;

extern crate solana_reed_solomon_erasure;

use rand::{self, Rng};
use solana_reed_solomon_erasure::*;
// use std::time::Instant;

pub fn create_block_data_shards(
    data_shards: u32,
    shard_size: u32,
) -> (Vec<Box<[u8]>>, ReedSolomon) {
    let r = ReedSolomon::new(128, 128).unwrap();

    // Construct the parity shards
    let mut original = vec![];
    for _ in 0..data_shards {
        let mut elem: Vec<u8> = Vec::with_capacity(shard_size as usize);
        for _ in 0..shard_size {
            elem.push(0);
        }
        let mut elem = elem.into_boxed_slice();
        for i in 0..shard_size {
            elem[i as usize] = rand::thread_rng().gen::<u8>()
        }
        original.push(elem.clone());
    }

    return (original, r);
}
pub fn simulate_loss_gf8(
    boxed_master_copy: Vec<Box<[u8]>>,
    range_end: u32,
) -> Vec<Option<Box<[u8]>>> {
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

pub fn encode_block_gf16(original: Vec<[u8; 1280]>, n: usize, k: usize) {
    reed_solomon_16::encode(
        n,                // total number of original shards
        k,                // total number of recovery shards
        original.clone(), // all original shards
    )
    .unwrap();
}
// pub fn recover_block_gf16(
//     n: usize,
//     k: usize,
//     data_shard_tuple: Vec<(usize, [u8; 1280])>,
//     parity_shard_tuple: Vec<(usize, Vec<u8>)>,
// ) {
//     let _restored = reed_solomon_16::decode(
//         n, // total number of original shards
//         k, // total number of recovery shards
//         data_shard_tuple[0..1000].to_owned(),
//         parity_shard_tuple[0..1000].to_owned(),
//     )
//     .unwrap();
// }

// pub fn simulate_loss_gf16(
//     original: Vec<[u8; 1280]>,
//     erasure_shards: Vec<Vec<u8>>,
// ) -> (Vec<(usize, [u8; 1280])>, Vec<(usize, Vec<u8>)>) {
//     let data_shard_tuple = original
//         .clone()
//         .into_iter()
//         .enumerate()
//         .collect::<Vec<(usize, [u8; 1280])>>();
//     let parity_shard_tuple = erasure_shards
//         .clone()
//         .into_iter()
//         .enumerate()
//         .collect::<Vec<(usize, Vec<u8>)>>();
//     return (data_shard_tuple, parity_shard_tuple);
// }
