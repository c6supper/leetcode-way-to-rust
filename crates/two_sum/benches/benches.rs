<<<<<<< HEAD
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
=======
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use two_sum::Solution;

#[inline]
fn insufficient_two_sum(nums: Vec<i32>, target: i32, mut count: u64) {
    while count > 0 {
        Solution::two_sum_inefficient(&nums, target);
        count = count - 1;
    }
}

#[inline]
fn sufficient_two_sum(nums: Vec<i32>, target: i32, mut count: u64) {
    while count > 0 {
        Solution::two_sum(&nums, target);
        count = count - 1;
    }
}

pub fn insufficient_benchmark(c: &mut Criterion) {
    c.bench_function("fib 1", |b| {
        b.iter(|| insufficient_two_sum(vec![2, 7, 11, 15], 9, black_box(1)))
    });
}

pub fn sufficient_benchmark(c: &mut Criterion) {
    c.bench_function("fib 1", |b| {
        b.iter(|| sufficient_two_sum(vec![2, 7, 11, 15], 9, black_box(1)))
    });
}

criterion_group!(benches, insufficient_benchmark, sufficient_benchmark);
criterion_main!(benches);
>>>>>>> 2e1634b5e6f3a9e17c79e85a5f79dc693da62036
