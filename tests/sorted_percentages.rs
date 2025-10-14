use std::collections::HashMap;
use symfreq::sorted_percentages;

#[test]
fn simple_sorted_percentages() {
    let mut chars = HashMap::new();

    chars.insert('(', 20.0);
    chars.insert(')', 80.0);

    let result = sorted_percentages(&chars);
    assert_eq!(result, vec![(')', 80.0), ('(', 20.0)]);
    assert_eq!(result.len(), 2);
}