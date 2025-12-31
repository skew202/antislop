//! Benchmark suite for the scanner.

use antislop::config::{Config, Pattern, PatternCategory, Severity};
use antislop::Scanner;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_patterns() -> Vec<Pattern> {
    vec![
        Pattern {
            regex: "(?i)TODO:".to_string(),
            severity: Severity::Medium,
            message: "TODO".to_string(),
            category: PatternCategory::Placeholder,
        },
        Pattern {
            regex: "(?i)for now".to_string(),
            severity: Severity::Low,
            message: "deferral".to_string(),
            category: PatternCategory::Deferral,
        },
        Pattern {
            regex: "(?i)hopefully".to_string(),
            severity: Severity::Low,
            message: "hedging".to_string(),
            category: PatternCategory::Hedging,
        },
    ]
}

fn bench_python_scan(c: &mut Criterion) {
    let scanner = Scanner::new(bench_patterns()).unwrap();

    let clean_code = r#"
def process_data(items):
    result = []
    for item in items:
        if item.is_valid():
            result.append(item.transform())
    return result
"#;

    let sloppy_code = r#"
def process_data(items):
    # TODO: implement proper validation
    # for now we just filter
    result = []
    for item in items:
        # hopefully this works
        if item.is_valid():
            result.append(item.transform())
    return result
"#;

    let mut group = c.benchmark_group("scan/python");

    group.bench_function("clean", |b| {
        b.iter(|| {
            scanner.scan_file(
                black_box("test.py"),
                black_box(clean_code),
            )
        })
    });

    group.bench_function("sloppy", |b| {
        b.iter(|| {
            scanner.scan_file(
                black_box("test.py"),
                black_box(sloppy_code),
            )
        })
    });

    group.finish();
}

fn bench_file_size(c: &mut Criterion) {
    let scanner = Scanner::new(bench_patterns()).unwrap();

    let mut group = c.benchmark_group("scan/file_size");

    for size in [100, 1000, 10000].iter() {
        let code: String = (0..*size)
            .map(|i| format!("# Line {}: TODO: fix this\npass\n", i))
            .collect();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                scanner.scan_file(
                    black_box("test.py"),
                    black_box(&code),
                )
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_python_scan, bench_file_size);
criterion_main!(benches);
