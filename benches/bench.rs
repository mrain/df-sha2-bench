//! Benchmark of
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use df_sha2_bench::{hash_chain, hash_chain2};

fn sha2_hash_chain_benchmark(c: &mut Criterion) {
    let mut grp = c.benchmark_group("Sha2 hash chain");
    let num_iters: u64 = 100000;
    grp.throughput(Throughput::Elements(num_iters));
    grp.bench_function("Digest", |b| b.iter(|| hash_chain(num_iters)));
    grp.bench_function("Manual", |b| b.iter(|| hash_chain2(num_iters)));
    grp.finish();
}

criterion_group!(benches, sha2_hash_chain_benchmark);
criterion_main!(benches);
