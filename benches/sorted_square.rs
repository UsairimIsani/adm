#[macro_use]
extern crate criterion;
extern crate adm;
use adm::sorted_squares::{sorted_squares, sorted_squares_quick};
use criterion::Criterion;
fn sorted_square(c: &mut Criterion) {
    c.bench_function("Sorted Squares Usairim Implementation", |b| {
        b.iter(|| sorted_squares(vec![1, 2, 5, 6, 2, 34, 235, 56, 143, -20, -1]))
    });
}
fn sorted_square_quick(c: &mut Criterion) {
    c.bench_function("Sorted Square Quick  Leetcode Implementation", |b| {
        b.iter(|| sorted_squares_quick(vec![1, 2, 5, 6, 2, 34, 235, 56, 143, -20, -1]))
    });
}
criterion_group!(ss_bench, sorted_square, sorted_square_quick);
criterion_main!(ss_bench);
