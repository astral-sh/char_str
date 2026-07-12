use core::hint::black_box;
use std::time::Duration;

use char_str::{CharStr, CharString};
use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

struct Case {
    name: &'static str,
    parts: &'static [&'static str],
}

const CONCAT_CASES: [Case; 3] = [
    Case { name: "inline_16", parts: &["abcdefgh", "ijklmnop"] },
    Case { name: "heap_17", parts: &["abcdefghijklmnop", "q"] },
    Case { name: "member_path", parts: &["a_very_long_symbol.attribute", "first"] },
];

const JOIN_CASES: [Case; 3] = [
    Case { name: "qualified_3", parts: &["package", "module", "name"] },
    Case { name: "qualified_8", parts: &["a", "b", "c", "d", "e", "f", "g", "h"] },
    Case { name: "long_components", parts: &["typing_extensions", "collections", "abc"] },
];

fn concat_builder(parts: &[&str]) -> CharStr {
    let capacity = parts.iter().map(|part| part.len()).sum();
    let mut value = CharString::with_capacity(capacity);
    for part in parts {
        value.push_str(part);
    }
    value.freeze()
}

fn join_builder(parts: &[&str], separator: &str) -> CharStr {
    let capacity = parts.iter().map(|part| part.len()).sum::<usize>()
        + separator.len() * parts.len().saturating_sub(1);
    let mut value = CharString::with_capacity(capacity);
    if let Some((first, rest)) = parts.split_first() {
        value.push_str(first);
        for part in rest {
            value.push_str(separator);
            value.push_str(part);
        }
    }
    value.freeze()
}

fn concat(c: &mut Criterion) {
    let mut group = c.benchmark_group("exact_construction/concat");

    for case in &CONCAT_CASES {
        let bytes = case.parts.iter().map(|part| part.len()).sum::<usize>();
        group.throughput(Throughput::Bytes(bytes as u64));
        group.bench_function(BenchmarkId::new("char_str", case.name), |b| {
            b.iter(|| black_box(CharStr::concat(black_box(case.parts))))
        });
        group.bench_function(BenchmarkId::new("builder_freeze", case.name), |b| {
            b.iter(|| black_box(concat_builder(black_box(case.parts))))
        });
    }

    group.finish();
}

fn join(c: &mut Criterion) {
    let mut group = c.benchmark_group("exact_construction/join");
    let separator = ".";

    for case in &JOIN_CASES {
        let bytes = case.parts.iter().map(|part| part.len()).sum::<usize>()
            + separator.len() * case.parts.len().saturating_sub(1);
        group.throughput(Throughput::Bytes(bytes as u64));
        group.bench_function(BenchmarkId::new("char_str", case.name), |b| {
            b.iter(|| black_box(CharStr::join(black_box(case.parts), separator)))
        });
        group.bench_function(BenchmarkId::new("builder_freeze", case.name), |b| {
            b.iter(|| black_box(join_builder(black_box(case.parts), separator)))
        });
    }

    group.finish();
}

fn inline_freeze(c: &mut Criterion) {
    let mut group = c.benchmark_group("exact_construction/inline_freeze");
    let full = "a".repeat(size_of::<CharStr>());

    for len in 0..=size_of::<CharStr>() {
        group.throughput(Throughput::Bytes(len as u64));
        let text = &full[..len];

        group.bench_function(BenchmarkId::new("fresh", len), |b| {
            b.iter_batched(
                || CharString::from(text),
                |string| black_box(string.freeze()),
                BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("truncated", len), |b| {
            b.iter_batched(
                || {
                    let mut string = CharString::from(full.as_str());
                    string.truncate(len);
                    string
                },
                |string| black_box(string.freeze()),
                BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("cleared", len), |b| {
            b.iter_batched(
                || {
                    let mut string = CharString::from(full.as_str());
                    string.clear();
                    string.push_str(text);
                    string
                },
                |string| black_box(string.freeze()),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(2))
        .sample_size(50);
    targets = concat, join, inline_freeze
}
criterion_main!(benches);
