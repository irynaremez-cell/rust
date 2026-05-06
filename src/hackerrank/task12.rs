use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let max_height = *candles.iter().max().unwrap();

    candles
        .iter()
        .filter(|&&candle| candle == max_height)
        .count() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _candles_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let candles: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = birthday_cake_candles(&candles);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birthday_cake_candles_example() {
        let candles = vec![3, 2, 1, 3];

        assert_eq!(birthday_cake_candles(&candles), 2);
    }

    #[test]
    fn test_birthday_cake_candles_all_same() {
        let candles = vec![4, 4, 4, 4];

        assert_eq!(birthday_cake_candles(&candles), 4);
    }
}
