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

pub fn count_percentages(chars: &HashMap<char, usize>) -> HashMap<char, f64> {
    let total: usize = chars.values().sum();

    if total == 0 {
        return HashMap::new();
    }

    chars.iter()
        .map(|(&ch, &count)| (ch, (count as f64 / total as f64) * 100.0))
        .collect()
}