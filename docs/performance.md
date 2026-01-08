# Performance

AntiSlop uses tree-sitter AST parsing for accurate detection. Regex-only mode is ~10x faster.

| Language | Mode | Time | Throughput |
|:---------|:-----|:-----|:-----------|
| **Python** | AST | **3.89 ms** | 392 KiB/s |
| **JavaScript** | AST | **1.72 ms** | 776 KiB/s |
| **TypeScript** | AST | **4.96 ms** | 378 KiB/s |
| **Go** | AST | **1.28 ms** | 1.25 MiB/s |
| **Rust** | AST | **3.33 ms** | 606 KiB/s |
| **Python** | Regex | **385 µs** | — |
| **Rust** | Regex | **384 µs** | — |

**Scaling:**
| Lines | Time | throughput |
|:------|:-----|:-----------|
| 10,000 | **75.6 ms** | 1.9 MiB/s |
| 50,000 | **382 ms** | 1.9 MiB/s |

*Benchmarks run on standard laptop hardware (Linux x86_64). Run `cargo bench` to reproduce.*
