//! Comprehensive benchmark suite for the scanner.
//!
//! Benchmarks cover:
//! - Multiple languages (Python, JavaScript, TypeScript, Go, Rust)
//! - Clean vs sloppy code comparisons
//! - Scaling from 100 to 100,000 lines
//! - Tree-sitter vs regex mode comparison

use antislop::config::Config;
use antislop::Scanner;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

// Embedded fixture files
const PYTHON_CLEAN: &str = include_str!("fixtures/python/clean.py");
const PYTHON_SLOPPY: &str = include_str!("fixtures/python/sloppy.py");
const JS_CLEAN: &str = include_str!("fixtures/javascript/clean.js");
const JS_SLOPPY: &str = include_str!("fixtures/javascript/sloppy.js");
const TS_CLEAN: &str = include_str!("fixtures/typescript/clean.ts");
const TS_SLOPPY: &str = include_str!("fixtures/typescript/sloppy.ts");
const GO_CLEAN: &str = include_str!("fixtures/go/clean.go");
const GO_SLOPPY: &str = include_str!("fixtures/go/sloppy.go");
const RS_CLEAN: &str = include_str!("fixtures/rust/clean.rs");
const RS_SLOPPY: &str = include_str!("fixtures/rust/sloppy.rs");

fn get_scanner() -> Scanner {
    let config = Config::default();
    Scanner::new(config.patterns).expect("Failed to create scanner")
}

/// Benchmark Python scanning
fn bench_python(c: &mut Criterion) {
    let scanner = get_scanner();
    let mut group = c.benchmark_group("scan/python");

    group.throughput(Throughput::Bytes(PYTHON_CLEAN.len() as u64));
    group.bench_function("clean", |b| {
        b.iter(|| scanner.scan_file(black_box("test.py"), black_box(PYTHON_CLEAN)))
    });

    group.throughput(Throughput::Bytes(PYTHON_SLOPPY.len() as u64));
    group.bench_function("sloppy", |b| {
        b.iter(|| scanner.scan_file(black_box("test.py"), black_box(PYTHON_SLOPPY)))
    });

    group.finish();
}

/// Benchmark JavaScript scanning
fn bench_javascript(c: &mut Criterion) {
    let scanner = get_scanner();
    let mut group = c.benchmark_group("scan/javascript");

    group.throughput(Throughput::Bytes(JS_CLEAN.len() as u64));
    group.bench_function("clean", |b| {
        b.iter(|| scanner.scan_file(black_box("test.js"), black_box(JS_CLEAN)))
    });

    group.throughput(Throughput::Bytes(JS_SLOPPY.len() as u64));
    group.bench_function("sloppy", |b| {
        b.iter(|| scanner.scan_file(black_box("test.js"), black_box(JS_SLOPPY)))
    });

    group.finish();
}

/// Benchmark TypeScript scanning
fn bench_typescript(c: &mut Criterion) {
    let scanner = get_scanner();
    let mut group = c.benchmark_group("scan/typescript");

    group.throughput(Throughput::Bytes(TS_CLEAN.len() as u64));
    group.bench_function("clean", |b| {
        b.iter(|| scanner.scan_file(black_box("test.ts"), black_box(TS_CLEAN)))
    });

    group.throughput(Throughput::Bytes(TS_SLOPPY.len() as u64));
    group.bench_function("sloppy", |b| {
        b.iter(|| scanner.scan_file(black_box("test.ts"), black_box(TS_SLOPPY)))
    });

    group.finish();
}

/// Benchmark Go scanning
fn bench_go(c: &mut Criterion) {
    let scanner = get_scanner();
    let mut group = c.benchmark_group("scan/go");

    group.throughput(Throughput::Bytes(GO_CLEAN.len() as u64));
    group.bench_function("clean", |b| {
        b.iter(|| scanner.scan_file(black_box("test.go"), black_box(GO_CLEAN)))
    });

    group.throughput(Throughput::Bytes(GO_SLOPPY.len() as u64));
    group.bench_function("sloppy", |b| {
        b.iter(|| scanner.scan_file(black_box("test.go"), black_box(GO_SLOPPY)))
    });

    group.finish();
}

/// Benchmark Rust scanning
fn bench_rust(c: &mut Criterion) {
    let scanner = get_scanner();
    let mut group = c.benchmark_group("scan/rust");

    group.throughput(Throughput::Bytes(RS_CLEAN.len() as u64));
    group.bench_function("clean", |b| {
        b.iter(|| scanner.scan_file(black_box("test.rs"), black_box(RS_CLEAN)))
    });

    group.throughput(Throughput::Bytes(RS_SLOPPY.len() as u64));
    group.bench_function("sloppy", |b| {
        b.iter(|| scanner.scan_file(black_box("test.rs"), black_box(RS_SLOPPY)))
    });

    group.finish();
}

/// Benchmark scaling across file sizes
fn bench_scaling(c: &mut Criterion) {
    let scanner = get_scanner();
    let mut group = c.benchmark_group("scan/scaling");

    // Test various file sizes
    for size in [100, 1_000, 10_000, 50_000].iter() {
        // Generate sloppy Python code of given size
        let code: String = (0..*size)
            .map(|i| {
                if i % 10 == 0 {
                    format!("# TODO: fix line {}\n", i)
                } else if i % 15 == 0 {
                    format!("# for now just skip line {}\n", i)
                } else if i % 20 == 0 {
                    format!("# hopefully this works for line {}\n", i)
                } else {
                    format!("x_{} = {}\n", i, i)
                }
            })
            .collect();

        group.throughput(Throughput::Bytes(code.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_lines", size)),
            size,
            |b, _| b.iter(|| scanner.scan_file(black_box("test.py"), black_box(&code))),
        );
    }

    group.finish();
}

/// Benchmark regex fallback mode (using .txt extension to force regex)
fn bench_regex_fallback(c: &mut Criterion) {
    let scanner = get_scanner();
    let mut group = c.benchmark_group("scan/regex_fallback");

    // Use .txt extension to force regex fallback
    group.bench_function("python_code_as_txt", |b| {
        b.iter(|| scanner.scan_file(black_box("test.txt"), black_box(PYTHON_SLOPPY)))
    });

    group.bench_function("js_code_as_txt", |b| {
        b.iter(|| scanner.scan_file(black_box("test.txt"), black_box(JS_SLOPPY)))
    });

    group.finish();
}

/// Compare tree-sitter vs regex for same content
fn bench_treesitter_vs_regex(c: &mut Criterion) {
    let scanner = get_scanner();
    let mut group = c.benchmark_group("scan/mode_comparison");

    // Same content, different extensions
    group.bench_function("python_treesitter", |b| {
        b.iter(|| scanner.scan_file(black_box("test.py"), black_box(PYTHON_SLOPPY)))
    });

    group.bench_function("python_regex", |b| {
        b.iter(|| scanner.scan_file(black_box("test.txt"), black_box(PYTHON_SLOPPY)))
    });

    group.bench_function("rust_treesitter", |b| {
        b.iter(|| scanner.scan_file(black_box("test.rs"), black_box(RS_SLOPPY)))
    });

    group.bench_function("rust_regex", |b| {
        b.iter(|| scanner.scan_file(black_box("test.txt"), black_box(RS_SLOPPY)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_python,
    bench_javascript,
    bench_typescript,
    bench_go,
    bench_rust,
    bench_scaling,
    bench_regex_fallback,
    bench_treesitter_vs_regex,
);
criterion_main!(benches);
