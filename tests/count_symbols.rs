use symfreq::count_symbols;

#[test]
fn count_basic_symbols() {
    let result = count_symbols("a + b * (c - d)");
    assert_eq!(*result.get(&'+').unwrap(), 1);
    assert_eq!(*result.get(&'(').unwrap(), 1);
    assert_eq!(*result.get(&')').unwrap(), 1);
}