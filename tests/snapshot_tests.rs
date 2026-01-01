use antislop::{config::Config, Scanner};
use insta::assert_json_snapshot;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_json_output_snapshot() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.py");
    fs::write(
        &file_path,
        "def foo():\n    # TODO: implement this\n    pass",
    )
    .unwrap();

    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");
    let content = fs::read_to_string(&file_path).unwrap();
    let results = scanner.scan_file(file_path.to_str().unwrap(), &content);

    // Filter out absolute paths to make snapshot stable
    let mut stable_result = results.clone();
    stable_result.path = "test.py".to_string();
    for finding in &mut stable_result.findings {
        finding.file = "test.py".to_string();
    }

    assert_json_snapshot!(stable_result);
}

#[test]
fn test_multiple_findings_snapshot() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    // Creative sloppy code with multiple issue types
    let sloppy_code = r#"
def process_data(data):
    # TODO: implement validation with json schema
    # FIXME: handle edge cases where data is None
    
    try:
        # for now just return the data if it looks ok
        if len(data) > 0:
            return data
    except:
        # hopefully this works and doesn't crash
        pass
        
    return None
"#;

    let results = scanner.scan_file("test.py", sloppy_code);

    // Stabilize results for snapshot
    let mut stable_result = results.clone();
    stable_result.path = "test.py".to_string();
    for finding in &mut stable_result.findings {
        finding.file = "test.py".to_string();
    }

    assert_json_snapshot!("multiple_findings", stable_result);
}

#[test]
fn test_clean_code_snapshot() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    // Clean code with no slop
    let clean_code = r#"
def add(a: int, b: int) -> int:
    """Add two integers and return the result."""
    return a + b

def multiply(x: float, y: float) -> float:
    """Multiply two floats and return the result."""
    return x * y
"#;

    let results = scanner.scan_file("clean.py", clean_code);

    // Stabilize results for snapshot
    let mut stable_result = results.clone();
    stable_result.path = "clean.py".to_string();

    assert_json_snapshot!("clean_code", stable_result);
}
