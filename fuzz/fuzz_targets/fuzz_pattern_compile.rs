#![no_main]

use antislop::config::RegexPattern;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Convert bytes to string
    let pattern = match std::str::from_utf8(data) {
        Ok(s) => s,
        Err(_) => return,
    };

    // Attempt to compile as regex pattern
    // This should never panic, just return Err for invalid patterns
    let _ = RegexPattern::new(pattern.to_string());
});
