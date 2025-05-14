use snorkel::commands::{cycles::handle_cycles, ema::handle_ema, zscore::handle_zscore};
use std::fs::File;
use std::io::{BufWriter, Write};
use tempfile::NamedTempFile;

#[test]
fn test_detect_cycles_true() {
    let edges = [("A", "B"), ("B", "C"), ("C", "A")];
    let mut file = NamedTempFile::new().unwrap();
    for (a, b) in &edges {
        writeln!(file, "{a},{b}").unwrap();
    }
    let results = handle_cycles(Some(file.path().to_str().unwrap().to_string()), true);
    assert!(result.is_ok());
}
#[test]
fn test_zscore_runs() {
    let values = ["10.0", "10.1", "10.2", "15.0", "10.3"];
    let mut file = NamedTempFile::new().unwrap();
    for v in &values {
        writeln!(file, "{v}").unwrap();
    }
    let result = handle_zscore(
        2.0,
        3,
        Some(file.path().to_str().unwrap().to_string()),
        None,
        false,
    );
    assert!(result.is_ok());
}
#[test]
fn test_ema_runs() {
    let values = ["1.0", "2.0", "3.0", "4.0"];
    let mut file = NamedTempFile::new().unwrap();
    for v in &values {
        writeln!(file, "{v}").unwrap();
    }
    let result = handle_ema(
        None,
        Some(file.path().to_str().unwrap().to_string()),
        None,
        false,
    );
    assert!(result.is_ok());
}
