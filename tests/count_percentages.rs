use std::collections::HashMap;
use symfreq::count_percentages;

#[test]
fn count_basic_percentages() {
    let mut chars = HashMap::new();
    chars.insert('(', 2);
    chars.insert(')', 2);
    chars.insert('+', 1);

    let result = count_percentages(&chars);

    assert!((result[&'('] - 40.0).abs() < 0.01);
    assert!((result[&')'] - 40.0).abs() < 0.01);
    assert!((result[&'+'] - 20.0).abs() < 0.01);
}

#[test]
fn count_empty() {
    let chars = HashMap::new();
    let result = count_percentages(&chars);

    assert!(result.is_empty());
}
