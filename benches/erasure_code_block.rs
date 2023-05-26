use rand::Rng;

use {erasure_block_bench::*, glassbench::*};

fn erasure_block_bench_gf8(b: &mut Bench) {
    let (mc, r) = create_block_data_shards(256, 20000);
    b.task(
        "Encoding gf8 - block size: 2.56MB, shard size: 20KB, n:128, k:128",
        |task| {
            task.iter(|| encode_block_gf8(mc.clone(), r.clone()));
        },
    );

    let recovery_shards = simulate_loss_gf8(mc, 128);
    b.task(
        "Recovering gf8 - block size: 2.56MB, shard size: 20KB, n:128, k:128",
        |task| {
            task.iter(|| recover_block_gf8(recovery_shards.clone(), r.clone()));
        },
    );
    let mut original = vec![];
    for _ in 0..2000 {
        let mut elem: [u8; 1280] = [0u8; 1280];
        for i in 0..1280 {
            elem[i] = rand::thread_rng().gen::<u8>()
        }
        original.push(elem.clone());
    }

    b.task(
        "Encoding gf16 - block size: 2.56MB, shard size: 1280 bytes, n:2000, k:2000",
        |task| {
            task.iter(|| encode_block_gf16(original.clone(), 2000, 2000));
        },
    );
    // let (dr, er) = simulate_loss_gf16(original);
    // b.task(
    //     "Recovering gf16 - block size: 2.56MB, shard size: 1280 bytes, n:2000, k:2000",
    //     |task| {
    //         task.iter(|| recover_block_gf16(2000, 2000, dr.clone(), er.clone()));
    //     },
    // );
}

glassbench!(
    "Benchmark Erasure Coding Solana Block",
    erasure_block_bench_gf8,
);
