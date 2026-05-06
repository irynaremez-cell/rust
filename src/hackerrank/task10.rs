use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    let mut counts = HashMap::new();

    for &color in ar {
        *counts.entry(color).or_insert(0) += 1;
    }

    counts.values().map(|count| count / 2).sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let ar: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = sock_merchant(n, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant_example() {
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];

        assert_eq!(sock_merchant(9, &ar), 3);
    }

    #[test]
    fn test_sock_merchant_second_case() {
        let ar = vec![1, 2, 1, 2, 1, 3, 2];

        assert_eq!(sock_merchant(7, &ar), 2);
    }
}
