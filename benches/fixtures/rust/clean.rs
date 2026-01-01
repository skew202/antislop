//! Clean Rust code without slop

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Config {
    pub required_fields: Vec<String>,
    pub max_batch_size: usize,
}

#[derive(Debug, Clone)]
pub struct Record {
    pub id: String,
    pub name: Option<String>,
    pub data: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ProcessedRecord {
    pub id: String,
    pub name: String,
    pub processed: bool,
}

pub fn calculate_fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

pub fn merge_sorted_slices<T: Ord + Clone>(slice1: &[T], slice2: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(slice1.len() + slice2.len());
    let mut i = 0;
    let mut j = 0;

    while i < slice1.len() && j < slice2.len() {
        if slice1[i] <= slice2[j] {
            result.push(slice1[i].clone());
            i += 1;
        } else {
            result.push(slice2[j].clone());
            j += 1;
        }
    }

    result.extend_from_slice(&slice1[i..]);
    result.extend_from_slice(&slice2[j..]);
    result
}

pub struct DataProcessor {
    config: Config,
    processed_count: usize,
}

impl DataProcessor {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            processed_count: 0,
        }
    }

    pub fn process(&mut self, records: &[Record]) -> Vec<ProcessedRecord> {
        records
            .iter()
            .filter(|r| self.is_valid(r))
            .map(|r| {
                self.processed_count += 1;
                self.transform(r)
            })
            .collect()
    }

    pub fn processed_count(&self) -> usize {
        self.processed_count
    }

    fn is_valid(&self, record: &Record) -> bool {
        self.config
            .required_fields
            .iter()
            .all(|field| record.data.contains_key(field))
    }

    fn transform(&self, record: &Record) -> ProcessedRecord {
        ProcessedRecord {
            id: record.id.clone(),
            name: record.name.clone().unwrap_or_default().to_uppercase(),
            processed: true,
        }
    }
}
