use criterion::{criterion_group, criterion_main, Criterion};
use rust_zero_book_exercise::regex::engine::do_matching;
use std::time::Duration;

const COX_REGEX: &[(&str, &str, &str)] = &[
    ("n=02", "a?a?aa", "aa"),
    ("n=04", "a?a?a?a?aaaa", "aaaa"),
    ("n=08", "a?a?a?a?a?a?a?a?aaaaaaaa", "aaaaaaaa"),
    (
        "n=16",
        "a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
    ),
    // ("n=32", "a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
    // ("n=64", "a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
];

fn depth_first(c: &mut Criterion) {
    let mut g = c.benchmark_group("Depth First");
    g.measurement_time(Duration::from_secs(12));

    for i in COX_REGEX {
        g.bench_with_input(i.0, &(i.1, i.2), |b, args| {
            b.iter(|| do_matching(args.0, args.1, true))
        });
    }
}

fn width_first(c: &mut Criterion) {}

criterion_group!(benches, width_first, depth_first);
criterion_main!(benches);