use criterion::{black_box, criterion_group, criterion_main, Criterion};
use two_sum::Solution;

#[inline]
fn benchmark(nums: Vec<i32>, target: i32, mut count: u64) {
    while count > 0 {
        Solution::two_sum_inefficient(&nums, target);
        count = count - 1;
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 5", |b| {
        b.iter(|| benchmark(vec![2, 7, 11, 15], 9, black_box(5)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
