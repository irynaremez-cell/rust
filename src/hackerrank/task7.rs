use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let start = *a.iter().max().unwrap();
    let end = *b.iter().min().unwrap();
    let mut count = 0;

    for number in start..=end {
        let first_condition = a.iter().all(|value| number % value == 0);
        let second_condition = b.iter().all(|value| value % number == 0);

        if first_condition && second_condition {
            count += 1;
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let _n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let _m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let total = get_total_x(&arr, &brr);

    writeln!(&mut fptr, "{}", total).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x_example() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];

        assert_eq!(get_total_x(&a, &b), 3);
    }

    #[test]
    fn test_get_total_x_second_example() {
        let a = vec![3, 4];
        let b = vec![24, 48];

        assert_eq!(get_total_x(&a, &b), 2);
    }
}