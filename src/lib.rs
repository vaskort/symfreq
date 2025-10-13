use std::collections::HashMap;

const SYMBOLS: &str = "(){}[]<>;:.,'\"!@#%^&*-=+_`~|\\/?$";

pub fn count_symbols(input: &str) -> HashMap<char, usize> {
    input.chars().
        filter(|c| SYMBOLS.contains(*c))
        .fold(HashMap::new(), |mut acc, ch| {
            *acc.entry(ch).or_insert(0) += 1;
            acc
        })
}
