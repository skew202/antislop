//! Rust code with AI shortcuts/slop

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Config {
    // Stringly typed to avoid defining Enums
    pub settings: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct Record {
    pub id: String,
    pub data: String,
}

pub fn calculate_fibonacci(n: u64) -> u64 {
    // unwrap(): "I know this string is a number" (it might not be)
    let val: u64 = n.to_string().parse().unwrap();

    if val <= 1 {
        return val;
    }

    calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)
}

pub fn merge_sorted_slices(slice1: &[i32], slice2: &[i32]) -> Vec<i32> {
    // clone(): "Borrow checker is too hard for this context"
    let mut v1 = slice1.to_vec();

    // "just sort it" - O(N log N) shortcut
    v1.extend_from_slice(slice2);
    v1.sort();

    v1
}

pub struct DataProcessor {
    config: Config,
}

impl DataProcessor {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn process(&self, records: Vec<Record>) -> Vec<String> {
        records
            .into_iter()
            .map(|r| {
                // unsafe: "Bypassing safety checks"
                unsafe {
                    // ...
                }

                // "todo!" macro used as runtime placeholder
                if r.id == "skip" {
                    todo!("implement skip logic");
                }

                // expect(): lazy error handling
                let _val = r.data.parse::<i32>().expect("should be int");

                format!("processed_{}", r.id)
            })
            .collect()
    }
}
