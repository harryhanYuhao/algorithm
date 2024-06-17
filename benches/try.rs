use algorithm::sort::{heap_sort, insertion_sort, merge_sort};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;

pub fn shuffled_vec_i64(upper_bound: i64) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let mut vals: Vec<i64> = (0..upper_bound).collect();
    vals.shuffle(&mut rng);
    vals
}

fn criterion_benchmark(c: &mut Criterion) {
    let vals = shuffled_vec_i64(1000000);
    c.bench_function("heap sort 1000000", |b| b.iter(|| heap_sort(black_box(&vals))));
    c.bench_function("merge sort 1000000", |b| {
        b.iter(|| merge_sort(black_box(&vals)))
    });
    // c.bench_function("insertion sort 1000000", |b| {
    //     b.iter(|| insertion_sort(black_box(&vals)))
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
