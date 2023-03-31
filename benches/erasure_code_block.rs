use {erasure_block_bench::*, glassbench::*};

fn erasure_block_bench_gf8(b: &mut Bench) {
    let (mc, r) = create_block_data_shards(256);
    b.task(
        "Encoding - block size: 2.56MB, shard size: 20KB, n:128, k:128",
        |task| {
            task.iter(|| encode_block_gf8(mc.clone(), r.clone()));
        },
    );

    let recovery_shards = simulate_loss(mc, 128);
    b.task(
        "Recovering - block size: 2.56MB, shard size: 20KB, n:128, k:128",
        |task| {
            task.iter(|| recover_block_gf8(recovery_shards.clone(), r.clone()));
        },
    );
}

glassbench!(
    "Benchmark Erasure Coding Solana Block",
    erasure_block_bench_gf8,
);
