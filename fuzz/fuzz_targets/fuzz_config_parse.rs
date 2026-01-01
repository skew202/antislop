#![no_main]

use antislop::config::Config;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Convert bytes to string (TOML must be valid UTF-8)
    let toml_content = match std::str::from_utf8(data) {
        Ok(s) => s,
        Err(_) => return,
    };

    // Attempt to parse as TOML config
    // This should never panic, just return Err for invalid configs
    let _ = Config::from_toml_str(toml_content);
});
