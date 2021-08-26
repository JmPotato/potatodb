use std::sync::Arc;

use bytes::Bytes;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;
use tokio::runtime::Builder;

use potatodb::storage::DummyMemoryStorage;
use potatodb::value::{Entry, Request};
use potatodb::Potato;

fn rw_db_with_dummy_memory_storage(thread_num: usize, key_num: usize, key_size: usize) {
    let runtime = Builder::new_multi_thread()
        .worker_threads(thread_num)
        .thread_name("rw_db_with_dummy_memory_storage")
        .build()
        .unwrap();
    runtime.block_on(async {
        let mut rng = rand::thread_rng();
        let potato = Arc::new(Potato::new(DummyMemoryStorage::default()));
        for _ in 0..key_num {
            let key = Bytes::from(format!("key-{}", rng.gen_range(0..key_size)));
            let value = Bytes::from(format!("value-{}", rng.gen_range(0..key_size)));
            let potato_ref = potato.clone();
            tokio::spawn(async move {
                potato_ref.get(&key).await.unwrap();
                potato_ref
                    .put(Request {
                        entries: vec![Entry {
                            key: key.clone(),
                            value: value.clone(),
                        }],
                    })
                    .await
                    .unwrap();
            });
        }
    });
}

fn benchmark_different_key_size(c: &mut Criterion) {
    let thread_num = 4;
    let key_num = 10000;
    for key_size in [1, 10, 100, 1000, 10000] {
        c.bench_with_input(
            BenchmarkId::new(
                format!(
                    "read/write db with dummy_memory_storage, {} thread_num, {} key_num",
                    thread_num, key_num
                ),
                format!("{} key size", key_size),
            ),
            &key_size,
            |b, key_size| {
                b.iter(|| {
                    rw_db_with_dummy_memory_storage(
                        black_box(thread_num),
                        black_box(key_num),
                        black_box(*key_size),
                    )
                })
            },
        );
    }
}

fn benchmark_different_thread_num(c: &mut Criterion) {
    let key_num = 10000;
    let key_size = 100;
    for thread_num in [1, 2, 3, 4] {
        c.bench_with_input(
            BenchmarkId::new(
                format!(
                    "read/write db with dummy_memory_storage, {} key_num, {} key_size",
                    key_num, key_size
                ),
                format!("{} thread_num", thread_num),
            ),
            &thread_num,
            |b, thread_num| {
                b.iter(|| {
                    rw_db_with_dummy_memory_storage(
                        black_box(*thread_num),
                        black_box(key_num),
                        black_box(key_size),
                    )
                })
            },
        );
    }
}

criterion_group!(
    benches,
    benchmark_different_key_size,
    benchmark_different_thread_num
);
criterion_main!(benches);
