use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary = 0;
    let mut secondary = 0;

    for i in 0..n {
        primary += arr[i][i];
        secondary += arr[i][n - 1 - i];
    }

    (primary - secondary).abs()
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

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonal_difference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference_example() {
        let arr = vec![
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12],
        ];

        assert_eq!(diagonal_difference(&arr), 15);
    }

    #[test]
    fn test_diagonal_difference_zero() {
        let arr = vec![
            vec![1, 2],
            vec![3, 4],
        ];

        assert_eq!(diagonal_difference(&arr), 0);
    }
}
