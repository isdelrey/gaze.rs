#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::BenchmarkId;
use criterion::Throughput;
use std::iter::Iterator;
use gaze::storage::Store;

fn criterion_benchmark(c: &mut Criterion) {
    let mut store = Store::new();

    let mut group = c.benchmark_group("storage_write");
    for size in vec![10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000] {
        let filler = vec![8u8; size];
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::new("Write", size), &size, |b, &size| {
           b.iter(|| store.append(&filler));
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);