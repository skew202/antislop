#![no_main]

use antislop::{config::Config, Scanner};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Convert bytes to string, skip if not valid UTF-8
    let source = match std::str::from_utf8(data) {
        Ok(s) => s,
        Err(_) => return,
    };

    // Create scanner with default patterns
    let config = Config::default();
    let scanner = match Scanner::new(config.patterns) {
        Ok(s) => s,
        Err(_) => return,
    };

    // This should never panic
    let _ = scanner.scan_file("fuzz_input.py", source);
    let _ = scanner.scan_file("fuzz_input.rs", source);
    let _ = scanner.scan_file("fuzz_input.js", source);
    let _ = scanner.scan_file("fuzz_input.txt", source);
});
