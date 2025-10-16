use ignore::Walk;
use std::collections::HashMap;
use std::path::PathBuf;

const SYMBOLS: &str = "(){}[]<>;:.,'\"!@#%^&*-=+_`~|\\/?$";
pub const DEFAULT_EXTENSIONS: &[&str] = &["rs", "js", "jsx", "ts", "tsx"];

pub fn count_symbols(input: &str) -> HashMap<char, usize> {
    input
        .chars()
        .filter(|c| SYMBOLS.contains(*c))
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

    chars
        .iter()
        .map(|(&ch, &count)| (ch, (count as f64 / total as f64) * 100.0))
        .collect()
}

pub fn sorted_percentages(percentages: &HashMap<char, f64>) -> Vec<(char, f64)> {
    let mut values = percentages
        .iter()
        .map(|(&char, &percentage)| (char, percentage))
        .collect::<Vec<_>>();

    values.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    values
}

pub struct ReadResult {
    pub content: String,
    pub files_read: usize,
    pub files_skipped: usize,
    pub files_failed: usize,
}

pub fn read_path<P: AsRef<std::path::Path>>(
    path: P,
    exts: &std::collections::HashSet<&str>,
) -> Result<ReadResult, std::io::Error> {
    let mut collected = String::new();
    let mut files_read = 0;
    let mut files_skipped = 0;
    let mut files_failed = 0;
    for result in Walk::new(path) {
        match result {
            Ok(entry) => {
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
                        if !exts.contains(&ext) {
                            files_skipped += 1;
                            continue;
                        }

                        if let Ok(content) = std::fs::read_to_string(entry.path()) {
                            collected.push_str(&content);
                            files_read += 1;
                        } else {
                            files_failed += 1;
                        } 
                    }
                }
            }
            Err(err) => println!("ERROR: {}", err),
        }
    }

    Ok(ReadResult {
        content: collected,
        files_read,
        files_skipped,
        files_failed,
    })
}
